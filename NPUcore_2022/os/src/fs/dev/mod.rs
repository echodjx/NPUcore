pub mod null;
pub mod pipe;
pub mod tty;
pub mod zero;
pub mod socket;
pub mod hwclock;

#[macro_export]
macro_rules! makedev {
    ($x:literal, $y:literal) => {
        (($x & 0xfffff000) << 32)
            | (($x & 0x00000fff) << 8)
            | (($y & 0xffffff00) << 12)
            | ($y & 0x000000ff)
    };
}
