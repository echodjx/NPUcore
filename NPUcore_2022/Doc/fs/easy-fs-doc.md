我们仿照rcore对其easy-fs进行适配fat32文件系统的开发，整体思路与rcore相仿，采用松耦合模块化的设计思路让easy-fs与底层通过抽象接口进行连接避免与底层实现的绑定，实现了文件系统模块与内核其他模块的隔离。

easy-fs分成了两个不同的crate：

+ `easy-fs`为文件系统核心部分，它是一个库形式的crate，实现了fat32的文件系统磁盘布局。
+ `easy-fs-fuse`，是一个能够在开发环境中运行并能够测试`easy-fs`的程序。

`easy-fs`本身也划分了不同的层次，形成了层次化和模块化的设计架构
+ 块设备接口层
+ 块缓存接口层
+ 磁盘数据结构层
+ 简单文件系统层
+ 虚拟文件系统层


### 块设备接口层

定义设备驱动需要实现的块读写接口`BlockDevice`trait的块设备接口代码写在`block_dev.rs`中，在最底层声明了一个块设备的抽象接口：

```rust
pub trait BlockDevice: Send + Sync + Any {
    fn read_block(&self, block_id: usize, buf: &mut [u8]);
    fn write_block(&self, block_id: usize, buf: &[u8]);
    fn clear_block(&self, block_id: usize, num: u8) {
        self.write_block(block_id, &[num; BLOCK_SZ]);
    }
    fn clear_mult_block(&self, block_id: usize, cnt: usize, num: u8) {
        for i in block_id..block_id + cnt {
            self.write_block(i, &[num; BLOCK_SZ]);
        }
    }
}
```

它需要实现两个基础抽象方法：

+ `read_block`将`block_id`的块从磁盘读入内存中的缓冲区`buf`。
+ `write_block`将内存中的缓冲区`buf`中的数据写入磁盘编号为`block_id`的块。

它还内置了两种预实现的抽象方法：

+ `clear_block`用`num`填充磁盘编号为`block_id`的块。
+ `clear_mult_block`用`num`填充磁盘编号从`block_id`开始的连续`cnt`块

上面两种方法可以由块设备重实现，设计这两个接口的考量是SD卡有对应的执行指令，可以更高效地处理填充这样的应用场景。

### 块缓存接口层

为了减少块设备的I/O时间，提升系统性能，需要在内存中存放一个缓冲区，后续对数据块的大部分读写就可以直接在内存中完成，出于测试不同缓存策略对性能的影响的考量，定义了块缓存需要实现的接口`Cache`和`CacheManager`，代码写在`block_cache.rs`中，声明了块缓存的抽象接口：

```rust
pub trait Cache {
    fn read<T, V>(&self, offset: usize, f: impl FnOnce(&T) -> V) -> V;
    fn modify<T, V>(&mut self, offset: usize, f: impl FnOnce(&mut T) -> V) -> V;
}
```

`Cache`是单个块缓存的抽象接口，它需要实现两个基础抽象方法：

+ `read`读出该块起始地址在`offset`开始的`T`泛型变量，不过出于灵活性考量，通过传入一个函数`f`将读出过程化。
+ `write`写入该块起始地址在`offset`开始的`T`泛型变量，不过出于灵活性考量，通过传入一个函数`f`将写入过程化，而不需要传入缓冲区地址。

```rust
pub trait CacheManager {
    const CACHE_SZ: usize;
    type CacheType: Cache;

    fn new() -> Self
    where
        Self: Sized;

    fn try_get_block_cache(
        &mut self,
        block_id: usize,
        inner_cache_id: usize,
    ) -> Option<Arc<Mutex<Self::CacheType>>>;

    fn get_block_cache<FUNC>(
        &mut self,
        block_id: usize,
        inner_cache_id: usize,
        neighbor: FUNC,
        block_device: Arc<dyn BlockDevice>,
    ) -> Arc<Mutex<Self::CacheType>>
    where
        FUNC: Fn() -> Vec<usize>;
}
```

`CacheManager`是多个缓存块的管理器的抽象接口，用于管理多个`Cache`。它需要实现一个抽象常量和三个抽象方法：

+ `CACHE_SZ`用于描述管理器管理的`CACHE`的大小，这个变量是出于不同的`CACHE`它们大小会是不同的考虑，一个简单的例子是对于一个页缓存它的块大小是4096字节，而对于fat表缓存它的块大小是512字节。
+ `new`用于初始创建管理器。
+ `try_get_block_cache`该函数**尝试**获取传入参数`block_id`（磁盘编号）和`inner_cache_id`（管理器内部编号）对应的`Cache`。
+ `get_block_cache`该函数获取传入参数`block_id`（磁盘编号）和`inner_cache_id`（管理器内部编号）对应的`Cache`。如果内存中没有，则会通过`block_device`从块设备中读入对应的内容（`neighbor`作为辅助函数可以获取读入的其余信息）。

`try_get_block_cache`和`get_block_cache`这两个抽象接口在实际实现的时候发现有些参数由于历史遗留因素显得冗余，这两个接口会在将来的开发中不断优化。

### 磁盘数据结构层

磁盘数据结构用于描述磁盘信息。

`BPB`，位于`layout.rs`，用于描述块设备的第一块扇区信息，参数遵守微软的文档，这里不作详细的展开。

`FSInfo`，位于`layout.rs`，用于描述FAT文件系统相关信息，参数遵守微软的文档，这里不作详细的展开，不过由于在实现过程中发现这部分信息并不刚需，所以暂且搁置不用。

`FATDirEnt`，位于`layout.rs`，用于描述目录项，它有两种形式：长目录项`FATLongDirEnt`和短目录项`FATShortDirEnt`，用union封装起来。目录项具体参数遵守微软文档，这里不作详细的展开。

```rust
pub union FATDirEnt {
    pub short_entry: FATShortDirEnt,
    pub long_entry: FATLongDirEnt,
}
```

`Fat`，位于`bitmap.rs`，用于维护fat表，声明的数据结构如下：

```rust
pub struct Fat<T> {
    pub fat_cache_mgr: Arc<Mutex<T>>,
    start_block_id: usize,
    byts_per_sec: usize,
    tot_ent: usize,
    vacant_clus: Mutex<VecDeque<u32>>,
    hint: Mutex<usize>,
}
```

+ `fat_cache_mgr`fat表的cache管理器。
+ `start_block_id`第一张fat表的起始磁盘编号。
+ `byts_per_sec`每个扇区有多少字节。
+ `tot_ent`一张fat表里一共有多少个簇。
+ `vacant_clus`记录那些被释放的簇的簇号，可用于下次申请。
+ `hint`优化参数，遍历fat表申请空簇时可以直接从`hint`开始而不用从头开始，减少遍历时间。

`Fat`现能支持：
+ 申请簇（`alloc_mult, alloc_one`）
+ 释放簇（`mult_dealloc, dealloc`）
+ 根据文件起始簇号获得文件的所有簇号（`get_all_clus_num`）
+ 统计所有空簇（`cnt_all_fat`）

### 简单文件系统层

简单文件系统`EasyFileSystem`位于`efs.rs`，用于维护数据区，声明的数据结构如下：

```rust
pub struct EasyFileSystem<T: CacheManager, F: CacheManager> {
    used_marker: PhantomData<T>,
    pub block_device: Arc<dyn BlockDevice>,
    pub fat: Fat<F>,
    pub data_area_start_block: u32,
    pub root_clus: u32,
    pub sec_per_clus: u8,
    pub byts_per_sec: u16,
    ino_cnt: spin::Mutex<u64>,
}
```

+ `used_marker` 无用，用于绕过rust的编译检查。
+ `block_device`块设备指针。
+ `fat`fat指针。
+ `data_area_start_block`数据区开始的磁盘编号。
+ `root_clus`根目录的簇号。
+ `sec_per_clus`每个簇有多少扇区。
+ `byts_per_sec`每个扇区有多少字节。
+ `ino_cnt`此参数本意是分配inode号，但功能未开发完全，先搁置。

`EasyFileSystem`现能支持：
+ 簇和扇区之间的转化（`first_sector_of_cluster`, `in_cluster`）
+ 打开文件（`open`）
+ 获取根节点inode（`root_inode`）

### 虚拟文件系统层

虚拟文件系统用于创建文件索引节点与内核的对接。

`FileContent`，位于`vfs.rs`，用于描述文件的内容，声明的数据结构如下：

```rust
pub struct FileContent<T: CacheManager> {
    pub size: u32,
    pub clus_list: Vec<u32>,
    pub file_cache_mgr: T,
    pub hint: u32,
}
```

+ `size`文件大小。
+ `clus_list`文件包含的簇号。
+ `file_cache_mgr`文件的cache管理器。
+ `hint`此参数本意是维护目录文件的特殊目录项（用于告知内核下面没东西了）的位置，但功能未开发完全，暂且搁置。

`Inode`，位于`vfs.rs`，用于描述文件信息，声明的数据结构如下：

```rust
pub struct Inode<T: CacheManager, F: CacheManager> {
    pub file_content: Mutex<FileContent<T>>,
    pub file_type: ReadOnly<DiskInodeType>,
    pub parent_dir: Option<(Arc<Self>, u32)>,
    pub fs: Arc<EasyFileSystem<T, F>>,
    pub time: Mutex<InodeTime>,
}
```

+ `file_content`文件内容。
+ `file_type`文件类型。
+ `parent_dir`父节点的Inode指针以及该文件对应父节点的偏移量。
+ `fs`简单文件系统指针。
+ `time`描述文件的打开时间等相关信息。

`Inode`现能支持：
+ 文件创建（`create`）
+ 文件删除（`delete_from_disk`）
+ 文件读写（`read_at_block_cache, write_at_block_cache`）
+ 文件修改大小（`modify_size`）
+ 文件遍历和查询（`ls, find_local`）

`DirIter`，位于`dir_iter.rs`，是一种用于目录项的迭代器，声明的数据结构如下：

```rust
pub struct DirIter<'a, T: CacheManager, F: CacheManager> {
    pub lock: MutexGuard<'a, FileContent<T>>,
    pub offset: Option<u32>,
    pub mode: DirIterMode,
    pub forward: bool,
    pub inode: &'a Inode<T, F>,
}
```

+ `lock`维护`FileContent`的锁。
+ `offset`当前指针指向的偏移量。
+ `mode`偏移模式，目前有6种模式可供选择。
+ `forward`描述指针是往偏移量增加/减少的方向移动。
+ `inode`文件指针。

这种迭代器的设计低耦合，可以随时根据需求进行逻辑的组合生成更加复杂的逻辑，同文件下的`DirWalker`就是一个很好的例子。