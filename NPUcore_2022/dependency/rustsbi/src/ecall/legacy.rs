use super::SbiRet;
use crate::hart_mask::HartMask;
use crate::ipi::send_ipi_many;
use crate::legacy_stdio::{legacy_stdio_getchar, legacy_stdio_putchar};
use riscv::register::{mie, mip};

#[inline]
pub fn console_putchar(param0: usize) -> SbiRet {
    let ch = (param0 & 0xff) as u8;
    legacy_stdio_putchar(ch);
    SbiRet::ok(0) // the return value 0 is ignored in legacy
}

// Returns a character, or -1 when failed.
#[inline]
pub fn console_getchar() -> SbiRet {
    let ch = legacy_stdio_getchar();
    SbiRet::legacy_ok(ch)
}

#[inline]
pub fn send_ipi(hart_mask_addr: usize) -> SbiRet {
    // note(unsafe): if any load fault, should be handled by machine level;
    // see docs at `legacy_from_addr`.
    let hart_mask = unsafe { HartMask::legacy_from_addr(hart_mask_addr) };
    send_ipi_many(hart_mask)
}

#[inline]
#[cfg(target_pointer_width = "64")]
pub fn set_timer_64(time_value: usize) -> SbiRet {
    crate::timer::set_timer(time_value as u64);

    let mtip = mip::read().mtimer();
    if mtip {
        unsafe {
            mie::clear_mtimer();
            mip::set_stimer();
        }
    } else {
        unsafe {
            mie::set_mtimer();
            mip::clear_stimer();
        }
    }
    SbiRet::ok(0)
}

#[inline]
#[cfg(target_pointer_width = "32")]
pub fn set_timer_32(arg0: usize, arg1: usize) -> SbiRet {
    let time_value = (arg0 as u64) + ((arg1 as u64) << 32);
    crate::timer::set_timer(time_value as u64);

    let mtip = mip::read().mtimer();
    if mtip {
        unsafe {
            mie::clear_mtimer();
            mip::set_stimer();
        }
    } else {
        unsafe {
            mie::set_mtimer();
            mip::clear_stimer();
        }
    }
    SbiRet::ok(0)
}

#[inline]
pub fn shutdown() -> SbiRet {
    crate::reset::legacy_reset()
}
