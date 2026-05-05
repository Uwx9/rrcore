/**
 *
 * user/src/lib.rs 
 *
 */

#![feature(linkage)]
  
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() -> !
{
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit");
}

/// 这里的weak main是为了让用户程序没有定义main时可以让编译通过
#[unsafe(no_mangle)]
#[linkage = "weak"]     /* 这里的main是个弱符号 */
fn main() -> i32
{
    panic!("Cannot find main!");
}



mod syscall;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

/// 功能：将内存中缓冲区中的数据写入文件。
/// 参数：`fd` 表示待写入文件的文件描述符；
///      `buf` 表示内存中缓冲区的起始地址；
///      `len` 表示内存中缓冲区的长度。
/// 返回值：返回成功写入的长度。
/// syscall ID：64
#[inline(always)]
pub fn write(fd: usize, buffer: &[u8]) -> isize
{
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

/// 功能：退出应用程序并将返回值告知批处理系统。
/// 参数：`exit_code` 表示应用程序的返回值。
/// 返回值：该系统调用返回退出码
/// syscall ID：93
#[inline(always)]
pub fn exit(exit_code: usize) -> isize
{
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}






pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

