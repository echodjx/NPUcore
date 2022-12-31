#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(btree_drain_filter)]
#![feature(drain_filter)]
#![feature(int_roundings)]
#![feature(string_remove_matches)]

extern crate alloc;

#[macro_use]
extern crate bitflags;

#[cfg(feature = "board_k210")]
#[path = "boards/k210.rs"]
mod board;
#[cfg(feature = "board_fu740")]
#[path = "boards/fu740.rs"]
mod board;
#[cfg(all(not(feature = "board_k210"), not(feature = "board_fu740")))]
#[path = "boards/qemu.rs"]
mod board;

#[macro_use]
mod console;
mod config;
mod drivers;
mod fs;
mod lang_items;
mod mm;
mod sbi;
mod syscall;
mod task;
mod timer;
mod trap;

core::arch::global_asm!(include_str!("entry.asm"));

fn mem_clear() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    #[cfg(feature = "zero_init")]
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, crate::config::MEMORY_END - sbss as usize)
            .fill(0);
    }
    #[cfg(not(feature = "zero_init"))]
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

#[no_mangle]
pub fn rust_main() -> ! {
    mem_clear();
    console::log_init();
    println!("[kernel] Hello, world!");
    mm::init();
    mm::remap_test();
    #[cfg(feature = "board_fu740")]
    board::clock_init();
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    fs::directory_tree::init_fs();
    task::add_initproc();
    task::run_tasks();
    panic!("Unreachable in rust_main!");
}
