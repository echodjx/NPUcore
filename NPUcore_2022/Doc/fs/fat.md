
# Table of Contents

1.  [前言](#org7567225)
    1.  [简介](#org4ca9ea4)
    2.  [参考文献](#orga68777f)
        1.  [常见](#org1744556)
        2.  [不太常见的资料](#org11138e8)
        3.  [上述文献的阅读建议](#org4ec4117)
    3.  [鸣谢](#orge96f20a)
2.  [部分需要强调的FAT文件系统布局](#orgf70dc58)
    1.  [FAT32 Layout](#orgddcec46)
        1.  [Reserved Area](#orgc5c572b)
        2.  [FAT Region[(usu. 2)]](#orgbe25520)
        3.  [File and Directory Data Region](#org9d51d01)
    2.  [文件目录项概述(具体建议看之前提到的文档)](#orgf6cc60e)
        1.  [长短目录项](#org967035e)
        2.  [文件名编码](#orgafe4cc5)
        3.  [排列方式](#orgf3b0de8)
    3.  [其他需要知道的基本信息](#org9462388)
3.  [改造](#orga4c2ba7)
    1.  [rCore FS/VFS 布局 & 设计及我们的改造要求](#org3dfad7d)
        1.  [Inode(vfs.rs)](#org594bd7a)
        2.  [Disk Manager layer(efs.rs)](#org82356e3)
        3.  [Disk block allocation layer(bitmap.rs)](#org2536881)
        4.  [Disk(layout.rs)](#org974d192)
        5.  [BlockCache(block\_cache.rs)](#org00527ab)
        6.  [BlockDevice(the Driver)(block\_dev.rs)](#orgcb1aa08)
    2.  [具体实施](#org154a700)
        1.  [内部可变性](#org4c89d1e)
        2.  [CacheManager的trait化](#org34bd534)
        3.  [块分配(bitmap.rs)](#org5c82814)
        4.  [Inode结构的合并和改造](#org0eeed52)
        5.  [目录项](#org6c0b2f1)
    3.  [Short Name generation](#org80e5b5c)
        1.  [需求分析](#orgf38667f)
        2.  [基本名和扩展名的生成](#orgf309043)
        3.  [一些细节](#org19ca56e)
4.  [目录树/文件打开表](#org70a6008)
    1.  [层次](#org88e7a27)
    2.  [目录树](#orge4f7afd)
    3.  [文件打开表](#org2cb661e)
        1.  [最初的尝试](#org146faa7)
        2.  [绕过失败](#org4d3d2fb)
        3.  [最后版本](#orga95cdab)
5.  [后续的思路](#org5042b38)



<a id="org7567225"></a>

# 前言

本文档遵循GNUDOC协议,用户有权自由使用和传播.


<a id="org4ca9ea4"></a>

## 简介

这是一份如何从关于我们从rCore::EasyFS改造出FAT32文件系统"前端"的过程的文档(也就是说本文件系统支持多种后端).
本文假设阅读者对FAT文件系统原理已经有了一定的了解(具体来说,和阅读并理解Tanenbaum的 *Modern Operating System* FAT部分相当的基础知识), 但对部分技术细节(如短文件名的生成)可以不完全了解.

但是,我们在一些时候省略了部分FAT"几何"(数据结构)计算的内容,这些内容可以从代码和技术规格以及参考文献中找到


<a id="orga68777f"></a>

## 参考文献


<a id="org1744556"></a>

### 常见

1.  微软的FAT系列文件系统(exFAT除外)技术规格和夏新手机厂技术人员04年的翻译版(网传中文版)

    微软的技术资料叫做fatgen103.pdf,基本可以搜索到;
    
    中文翻译文件: FAT32中文版.pdf
    
    注意,后者在一些字段长度上有错,需要配合原版进行阅读

2.  哈佛大学操作系统原理课的课程Slide(PDF)和作业指导

    用国外的搜索引擎搜索下列字符串
    
        FAT32 File Structure
        Prof. James L. Frankel
        Harvard University
        Version of 9:45 PM 24-Mar-2021


<a id="org11138e8"></a>

### 不太常见的资料

1.  某大佬提供的FAT32的技术资料

    "FAT32文件系统详解.pdf", 部分搜索引擎可以找到


<a id="org4ec4117"></a>

### 上述文献的阅读建议

最正宗的微软的技术规格自然是一切的根本,但不建议直接阅读,毕竟很多的技术规范在实际中不见得有人完全实现了,
甚至你不实现对于比赛和实际使用的影响也没有想象中大,而且其中也不都是和现代操作系统使用相关的,
而是向下兼容采取的权宜之计;但是,对于实现真正商品级的产品而不是玩具,这些规格必须遵守.

中文版的资料在FAT本身的文件系统的文件分配表(File Allocation Table)的原理上叙述比微软要详细的多,实际上,其用整整几页(Pg9~Pg11)叙述的FAT的文件识别原理
在英文版中几乎一笔带过,还不如你在操作系统课程上学得来的详细,而常见的中文版已经用下划线标出了笔记,所以体验还行.但对于形象的解释,其实后两个"不太常见的资料"也不错

文件目录项部分哈佛的资料还算不错,除了技术规格还讲了很多实现上需要绕过的坑,比技术规格信息多,但技术规格也可看一看

短文件名生成&#x2026;建议看Linux代码


<a id="orge96f20a"></a>

## 鸣谢

感谢我的指导老师,两位队友对我这种不靠谱的人的理解和支持和(主要是)容忍.

最后,请让我膜拜伟大的THU和Harvard University的各位大佬orz

(虽然可能只是他们一次信手拈来的作业罢了),也感谢这些巨佬和前辈的付出给我们提供了良好的框架和学习的对象


<a id="orgf70dc58"></a>

# 部分需要强调的FAT文件系统布局

出于简单考虑,本文献不考虑多分区/分区表处理
而实际上,如果有需要,可以在原本的SDCard层上再抽象出一个分区对象,然后按照常规思路继续实现.


<a id="orgddcec46"></a>

## FAT32 Layout


<a id="orgc5c572b"></a>

### Reserved Area

1.  DBR(Dos Boot Record, not mentioned in FAT32 layout)

    In our implementation, BS is used along with BPB.
    
    Besides, the items of the two are mixed instead of separated.
    
    1.  (DOS)Boot Sector
    
        marked with BS.
    
    2.  BPB


<a id="orgbe25520"></a>

### FAT Region[(usu. 2)]


<a id="org9d51d01"></a>

### File and Directory Data Region


<a id="orgf6cc60e"></a>

## 文件目录项概述(具体建议看之前提到的文档)


<a id="org967035e"></a>

### 长短目录项

FAT的目录项分为两种,长目录项和短目录项,长度一样所以本质上是C语言中的union.

区分的标志是其attribute域的类型,凡是全1的就是长目录项.


<a id="orgafe4cc5"></a>

### 文件名编码

由于历史原因, 短目录项的编码为UTF-8, 长目录项为UTF-16, 此外,长文件名的结尾是0,且之后一般填充0, 短文件名的结尾是32(空格),且排列是: 8+3, 8字符基本名+3字符扩展名,按照UTF-8个数向下取整(不允许半个字符的情况出现)


<a id="orgf3b0de8"></a>

### 排列方式

长目录项是从序号高到低排列的,之后跟一个短目录项,其初始序号为1,由其中的ord字节(第一个字节)控制


<a id="org9462388"></a>

## 其他需要知道的基本信息

对SD卡而言,一般来说物理的BLOCK\_SZ为512,这似乎是写入技术规格的

就我们的硬件而言,SD卡的block就是sector, 这两个词语经常混用,但对于硬盘而言这是不可以的


<a id="orga4c2ba7"></a>

# 改造


<a id="org3dfad7d"></a>

## rCore FS/VFS 布局 & 设计及我们的改造要求

以下资料按照自顶向下的思路写成,结构如下:


<a id="org594bd7a"></a>

### Inode(vfs.rs)

管理索引节点（即文件控制块）数据结构，并实现文件创建/文件打开/文件读写等成员函数来向上支持文件操作相关的系统调用的处理,数据结构只有Inode. 

经过代码阅读,DiskInode(实际在layout.rs)的read/write相关函数在更改了get\_block\_id()的情况下就能应用在全局型缓存管理器下,而如果是每个文件一个缓存管理器的架构,则完全不需要get\_block\_id()的改动(因为这个函数的作用是,给定特定块在),但出于兼容性我们保留了这一特性.

另外,modify\_size()需要根据我们的FAT的特性重写


<a id="org82356e3"></a>

### Disk Manager layer(efs.rs)

合并了上述核心数据结构和磁盘布局所形成的磁盘文件系统数据结构，以及创建/打开文件系统的相关处理和磁盘块的分配和回收处理

重要的数据结构只有EasyFileSystem


<a id="org2536881"></a>

### Disk block allocation layer(bitmap.rs)

数据结构只有Bitmap一个,
其对应的函数如下

    pub fn new(start_block_id: usize, blocks: usize) -> Self;
    
    pub fn alloc(&self, block_device: &Arc<dyn BlockDevice>) -> Option<usize>;
    
    pub fn dealloc(&self, block_device: &Arc<dyn BlockDevice>, bit: usize);
    
    pub fn maximum(&self) -> usize;

另外加上一个decomposition,是easy-fs中用于压缩位图的函数,对我们用处不大


<a id="org974d192"></a>

### Disk(layout.rs)

Layout磁盘上的超级块、位图、索引节点、数据块、目录项等核心数据结构和相关处理, 这里都是磁盘上的实际操作和磁盘上的数据结构,包括:
SuperBlock (文件系统元数据), 

DiskInode (磁盘上的文件元数据), 考虑到FAT本身比较需要我们会把这个移动到vfs.rs中实现

DirEntry (磁盘上的目录项)


<a id="org00527ab"></a>

### BlockCache(block\_cache.rs)

在内存中建立磁盘上的内容的缓存.主要结构为: BlockCache 和 BlockCacheManager,这里的改造目标主要是trait化和泛型化,从而适应不同的后端,特别是FAT和Inode分别使用不同后端的情况


<a id="orgcb1aa08"></a>

### BlockDevice(the Driver)(block\_dev.rs)

抽象的块设备,用于从外部储存而非缓存读入内容,这里基本保持不变


<a id="org154a700"></a>

## 具体实施


<a id="org4c89d1e"></a>

### 内部可变性

为了给前后段提供尽可能大的灵活性,我们在设计时尽量让所有的函数都使用不可变引用,这是为了在外部尽可能屏蔽,同时为未来改成多核安全的代码经可能留出

1.  对外套锁

    例如,在我们的实现中,vfs中的Inode的FileContent和InodeTime(用于实现时间的API和fstat/getdents64等)是用Mutex套住的
    
        pub struct Inode<T: CacheManager, F: CacheManager> {
            /// File Content
            pub file_content: Mutex<FileContent<T>>,
            /// Struct to hold time related information
            pub time: Mutex<InodeTime>,
            /*忽略其他域*/
        }
    
    这样,其大多数的api,甚至包括read/write都可以用不可变引用和Arc包裹
    
    锁的粒度更小使得允许一定程度上的并行访问,后期也许改为RwLock更好

2.  对内传锁

    事实上,我们如果需要访问/更改任何设计FileContent的内容,需要自行拿锁,例如在modify\_size()中
    
        if let Some((par_inode, offset)) = &self.parent_dir {
           let mut lock = par_inode.file_content.lock();
              par_inode.read_at_block_cache(&mut lock, *offset as usize, dir_ent.as_bytes_mut());


<a id="org34bd534"></a>

### CacheManager的trait化

1.  定义

    我们尽量不赘述其中注释已经有的内容
    
        pub trait Cache {
            /// The read-only mapper to the block cache
            fn read<T, V>(&self, offset: usize, f: impl FnOnce(&T) -> V) -> V;
            /// The mutable mapper to the block cache
            fn modify<T, V>(&mut self, offset: usize, f: impl FnOnce(&mut T) -> V) -> V;
        }
        
        pub trait CacheManager {
            /// The constant to mark the cache size.
            const CACHE_SZ: usize;
        
            type CacheType: Cache;
        
            /// Constructor to the struct.
            fn new() -> Self
            where
                Self: Sized;
        
            /// Try to get the block cache and return None if not found.
            /// # Argument
            /// block_id: The demanded block.
            /// inner_blk_id: The ordinal number of the block inside the block.
            /// inode_id: The inode_id the block cache belongs to.
            fn try_get_block_cache(
                &mut self,
                block_id: usize,
                inner_cache_id: usize,
            ) -> Option<Arc<Mutex<Self::CacheType>>>;
        
            /// Attempt to get block cache from the cache.
            /// If failed, the manager should try to copy the block from sdcard.
            /// # Argument
            /// block_id: The demanded block.
            /// inner_blk_id: The ordinal number of the block inside the block.
            /// inode_id: The inode_id the block cache belongs to.
            /// block_device: The pointer to the block_device.
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
    
    1.  FUNC闭包
    
        FUNC 是对应在Cache和Block的大小不同的时候,用于在后端一次性获取同一块Cache所有的其他的Blocks
        
        并且,由于使用了闭包,该获取过程是lazy的,意味着如果不主动执行,开销就不会很大,这可以用于兼容修改版的清华后端.
    
    2.  CACHE\_SZ
    
        该常数用于支持不同大小的Cache,每个Cache Manager必须且只能有1个,其原本应当属于Cache,但为了调用方便放在Manager里
    
    3.  索引方式
    
        此外,索引方式共计有两种,分别是 block\_id 和 inner\_cache\_id , 区别如下:block\_id是对于整个块设备的全局block\_id,而inner\_cache\_id是cache数组的内部的序号,之所以分成两种是因为存在两类的索引方式: 如果Cache Manager是全局的,就用全局的block\_id, 但如果Cache Manager是每个文件都有一个(这样搜索的速度会加快,因为当我们获取Cache的时候必然已经掌握了Inode的指针,如果其挂在一起那么就可以比较方便地节省掉很大的查找cache块的开销),但缺点是其后端实现的时候回收和写回会有较大的逻辑复杂度,且空间上会存在一定的浪费
    
    4.  try\_get\_block\_cache
    
        实际上可以不实现的一个函数: 你直接返回None即可,他原本是为了在后期优化中于特定的直接访问Cache,失败就不尝试直接读取外存的用途,只是个预留的接口. 另外,也可以用作get\_block\_cache的第一步

2.  应用

    为了更好地优化,我们建议将FAT32的Cache Manager进行了trait化, 具体来说, 如何通过编译器语法检查是这里的重中之重,因为部分的特性需要实例化.
    我们采用泛型+trait的方式定义这三个类.考虑到很多时候Inode和FAT的存取特征不同,我们的实现允许对FAT和Inode使用不同的Cache Manager
    
    如果采用纯trait抽象,会发现在许多时候尝试实例化/返回特定的类型的时候会产生类似"Object unsafe"的错误. 下面的T为Inode的CacheMgr类型, F为FAT的CacheMgr类型
    
        pub struct Fat<T> {}
        impl<T: CacheManager> Fat<T> {}
    
    EasyFS
    
        pub struct EasyFileSystem<T: CacheManager, F: CacheManager> {    
            used_marker: PhantomData<T>,
            //剩下省略
        }
    
    vfs.rs(inode)
    
        /// The functionality of ClusLi & Inode can be merged.
        /// The struct for file information
        pub struct Inode<T: CacheManager, F: CacheManager> {}
    
    EasyFileSystem部分,之所以需要 used\_marker, 是为了在其中 root\_inode() 函数的返回 Arc<Inode<T, F>>
    
    这样的方案也不是十全十美,例如其如果不使用unsafe的裸指针解引用的方式会无法生成static对象存储部分对struct而言全局的数据.
    
    但其是能通过编译检查中相对简单的一类操作.如果你想在EasyFileSystem上移除T泛型和used\_marker(这也是可行的),只保留F泛型,那么你可能需要考虑在vfs中建立一个相同功能的函数或者手动inline这段代码,并移除root\_inode()函数.


<a id="org5c82814"></a>

### 块分配(bitmap.rs)

FAT的块分配使用的数据结构为链表,所以其分配部分的easy-fs简单,

但由于FAT32上其FAT起到了的(ext4下的)Inode和bitmap的结构,其部分功能需要拓展.

具体用到的重要函数有:

1.  new

    生成不变,但是其需要的参数有变, FAT的初始化至少需要(start\_block\_id, byts\_per\_sec, tot\_ent)几个参数,并且如果还需要加速,需要存下所有已经回收的cluster号,所以需要一个Vec. 如果每次都从磁盘上搜索整个fat,除非是很小的fat表,否则开销极大,不建议在除了debug的场景下使用

2.  FAT-dependent

    1.  计算部分
    
            pub fn this_fat_inner_cache_num(&self, n: u32) -> usize
            pub fn this_fat_inner_sec_num(&self, n: u32) -> usize
            pub fn this_fat_sec_num(&self, n: u32) -> usize
            pub fn this_fat_ent_offset(&self, n: u32) -> usize
        
        这几个是用于计算FAT的位置,建议自行查看这些函数的注释
        
        inner是指不加偏移量的块号,即这一块是内部第几块/第几个缓存
        
        而cahce\_num则是指代缓存号,注意缓存的大小不一定等于Cache大小
        
        但其公式基本来自于微软的技术规格,实际上对多数实现者而言重复实现意义不大
        
        需要注意, 在实现this\_fat\_ent\_offset的时候,所用的块大小应该对应 T::CACHE\_SZ
        
            pub fn this_fat_ent_offset(&self, n: u32) -> usize {
                let fat_offset = n * 4;
                (fat_offset % (T::CACHE_SZ as u32)) as usize
            }
    
    2.  next\_clus的获取和修改(新增)
    
        函数共有2个,
        
        set\_next\_clus\_num()
        
        get\_next\_clus\_num()
        
        由于FAT的文件系统设计,我们将分配和块获取写在一起.
        注意,由于其FAT entry只使用了前28位,必须对结果取EOC.具体高4位的用途是检查(移动)介质是否正常弹出(见哈佛的ppt)
        但这里我们不涉及. 

3.  分配系列函数

    我们这里只介绍one系列函数,但其实是可以做成单次拿锁的函数的(拿锁是开销很大的行为)
    
    1.  分配
    
        1.  alloc\_one
        
            首先展示大体的框架,首先第一个if是检查是(如果有挂载参数的话,没有就略过该检查)否是最后一个cluster,不是就直接返回None,因为目前只有最后一个cluster支持扩展长度
            
                pub fn alloc_one(
                    &self,
                    block_device: &Arc<dyn BlockDevice>,
                    attach: Option<u32>,
                ) -> Option<u32> {
                    if attach.is_some()
                        && self.get_next_clus_num(attach.unwrap(), block_device) < FAT_ENTRY_RESERVED_TO_END
                    {
                        return None;
                    }
            
            然后是尝试获取一块空闲簇,如果空闲簇获取成功,检查"挂载参数"attach是否为Some,如果是则设定其下一个为
            
                    // now we can alloc freely
                    if let Some(next_clus_id) = self.alloc_one_no_attach_locked(block_device) {
                        if attach.is_some() {
                            self.set_next_clus(block_device, attach.unwrap(), next_clus_id);
                        }
                        return Some(next_clus_id);
                    }
                    None
                }
        
        2.  alloc\_one\_no\_attach\_locked
        
            其次是获取单个簇的具体动作,如果能从空闲块中获取,就获取空闲块,否则就搜索FAT获取新块,然后填入EOC
            
            注意到,这种搜索策略实际上是不完善的:其寻找的方向是从hint开始,然后向后,那么如果走完下半个盘就会失败,
            
            但实际上之前可能还有一些没有收集到的簇,但对我们的测试用例和比赛环境而言反而是更快的:因为其1.5GB大小的SD卡基本不会发生耗尽的问题,而删除则是基本上在本次发生,恶已久会被记录在vacant\_clus中,从头开始搜索(微软的推荐方法)反而更慢
            
                fn alloc_one_no_attach_locked(&self, block_device: &Arc<dyn BlockDevice>) -> Option<u32> {
                    // get from vacant_clus
                    if let Some(clus_id) = self.vacant_clus.lock().pop_back() {
                        // modify cached
                        self.set_next_clus(block_device, clus_id, EOC);
                        return Some(clus_id);
                    }
                
                    let mut hlock = self.hint.lock();
                    let start = *hlock;
                    let free_clus_id = self.get_next_free_clus(start as u32, block_device);
                    if free_clus_id.is_none() {
                        return None;
                    }
                    let free_clus_id = free_clus_id.unwrap();
                    *hlock = (free_clus_id + 1) as usize % self.tot_ent;
                    drop(hlock);
                
                    self.set_next_clus(block_device, free_clus_id, EOC);
                    Some(free_clus_id)
                }
    
    2.  释放
    
        没什么好说的,fat entry设为空并投入vacant\_clus
    
    3.  mult与优化
    
        很惭愧,其实所有的mult相关的函数是可以只拿一次锁的,但如果时间不足,也可以用for循环多次执行alloc/dealloc实现.
        
        也许像vfs中inode的各个函数一样显式拿锁,手动传锁会能更好地减小开销


<a id="org0eeed52"></a>

### Inode结构的合并和改造

考虑到FAT块分配和块检索是一体的,且其块的读取相对简单,加上easy-fs中的inode本身也基本就是其封装,我们可以将其DiskInode和Inode两个数据结构合并为一个,从而减少层级.具体来说,其对外接口应当保留,而应当考虑将DiskInode的各种read/write都暴露到外部. 这里解释一些比较复杂的函数,其他建议具体看代码

1.  FileContent

    我们建议将以下4个放在一起,用Mutex锁住同时修改
    
        pub struct FileContent<T: CacheManager> {
            /// For FAT32, size is a value computed from FAT.
            /// You should iterate around the FAT32 to get the size.
            pub size: u32,
            /// The cluster list.
            pub clus_list: Vec<u32>,
            /// File cache manager corresponding to this inode.
            pub file_cache_mgr: T,
            /// If this file is a directory, hint will record the position of last directory entry(the first byte is 0x00).
            pub hint: u32,
        }
    
    因为其修改事实上是事务性的,如果用多个锁则可能有不一致性,但锁的更大有可能并行性降低
    
    关于file\_cache\_mgr的做法,如果很需要Arc指向全局Manager,建议直接用自定义struct套住,甚至是(), 例如:
    
        pub struct DataCacheMgrWrapper {
            empty: (),
        }
        impl CacheManager for DataCacheMgrWrapper {
            const CACHE_SZ: usize = BLOCK_SZ;
            type CacheType = $cache;
            fn new() -> Self
            where
                Self: Sized,
            {
                Self { empty: () }
            }
            fn try_get_block_cache(
                &mut self,
                block_id: usize,
                inner_cache_id: usize,
            ) -> Option<Arc<Mutex<Self::CacheType>>> {
                None
            }
            fn get_block_cache<FUNC>(
                &mut self,
                block_id: usize,
                inner_cache_id: usize,
                neighbor: FUNC,
                block_device: Arc<dyn BlockDevice>,
            ) -> Arc<spin::Mutex<Self::CacheType>>
            where
                FUNC: Fn() -> Vec<usize>,
            {
                Arc::new(Mutex::new($cache::new(block_id)))
            }
        }
        pub struct DataBlockCacheWrapper {
            block_id: usize,
        }
    
    注意这种做法可能会造成重复套锁,需要考虑

2.  read/write

    原型: DiskInode的对应函数
    
    事实上,这两个函数修改的价值不大,清华已经很好地解决了相关问题了,我们只要替换DiskInode中的get\_block\_id即可(去FileContent的clus\_li转换)
    
    如果你的实现不需要block\_id, 其甚至不需要这种
    
        fn get_block_id(&self, lock: &MutexGuard<FileContent<T>>, blk: u32) -> Option<u32> {
            let idx = blk as usize / self.fs.sec_per_clus as usize;
            let clus_list = &lock.clus_list;
            if idx >= clus_list.len() {
                return None;
            }
            let base = self.fs.first_sector_of_cluster(clus_list[idx]);
            let offset = blk % self.fs.sec_per_clus as u32;
            Some(base + offset)
        }
    
    之所以清华的代码可以如此简单地替换,主要是因为其处理方式是用for循环不断获取当前inner\_cache\_id对应的block\_id,然后抽象出查询部分单独作函数

3.  modify\_size

    一开始主要是拿锁并获取当前目录项(如果有需要), 同时检测减小的大小超出size的情况
    
        pub fn modify_size(&self, lock: &mut MutexGuard<FileContent<T>>, diff: isize) {
            let mut dir_ent = FATDirEnt::empty();
        
            // Get parent lock and get directory entry of current file
            let mut may_par_lock: Option<MutexGuard<FileContent<T>>> = None;
            if let Some((par_inode, offset)) = &self.parent_dir {
                let mut lock = par_inode.file_content.lock();
                par_inode.read_at_block_cache(&mut lock, *offset as usize, dir_ent.as_bytes_mut());
                may_par_lock = Some(lock);
            }
            // This operation is ignored if the result size is negative
            if diff.saturating_add(lock.size as isize) <= 0 {
                return;
            }
    
    检查之后,如果是扩大则修改大小然后对0大小的修改dirent
    
        let old_size = lock.size;
        let new_size = (lock.size as isize + diff) as u32;
        
        let old_clus_num = self.total_clus(old_size) as usize;
        let new_clus_num = self.total_clus(new_size) as usize;
        
        if diff > 0 {
            self.alloc_clus(lock, new_clus_num - old_clus_num);
            lock.size = new_size;
            // If old size is 0, set first cluster bits in directory entry
            if old_size == 0 {
                dir_ent.set_fst_clus(lock.clus_list[0]);
            }
        } 
    
    对于大小缩小的情况,相对容易.注意,按照技术规格,dir\_ent的fst\_clus需要设为0(但如果保留最后一块,可以确保所有文件都能有至少一块,那么可以用first\_sector做inode number)
    
        else {
            lock.size = new_size;
            self.dealloc_clus(lock, old_clus_num - new_clus_num);
            // If new size is 0, clear first cluster bits in directory entry
            if new_size == 0 {
                dir_ent.set_fst_clus(0);
            }
        }
    
    最后写回,然后退出
    
                dir_ent.set_size(new_size);
                //println!("{}", new_size);
                // Write back
                if let Some((par_inode, offset)) = &self.parent_dir {
                    par_inode.write_at_block_cache(
                        &mut may_par_lock.unwrap(),
                        *offset as usize,
                        dir_ent.as_bytes_mut(),
                    );
                }
            }
        }

4.  create

    这里就是单纯的生成长文件名和父目录
    
        /// Create a file or a directory from the parent.
        pub fn create(
            parent_dir: &Arc<Self>,
            name: String,
            file_type: DiskInodeType,
        ) -> Result<Arc<Inode<T, F>>, ()> {
            if parent_dir.is_file()
                || name.len() >= 256
                || parent_dir
                    .ls()
                    .unwrap()
                    .iter()
                    .find(|(existed_name, _)| existed_name.to_uppercase() == name.to_uppercase())
                    .is_some()
            {
                Err(())
            } else {
                //get short name slice
                let mut short_name_slice: [u8; 11] = [' ' as u8; 11];
                if Self::gen_short_name_slice(&parent_dir, &name, &mut short_name_slice).is_err() {
                    return Err(());
                }
                //alloc parent's directory entries
                let lock = parent_dir.file_content.lock();
                let long_ent_num = name.len().div_ceil(LONG_DIR_ENT_NAME_CAPACITY);
                let short_ent_num = 1;
                let short_ent_offset =
                    Self::alloc_dir_ent(&parent_dir, lock, long_ent_num + short_ent_num);
                 if short_ent_offset.is_err() {
                    return Err(());
                }
                let (short_ent_offset, lock) = short_ent_offset.unwrap();
    
    注意到,对于一个新创建的文件夹,其总是有一个新的簇,但新的文件的则总是没有一个簇
    
    (但你可以认为指定一个,只是不符合技术规格,但理论上也能用)
    
    此外需要强调的是,文件夹的目录项中的大小始终为0,但文件夹的大小是数cluster chain得到的,
    
        //if file_type is Directory, alloc first cluster
        let fst_clus = if file_type == DiskInodeType::Directory {
            let fst_clus = parent_dir
                .fs
                .fat
                .alloc_one(&parent_dir.fs.block_device, None);
            if fst_clus.is_none() {
                return Err(());
            }
            fst_clus.unwrap()
        } else {
            0
        };
    
    生成并写回目录项,注意其序号从1开始
    
        // Generate short entry
        log::error!("short_name_slice: {:?}", short_name_slice);
        let short_ent = FATShortDirEnt::from_name(short_name_slice, fst_clus, file_type);
        // Generate long entries
        let mut long_ents = Vec::<FATLongDirEnt>::new();
        for i in 1..=long_ent_num {
            long_ents.push(FATLongDirEnt::from_name_slice(
                i == long_ent_num,
                i,
                Self::get_long_name_slice(&name, i),
            ))
        }
        // Write back parent's directory entry
        Self::write_back_dir_ent(&parent_dir, short_ent_offset, lock, short_ent, long_ents);
        //generate current directory
        let current_dir = Inode::from_ent(&parent_dir, &short_ent, short_ent_offset);
    
    对于文件夹还要填上".", ".."两个目录项,加上一个UNUSED\_AND\_LAST目录项(此由技术规格制定)
    
                //if file_type is Directory, set first 3 directory entry
                if file_type == DiskInodeType::Directory {
                    let lock = current_dir.file_content.lock();
                    //fill content
                    Self::fill_empty_dir(&parent_dir, &current_dir, lock, fst_clus);
                }
                Ok(current_dir)
            }
        }


<a id="org6c0b2f1"></a>

### 目录项

本质上,文件夹是一个保存了目录项数组的文件,对不同文件系统,其遍历需要单独适配.

由于历史原因,FAT的目录项相比easy-fs复杂很多,分为长目录项和短目录项,通过一个短目录项和多个长目录项的组合达到常规情况下同构多目录项即可完成的功能.因此我们单独用一模块处理.这是相对于清华新加入的.

关于其遍历有多个不同的思路,我们推荐将其通过两层iterator结构视为一层: 这样可以有效地得到目录项同时,一方面减小获取单个目录项的成本,一方面保持ls时的速度优势(通过编译器优化)

1.  第一层DirIter类

    1.  需求
    
            pub enum DirIterMode {
                LongIter,
                ShortIter,
                UsedIter,
                Unused,
                Enum,
                Dirent,
            }
        
        分别是只输出长目录项,短目录项,已用目录项(也就是长和短的并),未用目录项,所有目录项,以及预留的用于输出Linux Dirent的遍历模式(这个可以不管)
        
        此外,还需要支持双向移动. Rust有自己原生的双向Iterator,没有实现主要是历史原因,后续也许会改进
        
        此外不仅要能读出,还需要能写入当前目录项(方便就地修改)
    
    2.  具体实现
    
        当然,你也可以不借用MutexGuard, 而是采用外部调用的形式用裸指针解引用(曾经这是我们采用的技术方案),但这除了不太雅观之外,还有一定的其他风险
        这样使用的时候需要传入一个FileContent的MutexGuard.
        
            const STEP_SIZE: u32 = core::mem::size_of::<FATDirEnt>() as u32;
            pub struct DirIter<'a, T: CacheManager, F: CacheManager> {
                pub lock: MutexGuard<'a, FileContent<T>>,
                pub offset: Option<u32>,
                pub mode: DirIterMode,
                pub forward: bool,
                pub inode: &'a Inode<T, F>,
            }
        
        之所以需要一个Option包裹offset是因为从0开始,那么后退的时候撞到0会无法判断是第一次撞到应该返回Some还是后续撞到应当返回None.
        之后这里的step()是核心和重中之重,主要功能是在,有两个常见流派(实际上是基本等价的,但会有些区别)
        
        一类是先读取后移动(旧版),一个是先移动后读取(目前),没有本质区别,只看不同维护者/开发者的习惯,但需要注意,二者在获取和设置offset的行为上有诸多不同,需要注意细节
        
        此外,前者最好需要配合一个get\_current()获取当前的目录项函数使用
        
        后者需要在赋值的时候手工减去或者加上一个偏移量,且其方向设置必须在设置偏移量之前,否则会错;同时,不建议用get\_current(),而是应该用next()获取一次就存下来
        
                pub fn step(&mut self) -> Option<FATDirEnt> {
                    let mut dir_ent: FATDirEnt = FATDirEnt::empty();
                    if self.forward {
                        // if offset is None => 0
                        // if offset is non-negative => offset + STEP_SIZE
                        let offset = self.offset.map(|offset| offset + STEP_SIZE).unwrap_or(0);
                        if offset >= self.file_size() {
                            return None;
                        }
                        self.inode
                            .read_at_block_cache(&mut self.lock, offset as usize, dir_ent.as_bytes_mut());
                        match self.mode {
                            DirIterMode::Enum | DirIterMode::Dirent => (),
                            _ => {
                                // if directory entry is "last and unused", next is unavailable
                                if dir_ent.last_and_unused() {
                                    return None;
                                }
                            }
                        }
                        self.offset = Some(offset);
                    } else {
                        if self.offset.is_none() {
                            return None;
                        }
                        if self.offset.unwrap() == 0 {
                            self.offset = None;
                            return None;
                        }
                        self.offset = self.offset.map(|offset| offset - STEP_SIZE);
                        self.inode.read_at_block_cache(
                            &mut self.lock,
                            self.offset.unwrap() as usize,
                            dir_ent.as_bytes_mut(),
                        );
                    }
                    // println!("offset {:?}, unused: {:?}, {:?}", self.offset, dir_ent.unused(), dir_ent);
                    Some(dir_ent)
                }
            }
        
        这里到了检查和next部分,使用match判断不同的模式下目录项合法性相对更简洁
        
            impl<T: CacheManager, F: CacheManager> Iterator for DirIter<'_, T, F> {
                type Item = FATDirEnt;
                fn next(&mut self) -> Option<Self::Item> {
                    while let Some(dir_ent) = self.step() {
                        fn check_dir_ent_legality(mode: &DirIterMode, dir_ent: &FATDirEnt) -> bool {
                            match mode {
                                DirIterMode::Unused => dir_ent.unused_not_last(),
                                DirIterMode::UsedIter => !dir_ent.unused(),
                                DirIterMode::LongIter => !dir_ent.unused() && dir_ent.is_long(),
                                DirIterMode::ShortIter => !dir_ent.unused() && dir_ent.is_short(),
                                DirIterMode::Enum => true,
                                DirIterMode::Dirent => !dir_ent.unused() || dir_ent.last_and_unused(),
                            }
                        }
                        if check_dir_ent_legality(&self.mode, &dir_ent) {
                            return Some(dir_ent);
                        }
                    }
                    None
                }
            }

2.  第二层

    我们使用的双层iterator设计第二层如下:
    本身是由iterator套iterator组成,主要就是先读长目录项并校验序号(ord())(如果有),然后读短目录项,最后返回
    
        pub struct DirWalker<'a, T: CacheManager, F: CacheManager> {
            pub iter: DirIter<'a, T, F>,
        }
        
        impl<T: CacheManager, F: CacheManager> Iterator for DirWalker<'_, T, F> {
            type Item = (String, FATShortDirEnt);
            fn next(&mut self) -> Option<Self::Item> {
                let mut name = String::new();
                let mut should_be_ord = usize::MAX;
                while let Some(dir_ent) = self.iter.next() {
                    if dir_ent.is_long() {
                        if dir_ent.is_last_long_dir_ent() {
                            name = dir_ent.get_name() + &name;
                            should_be_ord = dir_ent.ord() - 1;
                        } else if dir_ent.ord() == should_be_ord {
                            name = dir_ent.get_name() + &name;
                            should_be_ord -= 1;
                        } else {
                            unreachable!()
                        }
                    } else if dir_ent.is_short() {
                        if name.is_empty() {
                            name = dir_ent.get_name();
                        }
                        return Some((name, dir_ent.get_short_ent().unwrap().clone()));
                    }
                }
                None
            }
        }
    
    事实上,曾经关于第二层还存在另一种设计,"通用ls()":
    
        pub fn ls(&self, cond: DirFilter) -> Vec<(String, FATDirShortEnt, u32)> {
            if !self.is_dir() {
                return Vec::new();
            }
            let mut v = Vec::with_capacity(30);
            let mut name = Vec::with_capacity(3);
            let lock = self.file_content.lock();
            let mut iter = self.dir_iter(lock, None, DirIterMode::UsedIter, FORWARD);
             let mut should_be_ord = usize::MAX;
            while let Some(dir_ent) = iter.next() {
                if dir_ent.is_long() {
                    if dir_ent.is_last_long_dir_ent() {
                        if !name.is_empty() {
                            //be warn future
                            panic!("why name isn't empty???");
                        }
                        name.push(dir_ent.get_name());
                        should_be_ord = dir_ent.ord() - 1;
                    } else if dir_ent.ord() == should_be_ord {
                        name.push(dir_ent.get_name());
                        should_be_ord -= 1;
                    } else {
                        unreachable!()
                    }
                } else if dir_ent.is_short() {
                    let filename: String;
                    if name.is_empty() {
                        filename = dir_ent.get_name();
                    } else {
                        name.reverse();
                        filename = name.concat();
                        name.clear();
                        //then match the name to see if it's correct.
                        //todo
                    };
                    if match cond {
                        DirFilter::DirOffset(_) | DirFilter::None => true,
                        DirFilter::Name(ref req_name) => *req_name == filename,
                        DirFilter::FstClus(inum) => {
                            inum as u32 == dir_ent.get_short_ent().unwrap().get_first_clus()
                        }
                    } {
                        v.push((
                            filename,
                            dir_ent.get_short_ent().unwrap().clone(),
                            iter.get_offset().unwrap(),
                        ));
                        if !cond.is_none() {
                            break;
                        }
                    }
                }
            }
            return v;
        }
    
    实质上就是相当于对上述程序直接展开,配上固定的(虽然灵活的一些也能用,比如传闭包)判定函数
    
    但你会发现由于iterator一方面可扩展性不强(iterator本身有自定义的函数和各种原生函数加持),另一方面代码更简洁(因为可以将判断疏散到find中,从而疏散其非iteration功能)且不用手动实现闭包判断
    
    事实上,如果这样性能本身的成本仰仗iterator本身的编译器优化,在大量多数量ls()的情况下会产生一定的额外开销,但考虑到我们唯一关于iterator测速的就是lmbench的fstat,这里的单次开销远远小于通用ls(),还是比较合适的
    
    此外,注意这里的name加法的用法比数组要相对更简洁,但性能可能差异不大;同时,新版没有了v申请空间的固定开销所以常数更小
    
    此外,之前的ls()语义有offset,如有必要,这里可以通过从内部的iterator获取得到


<a id="org80e5b5c"></a>

## Short Name generation

本来这个内容应当被归入Layout或者改名/创建文件夹,但考虑到他的难度,计划单列出一节.
这个部分参考资料不多,部分常见的实现是不完整的,某些的生成的尾部数字扩展恒为1,且没有判断重复的能力;哈佛的pdf直接转发微软的技术规格fatgen103.pdf,后者中的资料语焉不详,夏新手机厂中文版的翻译版和英文只是简单的翻译关系.本文献无意重复相关的资料和结果,但是我们可以使用Linux作为参考代码,分析相关内容.本文献的这部分为笔者有关算法的理解和分析
<https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/fs/fat/namei_vfat.c>
通过分析函数vfat\_create\_shortname,我们可以得出其具体的情况(该部分的注释由于不属于完整的代码,我们将其放在单独的"Linux下的短目录名生成"文档中),

我们的代码可以在easy-fs的中找到


<a id="orgf38667f"></a>

### 需求分析

首先考虑到我们的操作系统暂时不支持,可以只考虑英文的情况因此其代码的UTF8转换可以简化
当前的系统多数使用NT版本而非95的,可以少一些分支.


<a id="orgf309043"></a>

### 基本名和扩展名的生成

    pub fn gen_short_name_prefix(mut s: String) -> String {
        s.remove_matches(' ');//移除空格
        s.make_ascii_uppercase();//转为大写
        let split_res = s.rsplit_once('.');//截断出最右边的第一个'.',得到扩展名
        let (base, ext) = if split_res.is_some() {
            split_res.unwrap()//有扩展名就展开
        } else {
            (&s[..], "")//没有就在扩展名上填空串
        };
        //按照Windows做法将只有扩展名没有基本名的文件名的基本名作为扩展名
        let (base, ext) = if base.len() == 0 && ext.len() != 0 {
            (ext, base)
        } else {
            (base, ext)
        };
        let base = {
            let mut i = 0;
            while i != base.len() && base[i..].starts_with('.') {
                i += 1;
            }
            if i == base.len() {
                base
            } else {
                &base[i..]
            }
        };
        //删去头部多余的'.'
        let base = base.trim_start_matches('.');
        //最多留下8位的基本名,多了就截断,之后填空格
        let base = if ext.len() != 0 || base.len() != 0 {
            format!("{: <8}", base.split_at(8.min(base.len())).0)
        } else {
            "".to_string()
        };
        //最多留下3位的扩展名,多了就截断,之后填空格
        let ext = if ext.len() != 0 || base.len() != 0 {
            format!("{: <3}", ext.split_at(3.min(ext.len())).0)
        } else {
            "".to_string()
        };
        //如果既没有扩展名也没有基础名,则是非法名称,会返回空串
        [base, ext].concat()
    }


<a id="org19ca56e"></a>

### 一些细节

另外,数字结尾的生成算法过于复杂,很多,但理论上性能损失很大,我们建议尝试几个常见的数字之后就开始多次随机


<a id="org70a6008"></a>

# 目录树/文件打开表

文件系统中如果要保证文件的结构Inode一致性,最好是保证其唯一,使用Arc指针进行访问,

但必须要有一个储存的结构,从而查重并适当获取Arc


<a id="org88e7a27"></a>

## 层次

文件打开表的层次有两种,一种是在fat内部则是下文的"文件打开表部分"提供的写法(废案),另一种是在os::fs中,这里我们最终选用的是os::fs中的目录树结构,但easy-fs层文件打开表的建立方式我们也同样保留供参考


<a id="orge4f7afd"></a>

## 目录树

一种方式是建立目录树,虽然我们的实现中这部分是放到了os::fs中实现,但我还是简要概述一下这部分的实现方式

用一个lazy\_static的树形结构,用某种标志作key, 存下一级目录的集合的可变指针结构

注意,delete的操作在执行之前需要等待其Arc的strong\_count为0

另外注意,FAT32中每个非根目录文件夹都有"."和".."两个文件夹目录项,可能会导致问题.在查重的时候需要检查


<a id="org2cb661e"></a>

## 文件打开表

另一种方式是建立全局的文件打开表


<a id="org146faa7"></a>

### 最初的尝试

按照之前的实现,如果我们保留泛型,那么文件打开表的处理会非常复杂.特别是由于没有文件打开表,我们无法通过Inode的实例获取Arc<Inode>.
这时候很多人会考虑使用lazy\_static, 很遗憾,虽然这是Rust下static的常见方法,但这回可能是不行的.
因为Inode是Inode<T,F>, 而lazy\_static知道类型才能实例化,而对于EasyFS, 此时的类型实际上是Arc<Mutex<BTreeMap<usize,Arc<Inode<T,F>>>>>, 可以看到其中有两个未知的类型.
如果尝试像C++一样,使用per-type static又如何呢?很遗憾,还是不行,因为Rust不允许将struct的一个域声明为static. 但这可能给了我们一个思路: 如果使用某个static的函数/方法,或许可以绕过这个问题


<a id="org4d3d2fb"></a>

### 绕过失败

这就得提到笔者大二学编译原理的一个故事了,当时我们的老师,一位擅长给学生高分的老师,
要求大家不允许使用全局变量,只能传参数,本意是让大家多少用全局变量,
但我这个人就是不听话,所以在C语言下开发出了这样的奇怪打法

    int global_variable_a(bool is_set_mode, int val){
        static int a = 0;
        if (is_set_mode) a=val;
        else return a;
    }
    void set_a(int val) {global_variable_a(1,val);}
    int get_a(){return global_variable_a(0,0);}

强行绕过了这个限制,老师最终也没有说什么.
有没有一种可能,老师实际上是Rust爱好者呢?不过不要紧,这就试一下

    impl<T: CacheManager, F: CacheManager> Inode<T, F> {
        fn open_li(){
              static map = BTreeMap<usize, Arc<Self>>::new();
        }
    }

很遗憾,即使是这个失败的版本,Rust-analyzer就直接报错了,因为Rust不允许方法中的static有泛型.恐怕只能另辟蹊径


<a id="orga95cdab"></a>

### 最后版本

在队友的建议下,用unsafe裸指针解引用的方式解决编译通过的问题.大致思路如下:

    pub fn open_tab(file: OpenTabCmd<T, F>) -> Option<Arc<Self>> {
        static mut ORG_LI: usize = 0;
        unsafe {
            if ORG_LI == 0 {
                *(ORG_LI as *mut Mutex<alloc::collections::
                             BTreeMap<u64, Arc<Self>>>) =
                    Mutex::new(alloc::collections::BTreeMap::new());
            }
    //...
        None
    }

首先,函数是Inode<T,F>的关联函数,在其中使用静态变量ORG\_LI保持文件打开表.
但是要注意,Rust中由于默认不允许关联函数使用泛型,所有的Inode<T,F>会共用一个open\_tab(). 所以如果严谨的方法是利用非静态变量的TypeId(请自行搜索)建立一个BTreeMap然后对各个类型一一对应,但比较麻烦且对我们的两种运行环境没有意义(系统和FUSE模拟),所以完全可以忽略.
另外,初始化的时候,如为多核,理论上Rust不使用原子操作保证其一致性,所以需要先完成初始化再打开第二个核,否则会出现竞争.
然后是之后的命令设计.我们注意到,为了方便处理,此函数使用了OpenTabCmd<T,F>进行文件打开表的命令,其定义为:

    pub enum OpenTabCmd<T: CacheManager, F: CacheManager> {
        InsertFile(Arc<Inode<T, F>>),
        GetFileByInode(u64),
        DropFileByInode(u64),
        AddInode(u64, u64),
    }

事实上,应当在需要的地方部署相关的函数,如delete, create等的开头,以避免重复使用
但是这种方法我们并没有使用,如果你有需要,可以尝试实现.


<a id="org5042b38"></a>

# 后续的思路

事实上,由上述分析,EasyFS的代码中的文件系统可以继续抽象,将其Inode继续抽象出来,然后使用泛型;此外,在锁的部分也许可以更加抽象;此外,对read/write应当继续封装,从而减小对外部的锁的暴露

此外,对于某些特别专一的应用场景,会不会其实EasyFileSystem是不必要的?毕竟其在初始化出root\_inode和fat之后几乎所有的作用就是计算各种的block\_id了, 但其实这也许可以用别的方法,如存一些常数在Inode中或者static的方式处理

