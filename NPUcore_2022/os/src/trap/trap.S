.attribute arch, "rv64gc"
FP_START = 32
.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
.macro SAVE_FP n, m
    fsd f\n, \m*8(sp)
.endm
.macro LOAD_FP n, m
    fld f\n, \m*8(sp)
.endm
    .section .text.trampoline
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    csrrw sp, sscratch, sp
    # now sp->*TrapContext in user space, sscratch->user stack
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    .set n, 3
    .rept 29
        SAVE_GP %n
        .set n, n+1
    .endr
    .set n, 0
    .set m, FP_START
    .rept 32
        SAVE_FP %n, %m
        .set n, n+1
        .set m, m+1
    .endr
    # we can use t0/t1/t2 freely, because they have been saved in TrapContext
    csrr t0, fcsr
    sd t0, 64*8(sp)
    # save other general purpose registers
    sd a0, 65*8(sp)
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 66*8(sp)
    sd t1, 0(sp)
    # read user stack from sscratch and save it in TrapContext
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # load kernel_satp into t0
    ld t0, 67*8(sp)
    # load trap_handler into t1
    ld t1, 68*8(sp)
    # move to kernel_sp
    ld sp, 69*8(sp)
    # switch to kernel space
    csrw satp, t0
    sfence.vma
    # jump to trap_handler
    jr t1

__restore:
    # a0: *TrapContext in user space(Constant); a1: user space token
    # switch to user space
    csrw satp, a1
    sfence.vma
    csrw sscratch, a0
    mv sp, a0
    # now sp points to TrapContext in user space, start restoring based on it
    # restore sstatus/sepc
    ld t0, 64*8(sp)
    csrw fcsr, t0
    ld t0, 66*8(sp)
    ld t1, 0(sp)
    csrw sstatus, t0
    csrw sepc, t1
    # restore general purpose registers except x0/sp
    ld x1, 1*8(sp)
    .set n, 3
    .rept 29
        LOAD_GP %n
        .set n, n+1
    .endr
    .set n, 0
    .set m, FP_START
    .rept 32
        LOAD_FP %n, %m
        .set n, n+1
        .set m, m+1
    .endr
    # back to user stack
    ld sp, 2*8(sp)
    sret

    .section .text.signaltrampoline
    .globl __call_sigreturn
    .align 2
__call_sigreturn:
    # ecall sys_sigreturn
    addi	a7, zero, 139
    ecall