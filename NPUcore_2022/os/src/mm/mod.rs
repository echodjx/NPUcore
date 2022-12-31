mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;
#[cfg(feature = "zram")]
mod zram;

use core::arch::asm;

use address::VPNRange;
pub use address::{PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum};
pub use frame_allocator::{
    frame_alloc, frame_alloc_uninit, frame_dealloc, frame_reserve, unallocated_frames, FrameTracker,
};
pub use memory_set::{
    kernel_token, remap_test, Frame, MapFlags, MapPermission, MemoryError, MemorySet, KERNEL_SPACE,
};
use page_table::PTEFlags;
pub use page_table::{
    copy_from_user, copy_from_user_array, copy_to_user, copy_to_user_array, copy_to_user_string,
    get_from_user, try_get_from_user, translated_byte_buffer,
    translated_byte_buffer_append_to_existing_vec, translated_ref, translated_refmut,
    translated_str, PageTable, PageTableEntry, UserBuffer, UserBufferIterator,
};

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.lock().activate();
}
#[inline(always)]
pub fn tlb_invalidate() {
    unsafe {
        asm!("sfence.vma");
    }
}

#[macro_export]
/// Convert user pointer trg to `Some(*trg)` or `None` if null.
macro_rules! move_ptr_to_opt {
    ($trg:ident) => {
        if $trg != null() {
            let t = *translated_ref(current_user_token(), $trg);
            Some(t)
        } else {
            None
        }
    };
    ($token:ident,$trg:ident) => {
        if $trg != null() {
            let t = *translated_ref($token, $trg);
            Some(t)
        } else {
            None
        }
    };
}

#[macro_export]
/// Convert user pointer `trg:*const T` to `Some(trg as & T)` or `None` if null.
macro_rules! ptr_to_opt_ref {
    ($trg:ident) => {
        if $trg != null() {
            Some(translated_ref(current_user_token(), $trg))
        } else {
            None
        }
    };
    ($token:ident,$trg:ident) => {
        if $trg != null() {
            Some(translated_ref($token, $trg))
        } else {
            None
        }
    };
}
