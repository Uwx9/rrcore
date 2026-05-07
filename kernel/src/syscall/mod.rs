const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

mod fs;
mod process;

use fs::sys_write;

/// 在内核中真正进行系统调用处理
pub fn syscall(id: usize, args: [usize; 3]) -> isize
{
    match id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => process::sys_exit(args[0] as isize),
        _ => panic!("not supported syscall id: {}", id),
    }
}
