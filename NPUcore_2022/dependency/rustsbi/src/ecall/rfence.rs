use super::SbiRet;
use crate::hart_mask::HartMask;
use crate::rfence;

const FUNCTION_RFENCE_REMOTE_FENCE_I: usize = 0x0;
const FUNCTION_RFENCE_REMOTE_SFENCE_VMA: usize = 0x1;
const FUNCTION_RFENCE_REMOTE_SFENCE_VMA_ASID: usize = 0x2;
const FUNCTION_RFENCE_REMOTE_HFENCE_GVMA_VMID: usize = 0x3;
const FUNCTION_RFENCE_REMOTE_HFENCE_GVMA: usize = 0x4;
const FUNCTION_RFENCE_REMOTE_HFENCE_VVMA_ASID: usize = 0x5;
const FUNCTION_RFENCE_REMOTE_HFENCE_VVMA: usize = 0x6;

#[inline]
pub fn handle_ecall_rfence(
    function: usize,
    param0: usize,
    param1: usize,
    param2: usize,
    param3: usize,
    param4: usize,
) -> SbiRet {
    match function {
        FUNCTION_RFENCE_REMOTE_FENCE_I => remote_fence_i(param0, param1),
        FUNCTION_RFENCE_REMOTE_SFENCE_VMA => remote_sfence_vma(param0, param1, param2, param3),
        FUNCTION_RFENCE_REMOTE_SFENCE_VMA_ASID => {
            remote_sfence_vma_asid(param0, param1, param2, param3, param4)
        }
        FUNCTION_RFENCE_REMOTE_HFENCE_GVMA_VMID => {
            remote_hfence_gvma_vmid(param0, param1, param2, param3, param4)
        }
        FUNCTION_RFENCE_REMOTE_HFENCE_GVMA => remote_hfence_gvma(param0, param1, param2, param3),
        FUNCTION_RFENCE_REMOTE_HFENCE_VVMA_ASID => {
            remote_hfence_vvma_asid(param0, param1, param2, param3, param4)
        }
        FUNCTION_RFENCE_REMOTE_HFENCE_VVMA => remote_hfence_vvma(param0, param1, param2, param3),
        _ => SbiRet::not_supported(),
    }
}

#[inline]
fn remote_fence_i(hart_mask: usize, hart_mask_base: usize) -> SbiRet {
    let hart_mask = HartMask::from_mask_base(hart_mask, hart_mask_base);
    rfence::remote_fence_i(hart_mask)
}

#[inline]
fn remote_sfence_vma(
    hart_mask: usize,
    hart_mask_base: usize,
    start_addr: usize,
    size: usize,
) -> SbiRet {
    let hart_mask = HartMask::from_mask_base(hart_mask, hart_mask_base);
    rfence::remote_sfence_vma(hart_mask, start_addr, size)
}

#[inline]
fn remote_sfence_vma_asid(
    hart_mask: usize,
    hart_mask_base: usize,
    start_addr: usize,
    size: usize,
    asid: usize,
) -> SbiRet {
    let hart_mask = HartMask::from_mask_base(hart_mask, hart_mask_base);
    rfence::remote_sfence_vma_asid(hart_mask, start_addr, size, asid)
}

#[inline]
fn remote_hfence_gvma_vmid(
    hart_mask: usize,
    hart_mask_base: usize,
    start_addr: usize,
    size: usize,
    vmid: usize,
) -> SbiRet {
    let hart_mask = HartMask::from_mask_base(hart_mask, hart_mask_base);
    rfence::remote_hfence_gvma_vmid(hart_mask, start_addr, size, vmid)
}

#[inline]
fn remote_hfence_gvma(
    hart_mask: usize,
    hart_mask_base: usize,
    start_addr: usize,
    size: usize,
) -> SbiRet {
    let hart_mask = HartMask::from_mask_base(hart_mask, hart_mask_base);
    rfence::remote_hfence_gvma(hart_mask, start_addr, size)
}

#[inline]
fn remote_hfence_vvma_asid(
    hart_mask: usize,
    hart_mask_base: usize,
    start_addr: usize,
    size: usize,
    asid: usize,
) -> SbiRet {
    let hart_mask = HartMask::from_mask_base(hart_mask, hart_mask_base);
    rfence::remote_hfence_vvma_asid(hart_mask, start_addr, size, asid)
}

#[inline]
fn remote_hfence_vvma(
    hart_mask: usize,
    hart_mask_base: usize,
    start_addr: usize,
    size: usize,
) -> SbiRet {
    let hart_mask = HartMask::from_mask_base(hart_mask, hart_mask_base);
    rfence::remote_hfence_vvma(hart_mask, start_addr, size)
}
