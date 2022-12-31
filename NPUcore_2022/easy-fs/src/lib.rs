#![no_std]
#![feature(string_remove_matches)]
#![feature(int_roundings)]

extern crate alloc;

mod bitmap;
pub mod block_cache;
mod block_dev;
mod dir_iter;
mod efs;
pub mod layout;
mod vfs;

pub const BLOCK_SZ: usize = 512;
pub const CACHE_SZ: usize = 8 * BLOCK_SZ;
use bitmap::Fat;
//pub use block_cache::get_block_cache;
pub use block_cache::{Cache, CacheManager};
pub use block_dev::BlockDevice;
pub use efs::EasyFileSystem;
pub use layout::DiskInodeType;
pub use vfs::Inode;
