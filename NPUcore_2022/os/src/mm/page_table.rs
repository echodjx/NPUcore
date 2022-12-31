use super::memory_set::check_page_fault;
use super::{
    frame_alloc, tlb_invalidate, FrameTracker, MapPermission, PhysAddr, PhysPageNum, StepByOne,
    VirtAddr, VirtPageNum,
};
use _core::mem::MaybeUninit;
use _core::ops::{Index, IndexMut};
use alloc::vec::Vec;
use alloc::{string::String, sync::Arc};
use bitflags::*;
bitflags! {
    /// Page Table Entry flags
    pub struct PTEFlags: u8 {
    /// Valid Bit
        const V = 1 << 0;
    /// Readable Bit
        const R = 1 << 1;
    /// Writable Bit
        const W = 1 << 2;
    /// Executable Bit
        const X = 1 << 3;
    /// User Space Bit, true if it can be accessed from user space.
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
    /// Dirty Bit, true if it is modified.
        const D = 1 << 7;
    }
}

/// Page Table Entry
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    const PPN_MASK: usize = ((1usize << 44) - 1) << 10;
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: ppn.0 << 10 | flags.bits as usize,
        }
    }
    pub fn empty() -> Self {
        PageTableEntry { bits: 0 }
    }
    pub fn ppn(&self) -> PhysPageNum {
        ((self.bits & Self::PPN_MASK) >> 10).into()
    }
    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }
    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }
    pub fn is_dirty(&self) -> bool {
        (self.flags() & PTEFlags::D) != PTEFlags::empty()
    }
    pub fn readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }
    pub fn writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }
    pub fn executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }
    pub fn clear_access(&mut self) {
        self.bits &= !(PTEFlags::A.bits() as usize);
    }
    pub fn clear_dirty(&mut self) {
        self.bits &= !(PTEFlags::D.bits() as usize);
    }
    pub fn revoke_read(&mut self) {
        self.bits &= !(PTEFlags::R.bits() as usize);
    }
    pub fn revoke_write(&mut self) {
        self.bits &= !(PTEFlags::W.bits() as usize);
    }
    pub fn revoke_execute(&mut self) {
        self.bits &= !(PTEFlags::X.bits() as usize);
    }
    pub fn set_permission(&mut self, flags: MapPermission) {
        self.bits = (self.bits & 0xffff_ffff_ffff_ffe1) | (flags.bits() as usize)
    }
    pub fn set_ppn(&mut self, ppn: PhysPageNum) {
        self.bits = (self.bits & !Self::PPN_MASK) | ((ppn.0 << 10) & Self::PPN_MASK)
    }
}

pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<Arc<FrameTracker>>,
}

/// Assume that it won't encounter oom when creating/mapping.
impl PageTable {
    pub fn new() -> Self {
        let frame = frame_alloc().unwrap();
        PageTable {
            root_ppn: frame.ppn,
            frames: {
                let mut vec = Vec::with_capacity(256);
                vec.push(frame);
                vec
            },
        }
    }
    /// Create an empty page table from `satp`
    /// # Argument
    /// * `satp` Supervisor Address Translation & Protection reg. that points to the physical page containing the root page.
    pub fn from_token(satp: usize) -> Self {
        Self {
            root_ppn: PhysPageNum::from(satp & ((1usize << 44) - 1)),
            frames: Vec::new(),
        }
    }
    /// Predicate for the valid bit.
    pub fn is_mapped(&mut self, vpn: VirtPageNum) -> bool {
        if let Some(i) = self.find_pte(vpn) {
            if i.is_valid() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    /// Find the page in the page table, creating the page on the way if not exists.
    /// Note: It does NOT create the terminal node. The caller must verify its validity and create according to his own needs.
    fn find_pte_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for i in 0..3 {
            let pte = &mut ppn.get_pte_array()[idxs[i]];
            if i == 2 {
                // this condition is used to make sure the
                //returning predication is put before validity to quit before creating the terminal page entry.
                result = Some(pte);
                break;
            }
            if !pte.is_valid() {
                let frame = frame_alloc().unwrap();
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V);
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        result
    }
    /// Find the page table entry denoted by vpn, returning Some(&_) if found or None if not.
    fn find_pte(&self, vpn: VirtPageNum) -> Option<&PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&PageTableEntry> = None;
        for i in 0..3 {
            let pte = &ppn.get_pte_array()[idxs[i]];
            if !pte.is_valid() {
                return None;
            }
            if i == 2 {
                result = Some(pte);
                break;
            }
            ppn = pte.ppn();
        }
        result
    }
    /// Find and return reference the page table entry denoted by `vpn`, `None` if not found.
    fn find_pte_refmut(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for i in 0..3 {
            let pte = &mut ppn.get_pte_array()[idxs[i]];
            if !pte.is_valid() {
                return None;
            }
            if i == 2 {
                result = Some(pte);
                break;
            }
            ppn = pte.ppn();
        }
        result
    }
    #[allow(unused)]
    /// Map the `vpn` to `ppn` with the `flags`.
    /// # Note
    /// Allocation should be done elsewhere.
    /// # Exceptions
    /// Panics if the `vpn` is mapped.
    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte_create(vpn).unwrap();
        assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }
    #[allow(unused)]
    /// Unmap the `vpn` to `ppn` with the `flags`.
    /// # Exceptions
    /// Panics if the `vpn` is NOT mapped (invalid).
    pub fn unmap(&mut self, vpn: VirtPageNum) {
        //tlb_invalidate();
        let pte = self.find_pte_refmut(vpn).unwrap(); // was `self.find_creat_pte(vpn).unwrap()`;
        assert!(pte.is_valid(), "vpn {:?} is invalid before unmapping", vpn);
        *pte = PageTableEntry::empty();
    }
    /// Translate the `vpn` into its corresponding `Some(PageTableEntry)` if exists
    /// `None` is returned if nothing is found.
    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        // This is not the same map as we defined just now...
        // It is the map for func. programming.
        self.find_pte(vpn).map(|pte| pte.clone())
    }
    /// Translate the virtual address into its corresponding `PhysAddr` if mapped in current page table.
    /// `None` is returned if nothing is found.
    pub fn translate_va(&self, va: VirtAddr) -> Option<PhysAddr> {
        self.find_pte(va.clone().floor()).map(|pte| {
            let aligned_pa: PhysAddr = pte.ppn().into();
            let offset = va.page_offset();
            let aligned_pa_usize: usize = aligned_pa.into();
            (aligned_pa_usize + offset).into()
        })
    }
    pub fn translate_refmut(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        self.find_pte_refmut(vpn)
    }
    /// Return the physical token to current page.
    pub fn token(&self) -> usize {
        8usize << 60 | self.root_ppn.0
    }
    pub fn revoke_read(&mut self, vpn: VirtPageNum) -> Result<(), ()> {
        if let Some(pte) = self.find_pte_refmut(vpn) {
            pte.revoke_read();
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn revoke_write(&mut self, vpn: VirtPageNum) -> Result<(), ()> {
        if let Some(pte) = self.find_pte_refmut(vpn) {
            pte.revoke_write();
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn revoke_execute(&mut self, vpn: VirtPageNum) -> Result<(), ()> {
        if let Some(pte) = self.find_pte_refmut(vpn) {
            pte.revoke_execute();
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn set_ppn(&mut self, vpn: VirtPageNum, ppn: PhysPageNum) -> Result<(), ()> {
        if let Some(pte) = self.find_pte_refmut(vpn) {
            pte.set_ppn(ppn);
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn set_pte_flags(&mut self, vpn: VirtPageNum, flags: MapPermission) -> Result<(), ()> {
        //tlb_invalidate();
        if let Some(pte) = self.find_pte_refmut(vpn) {
            pte.set_permission(flags);
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn clear_access_bit(&mut self, vpn: VirtPageNum) -> Result<(), ()> {
        tlb_invalidate();
        if let Some(pte) = self.find_pte_refmut(vpn) {
            pte.clear_access();
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn clear_dirty_bit(&mut self, vpn: VirtPageNum) -> Result<(), ()> {
        tlb_invalidate();
        if let Some(pte) = self.find_pte_refmut(vpn) {
            pte.clear_dirty();
            Ok(())
        } else {
            Err(())
        }
    }
}

/// if `existing_vec == None`, a empty `Vec` will be created.
pub fn translated_byte_buffer_append_to_existing_vec(
    existing_vec: &mut Vec<&'static mut [u8]>,
    token: usize,
    ptr: *const u8,
    len: usize,
) -> Result<(), isize> {
    let page_table = PageTable::from_token(token);
    let mut start = ptr as usize;
    let end = start + len;
    while start < end {
        let start_va = VirtAddr::from(start);
        let mut vpn = start_va.floor();
        let ppn = match page_table.translate(vpn) {
            Some(pte) => pte.ppn(),
            None => {
                let pa = check_page_fault(vpn.into())?;
                pa.floor()
            }
        };
        vpn.step();
        let mut end_va: VirtAddr = vpn.into();
        end_va = end_va.min(VirtAddr::from(end));
        if end_va.page_offset() == 0 {
            existing_vec.push(&mut ppn.get_bytes_array()[start_va.page_offset()..]);
        } else {
            existing_vec
                .push(&mut ppn.get_bytes_array()[start_va.page_offset()..end_va.page_offset()]);
        }
        start = end_va.into();
    }
    Ok(())
}

pub fn translated_byte_buffer(
    token: usize,
    ptr: *const u8,
    len: usize,
) -> Result<Vec<&'static mut [u8]>, isize> {
    let page_table = PageTable::from_token(token);
    let mut start = ptr as usize;
    let end = start + len;
    let mut v = Vec::with_capacity(32);
    while start < end {
        let start_va = VirtAddr::from(start);
        let mut vpn = start_va.floor();
        let ppn = match page_table.translate(vpn) {
            Some(pte) => pte.ppn(),
            None => {
                let pa = check_page_fault(vpn.into())?;
                pa.floor()
            }
        };
        vpn.step();
        let mut end_va: VirtAddr = vpn.into();
        end_va = end_va.min(VirtAddr::from(end));
        if end_va.page_offset() == 0 {
            v.push(&mut ppn.get_bytes_array()[start_va.page_offset()..]);
        } else {
            v.push(&mut ppn.get_bytes_array()[start_va.page_offset()..end_va.page_offset()]);
        }
        start = end_va.into();
    }
    Ok(v)
}

/// Load a string from other address spaces into kernel space without an end `\0`.
pub fn translated_str(token: usize, ptr: *const u8) -> Result<String, isize> {
    let page_table = PageTable::from_token(token);
    let mut string = String::new();
    let mut cur = ptr as usize;
    loop {
        let ch: u8 = *({
            let va = VirtAddr::from(cur);
            let pa = match page_table.translate_va(va) {
                Some(pa) => pa,
                None => check_page_fault(va)?,
            };
            pa.get_mut()
        });
        if ch == 0 {
            break;
        }
        string.push(ch as char);
        cur += 1;
    }
    Ok(string)
}

/// Translate the user space pointer `ptr` into a reference in user space through page table `token`
pub fn translated_ref<T>(token: usize, ptr: *const T) -> Result<&'static T, isize> {
    let page_table = PageTable::from_token(token);
    let va = VirtAddr::from(ptr as usize);
    let pa = match page_table.translate_va(va) {
        Some(pa) => pa,
        None => check_page_fault(va)?,
    };
    Ok(pa.get_ref())
}

/// Translate the user space pointer `ptr` into a mutable reference in user space through page table `token`
/// # Implementation Information
/// * Get the pagetable from token
pub fn translated_refmut<T>(token: usize, ptr: *mut T) -> Result<&'static mut T, isize> {
    let page_table = PageTable::from_token(token);
    let va = VirtAddr::from(ptr as usize);
    let pa = match page_table.translate_va(va) {
        Some(pa) => pa,
        None => check_page_fault(va)?,
    };
    Ok(pa.get_mut())
}
/// A buffer in user space. Kernel space code may use this struct to copy to/ read from user space.
/// This struct is meaningless in case that the kernel page is present in the user side MemorySet.
pub struct UserBuffer {
    /// The segmented array, or, a "vector of vectors".
    /// # Design Information
    /// In Rust, reference lifetime is a must for this template.
    /// The lifetime of buffers is `static` because the buffer 'USES A' instead of 'HAS A'
    pub buffers: Vec<&'static mut [u8]>,
    /// The total size of the Userbuffer.
    pub len: usize,
}

impl UserBuffer {
    pub fn new(buffers: Vec<&'static mut [u8]>) -> Self {
        Self {
            len: buffers.iter().map(|buffer| buffer.len()).sum(),
            buffers,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn read(&self, dst: &mut [u8]) -> usize {
        let mut start = 0;
        let dst_len = dst.len();
        for buffer in self.buffers.iter() {
            let end = start + buffer.len();
            if end > dst_len {
                dst[start..].copy_from_slice(&buffer[..dst_len - start]);
                return dst_len;
            } else {
                dst[start..end].copy_from_slice(buffer);
            }
            start = end;
        }
        self.len
    }

    pub fn write(&mut self, src: &[u8]) -> usize {
        let mut start = 0;
        let src_len = src.len();
        for buffer in self.buffers.iter_mut() {
            let end = start + buffer.len();
            if end > src_len {
                buffer[..src_len - start].copy_from_slice(&src[start..]);
                return src_len;
            } else {
                buffer.copy_from_slice(&src[start..end]);
            }
            start = end;
        }
        self.len
    }

    /// Write to `self` starting at `offset`, and return written bytes.
    /// This funtion will try to write as much as possible data
    /// in the limit of `self.len()` and `src.len()`.
    /// It guarantees that won't read/write out of bound.
    pub fn write_at(&mut self, offset: usize, src: &[u8]) -> usize {
        if offset >= self.len {
            return 0;
        }
        let mut write_bytes = 0usize;
        let mut dst_start = 0usize;
        for buffer in self.buffers.iter_mut() {
            let dst_end = dst_start + buffer.len();
            //we can image mapping 'src' categories to 'dst' categories
            //then we just need to intersect two intervals to get the corresponding interval
            let copy_dst_start = dst_start.max(offset);
            //we may worry about overflow,
            //but we can guarantee that offset(we have checked before) and
            //src.len()(because of limited memory) won't be too large
            let copy_dst_end = dst_end.min(src.len() + offset);
            if copy_dst_start >= copy_dst_end {
                dst_start = dst_end; //don't forget to update dst_start
                continue;
            }
            //mapping 'dst' categories to 'src' categories
            let copy_src_start = copy_dst_start - offset;
            let copy_src_end = copy_dst_end - offset;
            //mapping 'dst' categories to 'buffer' categories
            let copy_buffer_start = copy_dst_start - dst_start;
            let copy_buffer_end = copy_dst_end - dst_start;
            buffer[copy_buffer_start..copy_buffer_end]
                .copy_from_slice(&src[copy_src_start..copy_src_end]);
            write_bytes += copy_dst_end - copy_dst_start;
            dst_start = dst_end; //don't forget to update dst_start
        }
        write_bytes
    }

    pub fn clear(&mut self) {
        self.buffers.iter_mut().for_each(|buffer| {
            buffer.fill(0);
        })
    }
}

//There may be better implementations here to cover more types
impl Index<usize> for UserBuffer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        assert!((index as usize) < self.len);
        let mut left = index;
        for buffer in &self.buffers {
            if (left as usize) < buffer.len() {
                return &buffer[left];
            } else {
                left -= buffer.len();
            }
        }
        unreachable!();
    }
}
impl IndexMut<usize> for UserBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!((index as usize) < self.len);
        let mut left = index;
        for buffer in &mut self.buffers {
            if (left as usize) < buffer.len() {
                return &mut buffer[left];
            } else {
                left -= buffer.len();
            }
        }
        unreachable!();
    }
}

impl IntoIterator for UserBuffer {
    type Item = *mut u8;
    type IntoIter = UserBufferIterator;
    fn into_iter(self) -> Self::IntoIter {
        UserBufferIterator {
            buffers: self.buffers,
            current_buffer: 0,
            current_idx: 0,
        }
    }
}

/// Iterator to a UserBuffer returning u8
pub struct UserBufferIterator {
    buffers: Vec<&'static mut [u8]>,
    current_buffer: usize,
    current_idx: usize,
}

impl Iterator for UserBufferIterator {
    type Item = *mut u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_buffer >= self.buffers.len() {
            None
        } else {
            let r = &mut self.buffers[self.current_buffer][self.current_idx] as *mut _;
            if self.current_idx + 1 == self.buffers[self.current_buffer].len() {
                self.current_idx = 0;
                self.current_buffer += 1;
            } else {
                self.current_idx += 1;
            }
            Some(r)
        }
    }
}

/// Copy `*src: T` to kernel space.
/// `src` is a pointer in user space, `dst` is a pointer in kernel space.
pub fn copy_from_user<T: 'static + Copy>(
    token: usize,
    src: *const T,
    dst: *mut T,
) -> Result<(), isize> {
    let size = core::mem::size_of::<T>();
    // if all data of `*src` is in the same page, read directly
    if VirtAddr::from(src as usize).floor() == VirtAddr::from(src as usize + size - 1).floor() {
        unsafe { _core::ptr::copy_nonoverlapping(translated_ref(token, src)?, dst, 1) };
    // or we should use UserBuffer to read across user space pages
    } else {
        UserBuffer::new(translated_byte_buffer(token, src as *const u8, size)?)
            .read(unsafe { core::slice::from_raw_parts_mut(dst as *mut u8, size) });
    }
    Ok(())
}

/// Copy array `*src: [T;len]` to kernel space.
/// `src` is a pointer in user space, `dst` is a pointer in kernel space.
pub fn copy_from_user_array<T: 'static + Copy>(
    token: usize,
    src: *const T,
    dst: *mut T,
    len: usize,
) -> Result<(), isize> {
    let size = core::mem::size_of::<T>() * len;
    // if all data of `*src` is in the same page, read directly
    if VirtAddr::from(src as usize).floor() == VirtAddr::from(src as usize + size - 1).floor() {
        let page_table = PageTable::from_token(token);
        let src_va = VirtAddr::from(src as usize);
        let src_pa = match page_table.translate_va(src_va) {
            Some(pa) => pa,
            None => {
                let pa = check_page_fault(src_va)?;
                pa
            }
        };
        unsafe {
            _core::ptr::copy_nonoverlapping(src_pa.0 as *const T, dst, len);
        }
    // or we should use UserBuffer to read across user space pages
    } else {
        UserBuffer::new(translated_byte_buffer(token, src as *const u8, size)?)
            .read(unsafe { core::slice::from_raw_parts_mut(dst as *mut u8, size) });
    }
    Ok(())
}

/// Copy `*src: T` to user space.
/// `src` is a pointer in kernel space, `dst` is a pointer in user space.
pub fn copy_to_user<T: 'static + Copy>(
    token: usize,
    src: *const T,
    dst: *mut T,
) -> Result<(), isize> {
    let size = core::mem::size_of::<T>();
    // A nice predicate. Well done!
    // Re: Thanks!
    if VirtAddr::from(dst as usize).floor() == VirtAddr::from(dst as usize + size - 1).floor() {
        unsafe { _core::ptr::copy_nonoverlapping(src, translated_refmut(token, dst)?, 1) };
    // use UserBuffer to write across user space pages
    } else {
        UserBuffer::new(translated_byte_buffer(token, dst as *mut u8, size)?)
            .write(unsafe { core::slice::from_raw_parts(src as *const u8, size) });
    }
    Ok(())
}

/// Copy `*src: T` to kernel space.
/// `src` is a pointer in user space, `dst` is a pointer in kernel space.
#[inline(always)]
pub fn get_from_user<T: 'static + Copy>(token: usize, src: *const T) -> Result<T, isize> {
    unsafe {
        let mut dst: T = MaybeUninit::uninit().assume_init();
        copy_from_user(token, src, &mut dst)?;
        return Ok(dst);
    }
}

#[inline(always)]
pub fn try_get_from_user<T: 'static + Copy>(
    token: usize,
    src: *const T,
) -> Result<Option<T>, isize> {
    if !src.is_null() {
        Ok(Some(get_from_user(token, src)?))
    } else {
        Ok(None)
    }
}

/// Copy array `*src: [T;len]` to user space.
/// `src` is a pointer in kernel space, `dst` is a pointer in user space.
pub fn copy_to_user_array<T: 'static + Copy>(
    token: usize,
    src: *const T,
    dst: *mut T,
    len: usize,
) -> Result<(), isize> {
    let size = core::mem::size_of::<T>() * len;
    // if all data of `*dst` is in the same page, write directly
    if VirtAddr::from(dst as usize).floor() == VirtAddr::from(dst as usize + size - 1).floor() {
        let page_table = PageTable::from_token(token);
        let dst_va = VirtAddr::from(dst as usize);
        let dst_pa = match page_table.translate_va(dst_va) {
            Some(pa) => pa,
            None => {
                let pa = check_page_fault(dst_va)?;
                pa
            }
        };
        unsafe {
            _core::ptr::copy_nonoverlapping(src, dst_pa.0 as *mut T, len);
        };
    // or we should use UserBuffer to write across user space pages
    } else {
        UserBuffer::new(translated_byte_buffer(token, dst as *mut u8, size)?)
            .write(unsafe { core::slice::from_raw_parts(src as *const u8, size) });
    }
    Ok(())
}

/// Automatically add `'\0'` in the end,
/// so total written length is `src.len() + 1` (with trailing `'\0'`).
/// # Warning
/// Caller should ensure `src` is not too large, or this function will write out of bound.
pub fn copy_to_user_string(token: usize, src: &str, dst: *mut u8) -> Result<(), isize> {
    let size = src.len();
    let page_table = PageTable::from_token(token);
    let dst_va = VirtAddr::from(dst as usize);
    let dst_pa = match page_table.translate_va(dst_va) {
        Some(pa) => pa,
        None => {
            let pa = check_page_fault(dst_va)?;
            pa
        }
    };
    let dst_ptr = dst_pa.0 as *mut u8;
    // if all data of `*dst` is in the same page, write directly
    if VirtAddr::from(dst as usize).floor() == VirtAddr::from(dst as usize + size).floor() {
        unsafe {
            _core::ptr::copy_nonoverlapping(src.as_ptr(), dst_ptr, size);
            dst_ptr.add(size).write(b'\0');
        }
    // or we should use UserBuffer to write across user space pages
    } else {
        UserBuffer::new(translated_byte_buffer(token, dst as *mut u8, size)?)
            .write(unsafe { core::slice::from_raw_parts(src.as_ptr(), size) });
        unsafe {
            dst_ptr.add(size).write(b'\0');
        }
    }
    Ok(())
}
