#![feature(test)]

extern crate alloc;
extern crate easy_fs;
extern crate test;

use alloc::collections::VecDeque;
use alloc::sync::Arc;
use easy_fs::layout::DiskInodeType;
use easy_fs::*;
use lazy_static::*;
use spin::{Mutex, RwLock};
//use std::ascii::AsciiExt;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

//use std::sync::Mutex;
const BLOCK_SZ: usize = 512;

struct BlockFile(Mutex<File>);

impl BlockDevice for BlockFile {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        assert_eq!(buf.len() % BLOCK_SZ, 0);
        let mut file = self.0.lock();
        file.seek(SeekFrom::Start((block_id * BLOCK_SZ) as u64))
            .expect("Error when seeking!");
        assert_eq!(file.read(buf).unwrap(), BLOCK_SZ, "Not a complete block!");
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) {
        assert_eq!(buf.len() % BLOCK_SZ, 0);

        let mut file = self.0.lock();
        file.seek(SeekFrom::Start((block_id * BLOCK_SZ) as u64))
            .expect("Error when seeking!");
        assert_eq!(file.write(buf).unwrap(), BLOCK_SZ, "Not a complete block!");
    }
}

pub struct BlockCache {
    cache: [u8; BLOCK_SZ],
    block_id: usize,
    block_device: Arc<dyn BlockDevice>,
    modified: bool,
}

impl Drop for BlockCache {
    fn drop(&mut self) {
        self.sync()
    }
}

impl BlockCache {
    /// Private function.
    /// Get the address at the `offset` in the cache to the cache for later access.
    /// # Argument
    /// * `offset`: The offset from the beginning of the block
    fn addr_of_offset(&self, offset: usize) -> usize {
        &self.cache[offset] as *const _ as usize
    }

    /// Get a reference to the block at required `offset`, casting the in the coming area as an instance of type `&T`
    /// # Argument
    /// * `offset`: The offset from the beginning of the block
    fn get_ref<T>(&self, offset: usize) -> &T
    where
        T: Sized,
    {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SZ);
        let addr = self.addr_of_offset(offset);
        unsafe { &*(addr as *const T) }
    }

    /// The mutable version of `get_ref()`
    fn get_mut<T>(&mut self, offset: usize) -> &mut T
    where
        T: Sized,
    {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SZ);
        self.modified = true;
        let addr = self.addr_of_offset(offset);
        unsafe { &mut *(addr as *mut T) }
    }
    /// Load a new BlockCache from disk.
    fn new(block_id: usize, block_device: Arc<dyn BlockDevice>) -> Self {
        let mut cache = [0u8; BLOCK_SZ];
        block_device.read_block(block_id, &mut cache);
        Self {
            cache,
            block_id,
            block_device,
            modified: false,
        }
    }
}
impl Cache for BlockCache {
    /// The read-only mapper to the block cache
    fn read<T, V>(&self, offset: usize, f: impl FnOnce(&T) -> V) -> V {
        f(self.get_ref(offset))
    }

    /// The mutable mapper to the block cache    
    fn modify<T, V>(&mut self, offset: usize, f: impl FnOnce(&mut T) -> V) -> V {
        let ret = f(self.get_mut(offset));
        return ret;
    }
}
impl BlockCache {
    /// Synchronize the cache with the external storage, i.e. write it back to the disk.
    fn sync(&mut self) {
        if self.modified {
            self.modified = false;
            self.block_device.write_block(self.block_id, &self.cache);
        }
    }
}
const BLOCK_CACHE_SIZE: usize = 16;

pub struct BlockCacheManager {
    /// # Fields
    /// * `0`: `usize`, the Corresponding `block_id`
    /// * `1`: `Arc<Mutex<BlockCache>>`, the Pointer to BlockCache
    /// # Impl. Info
    /// Using RwLock for concurrent access.
    queue: RwLock<VecDeque<(usize, Arc<Mutex<BlockCache>>)>>,
}

impl BlockCacheManager {
    fn new() -> Self {
        Self {
            queue: RwLock::new(VecDeque::with_capacity(BLOCK_CACHE_SIZE)),
        }
    }
}
impl CacheManager for BlockCacheManager {
    type CacheType = BlockCache;
    const CACHE_SZ: usize = 512;

    fn try_get_block_cache(
        &mut self,
        block_id: usize,
        _inner_blk_id: usize,
    ) -> Option<Arc<Mutex<BlockCache>>> {
        if let Some(pair) = self.queue.read().iter().find(|pair| pair.0 == block_id) {
            Some(Arc::clone(&pair.1))
        } else {
            None
        }
    }

    fn get_block_cache<FUNC>(
        &mut self,
        block_id: usize,
        inner_blk_id: usize,
        _neighbor: FUNC,
        block_device: Arc<dyn BlockDevice>,
    ) -> Arc<Mutex<BlockCache>>
    where
        FUNC: Fn() -> Vec<usize>,
    {
        if let Some(i) = self.try_get_block_cache(block_id, inner_blk_id) {
            i
        } else {
            // substitute
            let rd = self.queue.read();
            let size = self.queue.read().len();
            drop(rd);
            if size == BLOCK_CACHE_SIZE {
                // from front to tail
                let rd = self.queue.read();
                if let Some((idx, _)) = rd
                    .iter()
                    .enumerate()
                    .find(|(_, pair)| Arc::strong_count(&pair.1) == 1)
                {
                    drop(rd);
                    self.queue.write().drain(idx..=idx);
                } else {
                    panic!("Run out of BlockCache!");
                }
            }
            // load block into mem and push back
            let block_cache = Arc::new(Mutex::new(BlockCache::new(
                block_id,
                Arc::clone(&block_device),
            )));
            self.queue
                .write()
                .push_back((block_id, Arc::clone(&block_cache)));
            block_cache
        }
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        Self::new()
    }
}

const IMAGE_PATH: &str = "../../os_2021/easy-fs-fuse/fat32.img";

lazy_static! {
    pub static ref BLOCK_CACHE_MANAGER: Arc<Mutex<BlockCacheManager>> =
        Arc::new(Mutex::new(BlockCacheManager::new()));
    pub static ref FILE_SYSTEM: Arc<EasyFileSystem<BlockCacheManager, BlockCacheManager>> =
        EasyFileSystem::open(
            Arc::new(BlockFile(Mutex::new(
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(IMAGE_PATH)
                    .unwrap()
            ))),
            BLOCK_CACHE_MANAGER.clone()
        );
    pub static ref ROOT: Arc<easy_fs::Inode<BlockCacheManager, BlockCacheManager>> = Inode::new(
        FILE_SYSTEM.root_clus,
        DiskInodeType::Directory,
        None,
        None,
        FILE_SYSTEM.clone(),
    );
}

use test::Bencher;

#[bench]
fn bench_create(b: &mut Bencher) {
    //    let ls0 = &ROOT.ls()[0];
    b.iter(|| {
        // if ROOT.fs.fat.cnt_all_fat(&ROOT.fs.block_device) == 0 {
        //     Inode::delete_from_disk(find_local(&ROOT, "cat".to_string()).unwrap()).unwrap();
        // }
        /* println!(
         *     "============FREE FAT: {}===========",
         *     ROOT.fs.fat.cnt_all_fat(&ROOT.fs.block_device)
         * ); */
        (0..20).for_each(|i: i32| {
            println!("Working on {}th creation.", i);
            if Inode::create(ROOT.clone(), i.to_string(), DiskInodeType::File).is_ok() {
                println!("Done with {}th creation.", i);
                Inode::delete_from_disk(find_local(&ROOT, i.to_string()).unwrap()).unwrap();
            } else {
                println!("Error on the {}th creation.", i);
            }

            /* println!(
             *     "============FREE FAT: {}===========",
             *     ROOT.fs.fat.cnt_all_fat(&ROOT.fs.block_device)
             * ); */
        });
        // panic!("wdnmd");
    })
}

const BUFFER_SIZE: usize = 8192;
const ZERO: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

/* #[bench]
 * fn bench_write(b: &mut Bencher) {
 *     b.iter(|| {
 *         //    Inode::create(ROOT.clone(), "test".to_string(), DiskInodeType::File).unwrap();
 *         if let Some(test) = find_local(ROOT.clone(), "test".to_string()) {
 *             (0..1024).for_each(|i| {
 *                 let sz = test.file_size();
 *                 if ROOT.fs.fat.cnt_all_fat(&ROOT.fs.block_device) >= sz + i * BUFFER_SIZE {
 *                     test.modify_size(-(sz as isize));
 *                 }
 *                 println!(
 *                     "============FREE FAT: {}===========",
 *                     ROOT.fs.fat.cnt_all_fat(&ROOT.fs.block_device)
 *                 );
 *                 test.write_at_block_cache(i * BUFFER_SIZE, &ZERO);
 *             })
 *         } else {
 *             let test =
 *                 Inode::create(ROOT.clone(), "test".to_string(), DiskInodeType::File).unwrap();
 *             (0..12).for_each(|i| {
 *                 let sz = test.file_size();
 *                 if ROOT.fs.fat.cnt_all_fat(&ROOT.fs.block_device) >= sz + i * BUFFER_SIZE {
 *                     test.modify_size(-(sz as isize));
 *                 }
 *                 println!(
 *                     "============FREE FAT: {}===========",
 *                     ROOT.fs.fat.cnt_all_fat(&ROOT.fs.block_device)
 *                 );
 *                 test.write_at_block_cache(i * BUFFER_SIZE, &ZERO);
 *             })
 *         }
 *     })
 * } */

#[bench]
fn bench_write_2(b: &mut Bencher) {
    b.iter(|| {
        let lock = BLOCK_CACHE_MANAGER.lock();
        let mut wr = lock.queue.write();
        for i in wr.iter_mut() {
            let mut j = i.1.lock();
            j.sync();
            drop(j);
        }
        drop(wr);
        drop(lock);
        if let Some(i) = ROOT
            .ls(DirFilter::None)
            .iter()
            .find(|i| i.0 == "cat")
            .map(|i| (Inode::from_ent(&ROOT, &i.1, i.2)))
        {
            Inode::delete_from_disk(i);
            for i in ROOT.ls(DirFilter::None) {
                println!("{}", i.0);
            }
        }
        // for i in ROOT.ls(DirFilter::None) {
        //     if i.0.to_ascii_uppercase() == i.0 {
        //         let mut j = ROOT.iter();
        //         j.set_offset(i.2);
        //         println!("{:?}", j.current_clone().unwrap());
        //     }
        // }
        Inode::create(ROOT.clone(), "test".to_string(), DiskInodeType::File).unwrap();

        if let Some(test) = find_local(&ROOT, "test".to_string()) {
            let mut lock = test.file_content.lock();
            (0..5).for_each(|i| {
                /* println!(
                 *     "============FREE FAT: {}===========",
                 *     ROOT.fs.fat.cnt_all_fat(&ROOT.fs.block_device)
                 * ); */
                test.write_at_block_cache(&mut lock, i * BUFFER_SIZE, &ZERO);
            });
        } else {
            let test =
                Inode::create(ROOT.clone(), "test".to_string(), DiskInodeType::File).unwrap();
            let mut lock = test.file_content.lock();
            (0..5).for_each(|i| {
                /* println!(
                 *     "============FREE FAT: {}===========",
                 *     ROOT.fs.fat.cnt_all_fat(&ROOT.fs.block_device)
                 * ); */
                test.write_at_block_cache(&mut lock, i * BUFFER_SIZE, &ZERO);
            })
        }
        for i in ROOT.ls(DirFilter::None) {
            println!("{:?}", i);
        }
        Inode::delete_from_disk(find_local(&ROOT, "test".to_string()).unwrap()).unwrap();
        for i in ROOT.ls(DirFilter::None) {
            println!("{:?}", i);
        }
    })
}
