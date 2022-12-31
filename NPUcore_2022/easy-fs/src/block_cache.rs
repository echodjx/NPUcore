use super::BlockDevice;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

pub trait Cache {
    /// The read-only mapper to the block cache
    /// # Argument
    /// + `offset`: offset in cache
    /// + `f`: a closure to read 
    fn read<T, V>(&self, offset: usize, f: impl FnOnce(&T) -> V) -> V;
    /// The mutable mapper to the block cache
    /// # Argument
    /// + `offset`: offset in cache
    /// + `f`: a closure to write 
    fn modify<T, V>(&mut self, offset: usize, f: impl FnOnce(&mut T) -> V) -> V;
    /// Tell cache to write back
    /// # Argument
    /// + `block_ids`: block ids in this cache
    /// + `block_device`: The pointer to the block_device.
    fn sync(&self, _block_ids: Vec<usize>, _block_device: &Arc<dyn BlockDevice>) {}
}

pub trait CacheManager {
    /// The constant to mark the cache size.
    const CACHE_SZ: usize;

    type CacheType: Cache;

    /// Constructor to the struct.
    fn new() -> Self
    where
        Self: Sized;
    /// Tell cache manager to write back cache and release memory
    /// # Argument
    /// + `neighbor`: A closure to get block ids when cache miss.
    /// + `block_device`: The pointer to the block_device.
    /// # Return Value
    /// Number of caches freed
    fn oom<FUNC>(
        &self,
        _neighbor: FUNC,
        _block_device: &Arc<dyn BlockDevice>
    ) -> usize
    where
        FUNC: Fn(usize) -> Vec<usize>
    {
        unreachable!()
    }
    /// When file size changed, we should notify cache manager to drop some cache
    /// # Argument
    /// + `new_size`: File's new size
    fn notify_new_size(
        &self,
        _new_size: usize
    ) {
        unreachable!()
    }
}

pub trait BlockCacheManager: CacheManager {
    /// Try to get the block cache and return `None` if not found.
    /// # Argument
    /// + `block_id`: The demanded block id(for block cache).
    /// + `inner_cache_id`: The ordinal number of the cache inside the file(for page cache).
    /// # Return Value
    /// If found, return Some(pointer to cache)
    /// otherwise, return None
    fn try_get_block_cache(
        &self,
        block_id: usize,
    ) -> Option<Arc<Mutex<Self::CacheType>>>;

    /// Attempt to get block cache from the cache.
    /// If failed, the manager should try to copy the block from sdcard.
    /// # Argument
    /// + `block_id`: The demanded block id(for block cache).
    /// + `inner_cache_id`: The ordinal number of the cache inside the file(for page cache).
    /// + `neighbor`: A closure to get block ids when cache miss.
    /// + `block_device`: The pointer to the block_device.
    /// # Return Value
    /// Pointer to cache
    fn get_block_cache<FUNC>(
        &self,
        block_id: usize,
        block_device: &Arc<dyn BlockDevice>,
    ) -> Arc<Mutex<Self::CacheType>>;
}

pub trait PageCacheManager: CacheManager {
    /// Try to get the block cache and return `None` if not found.
    /// # Argument
    /// + `block_id`: The demanded block id(for block cache).
    /// + `inner_cache_id`: The ordinal number of the cache inside the file(for page cache).
    /// # Return Value
    /// If found, return Some(pointer to cache)
    /// otherwise, return None
    fn try_get_page_cache(
        &self,
        inner_cache_id: usize,
    ) -> Option<Arc<Mutex<Self::CacheType>>>;

    /// Attempt to get block cache from the cache.
    /// If failed, the manager should try to copy the block from sdcard.
    /// # Argument
    /// + `block_id`: The demanded block id(for block cache).
    /// + `inner_cache_id`: The ordinal number of the cache inside the file(for page cache).
    /// + `neighbor`: A closure to get block ids when cache miss.
    /// + `block_device`: The pointer to the block_device.
    /// # Return Value
    /// Pointer to cache
    fn get_page_cache<FUNC>(
        &self,
        inner_id: usize,
        neighbor: FUNC,
        block_device: &Arc<dyn BlockDevice>,
    ) -> Arc<Mutex<Self::CacheType>>
    where
        FUNC: Fn() -> Vec<usize>;
}
