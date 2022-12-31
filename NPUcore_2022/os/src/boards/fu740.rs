use crate::config::DISK_IMAGE_BASE;
use fu740_hal::{clock::PrciExt, time::U32Ext};
use fu740_pac::Peripherals;

/// should be 1MHz.
/// # Reference
/// https://www.starfivetech.com/uploads/fu740-c000-manual-v1p2.pdf, page 76. \
/// "The CPU real time clock (rtcclk) runs at **1 MHz** and is driven from input pin RTCCLKIN. This
/// should be connected to an external oscillator."
pub const CLOCK_FREQ: usize = 1_000_000;

pub const MMIO: &[(usize, usize)] = &[
    (0x1000_0000, 0x1000),         // PRCI
    (DISK_IMAGE_BASE, 0x800_0000), // disk image
];

pub type BlockDeviceImpl = crate::drivers::block::MemBlockWrapper;

pub fn clock_init() {
    let prci = unsafe { Peripherals::steal().PRCI };
    let clock = prci.setup().coreclk(1_500u32.mhz()).freeze();
    crate::println!("coreclk frequency: {:?}", clock.coreclk().0)
}
