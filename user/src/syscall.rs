/**
 *
 * user/src/syscall.rs 
 *
 */

use core::arch::asm;

pub fn syscall(id: usize, args: [usize; 3]) -> isize
{
    let mut ret: isize;

    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,   // 第一个参数写入x10, 这里还表示x10会将系统调用返回值写入ret
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
        );
    }

    ret
}
