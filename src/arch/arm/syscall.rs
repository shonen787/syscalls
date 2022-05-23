// On arm, the following registers are used for args 1-6:
// arg1: %r0
// arg2: %r1
// arg3: %r2
// arg4: %r3
// arg5: %r4
// arg6: %r5
//
// %r7 is used for the syscall number.
//
// %r0 is reused for the syscall return value.
//
// No other registers are clobbered.
use core::arch::asm;

use super::syscalls::Sysno;

/// A syscall that takes 0 arguments.
#[inline]
pub unsafe fn syscall0(n: Sysno) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("r7") n as usize,
        lateout("r0") ret,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

/// A syscall that takes 1 argument.
#[inline]
pub unsafe fn syscall1(n: Sysno, arg1: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("r7") n as usize,
        inlateout("r0") arg1 => ret,
        options(nostack, preserves_flags)
    );
    ret
}

/// A syscall that takes 2 arguments.
#[inline]
pub unsafe fn syscall2(n: Sysno, arg1: usize, arg2: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("r7") n as usize,
        inlateout("r0") arg1 => ret,
        in("r1") arg2,
        options(nostack, preserves_flags)
    );
    ret
}

/// A syscall that takes 3 arguments.
#[inline]
pub unsafe fn syscall3(
    n: Sysno,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("r7") n as usize,
        inlateout("r0") arg1 => ret,
        in("r1") arg2,
        in("r2") arg3,
        options(nostack, preserves_flags)
    );
    ret
}

/// A syscall that takes 4 arguments.
#[inline]
pub unsafe fn syscall4(
    n: Sysno,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("r7") n as usize,
        inlateout("r0") arg1 => ret,
        in("r1") arg2,
        in("r2") arg3,
        in("r3") arg4,
        options(nostack, preserves_flags)
    );
    ret
}

/// A syscall that takes 5 arguments.
#[inline]
pub unsafe fn syscall5(
    n: Sysno,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("r7") n as usize,
        inlateout("r0") arg1 => ret,
        in("r1") arg2,
        in("r2") arg3,
        in("r3") arg4,
        in("r4") arg5,
        options(nostack, preserves_flags)
    );
    ret
}

/// A syscall that takes 6 arguments.
#[inline]
pub unsafe fn syscall6(
    n: Sysno,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("r7") n as usize,
        inlateout("r0") arg1 => ret,
        in("r1") arg2,
        in("r2") arg3,
        in("r3") arg4,
        in("r4") arg5,
        in("r5") arg6,
        options(nostack, preserves_flags)
    );
    ret
}