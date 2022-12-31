mod context;
use core::arch::{asm, global_asm};

use crate::config::TRAMPOLINE;
use crate::mm::{frame_reserve, MemoryError, VirtAddr};
use crate::syscall::syscall;
use crate::task::{
    current_task, current_trap_cx, do_signal, do_wake_expired, suspend_current_and_run_next,
    Signals,
};
use crate::timer::set_next_trigger;
pub use context::{MachineContext, TrapContext, UserContext};
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sepc, sie, stval, stvec,
};

global_asm!(include_str!("trap.S"));

extern "C" {
    pub fn __alltraps();
    pub fn __restore();
    pub fn __call_sigreturn();
}

pub fn init() {
    set_kernel_trap_entry();
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[no_mangle]
pub fn trap_handler() -> ! {
    set_kernel_trap_entry();
    {
        let task = current_task().unwrap();
        let mut inner = task.acquire_inner_lock();
        inner.update_process_times_enter_trap();
    }
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            // jump to next instruction anyway
            let mut cx = current_trap_cx();
            cx.gp.pc += 4;
            // get system call return value
            let result = syscall(
                cx.gp.a7,
                [cx.gp.a0, cx.gp.a1, cx.gp.a2, cx.gp.a3, cx.gp.a4, cx.gp.a5],
            );
            // cx is changed during sys_exec, so we have to call it again
            cx = current_trap_cx();
            cx.gp.a0 = result as usize;
        }
        Trap::Exception(Exception::StoreFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::InstructionFault)
        | Trap::Exception(Exception::InstructionPageFault)
        | Trap::Exception(Exception::LoadFault)
        | Trap::Exception(Exception::LoadPageFault) => {
            let task = current_task().unwrap();
            let mut inner = task.acquire_inner_lock();
            let addr = VirtAddr::from(stval);
            log::debug!(
                "[page_fault] pid: {}, type: {:?}",
                task.pid.0,
                scause.cause()
            );
            // This is where we handle the page fault.
            frame_reserve(3);
            if let Err(error) = task.vm.lock().do_page_fault(addr) {
                match error {
                    MemoryError::BeyondEOF => {
                        inner.add_signal(Signals::SIGBUS);
                    }
                    MemoryError::NoPermission | MemoryError::BadAddress => {
                        inner.add_signal(Signals::SIGSEGV);
                    }
                    _ => unreachable!()
                }
            };
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            let task = current_task().unwrap();
            let mut inner = task.acquire_inner_lock();
            inner.add_signal(Signals::SIGILL);
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            do_wake_expired();
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    {
        let task = current_task().unwrap();
        let mut inner = task.acquire_inner_lock();
        inner.update_process_times_leave_trap(scause.cause());
    }
    trap_return();
}

#[no_mangle]
pub fn trap_return() -> ! {
    do_signal();
    set_user_trap_entry();
    let task = current_task().unwrap();
    let trap_cx_ptr = task.trap_cx_user_va();
    let user_satp = task.get_user_token();
    drop(task);
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,
            in("a1") user_satp,
            options(noreturn)
        );
    }
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!(
        "a trap {:?} from kernel! bad addr = {:#x}, bad instruction = {:#x}",
        scause::read().cause(),
        stval::read(),
        match current_task() {
            Some(task) => {
                task.acquire_inner_lock().get_trap_cx().gp.pc
            }
            None => {
                sepc::read()
            }
        }
    );
}
