use alloc::sync::Arc;
use crate::fs::DiskInodeType;

use crate::{
    fs::{directory_tree::DirectoryTreeNode, file_trait::File, layout::Stat, StatMode},
    mm::UserBuffer,
    syscall::errno::{ENOTDIR, ESPIPE},
};

/// Data Sink
/// Data written to the `/dev/null` special files is discarded.
/// Reads  from `/dev/null` always return end of file (i.e., read(2) returns 0)
pub struct Null;
#[allow(unused)]
impl File for Null {
    fn deep_clone(&self) -> Arc<dyn File> {
        Arc::new(Null {})
    }

    fn readable(&self) -> bool {
        true
    }

    fn writable(&self) -> bool {
        true
    }

    fn read(&self, offset: Option<&mut usize>, buf: &mut [u8]) -> usize {
        unreachable!()
    }

    fn write(&self, offset: Option<&mut usize>, buf: &[u8]) -> usize {
        unreachable!()
    }

    fn r_ready(&self) -> bool {
        true
    }

    fn w_ready(&self) -> bool {
        true
    }

    fn get_size(&self) -> usize {
        todo!()
    }

    fn get_stat(&self) -> Stat {
        Stat::new(
            crate::makedev!(0, 5),
            1,
            StatMode::S_IFCHR.bits() | 0o666,
            1,
            crate::makedev!(1, 3),
            0,
            0,
            0,
            0,
        )
    }

    fn read_user(&self, offset: Option<usize>, buf: UserBuffer) -> usize {
        0
    }

    fn write_user(&self, offset: Option<usize>, buf: UserBuffer) -> usize {
        buf.len()
    }
    
    fn get_file_type(&self) -> DiskInodeType {
        DiskInodeType::File
    }

    fn info_dirtree_node(
        &self,
        dirnode_ptr: alloc::sync::Weak<crate::fs::directory_tree::DirectoryTreeNode>,
    ) {
    }

    fn get_dirtree_node(&self) -> Option<Arc<DirectoryTreeNode>> {
        todo!()
    }

    fn open(&self, flags: crate::fs::layout::OpenFlags, special_use: bool) -> Arc<dyn File> {
        Arc::new(Null {})
    }

    fn open_subfile(&self) -> Result<alloc::vec::Vec<(alloc::string::String, alloc::sync::Arc<dyn File>)>, isize> {
        Err(ENOTDIR)
    }

    fn create(&self, name: &str, file_type: DiskInodeType) -> Result<Arc<dyn File>, isize> {
        todo!()
    }

    fn link_child(&self, name: &str, child: &Self) -> Result<(), isize>
    where
        Self: Sized,
    {
        todo!()
    }

    fn unlink(&self, delete: bool) -> Result<(), isize> {
        todo!()
    }

    fn get_dirent(&self, count: usize) -> alloc::vec::Vec<crate::fs::layout::Dirent> {
        todo!()
    }

    fn lseek(&self, offset: isize, whence: crate::fs::SeekWhence) -> Result<usize, isize> {
        Err(ESPIPE)
    }

    fn modify_size(&self, diff: isize) -> Result<(), isize> {
        todo!()
    }

    fn truncate_size(&self, new_size: usize) -> Result<(), isize> {
        todo!()
    }

    fn set_timestamp(&self, ctime: Option<usize>, atime: Option<usize>, mtime: Option<usize>) {
        todo!()
    }

    fn get_single_cache(
        &self,
        offset: usize,
    ) -> Result<Arc<spin::Mutex<crate::fs::PageCache>>, ()> {
        todo!()
    }

    fn get_all_caches(
        &self,
    ) -> Result<alloc::vec::Vec<Arc<spin::Mutex<crate::fs::PageCache>>>, ()> {
        todo!()
    }

    fn oom(&self) -> usize {
        0
    }

    fn hang_up(&self) -> bool {
        todo!()
    }

    fn fcntl(&self, cmd: u32, arg: u32) -> isize {
        todo!()
    }
}
