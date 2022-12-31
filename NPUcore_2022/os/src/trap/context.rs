use riscv::register::sstatus::{self, set_spp, Sstatus, SPP};

use crate::task::{Signals, SignalStack};

/// General registers
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct GeneralRegs {
    pub pc: usize,
    pub ra: usize,
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0: usize,
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
}

/// FP registers
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FloatRegs {
    pub f: [usize; 32],
    pub fcsr: u32,
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct MachineContext {
    gp: GeneralRegs,
    fp: FloatRegs,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UserContext
{
	pub flags: usize,
	pub link: usize,
	pub stack: SignalStack,
	pub sigmask: Signals,
    pub __pad: [u8; 128],
    pub mcontext: MachineContext,
}

impl UserContext {
    pub const PADDING_SIZE: usize = 128;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// The trap cotext containing the user context and the supervisor level
pub struct TrapContext {
    /// The registers to be preserved.
    pub gp: GeneralRegs,
    pub fp: FloatRegs,
    /// A copy of register a0, useful when we need to restart syscall
    pub origin_a0: usize,
    /// Privilege level of the trap context
    pub sstatus: Sstatus,
    /// Supervisor Address Translation and Protection
    pub kernel_satp: usize,
    /// The pointer to trap_handler
    pub trap_handler: usize,
    /// The current sp to be recovered on next entry into kernel space.
    pub kernel_sp: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.gp.sp = sp;
    }
    pub fn app_init_context(
        entry: usize,
        sp: usize,
        kernel_satp: usize,
        kernel_sp: usize,
        trap_handler: usize,
    ) -> Self {
        let mut sstatus = sstatus::read();
        // set CPU privilege to User after trapping back
        unsafe {
            set_spp(SPP::User);
        }
        let mut cx = Self {
            gp: GeneralRegs::default(),
            fp: FloatRegs::default(),
            origin_a0: 0,
            sstatus,
            kernel_satp,
            trap_handler,
            kernel_sp,
        };
        cx.gp.pc = entry;
        cx.set_sp(sp);
        cx
    }
}
