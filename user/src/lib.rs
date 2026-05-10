/*
 *
 *  user/src/lib.rs 
 *
 */

#![no_std]
#![feature(linkage)]
  
// pub mod console是为了让用户程序可见console模块中的println!宏
pub mod console;
mod syscall;
mod lang_items;


const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_GET_TASKINFO: usize = 94;
use syscall::syscall;

/// 用户入口函数，负责调用用户定义的main函数，并在main函数返回后调用exit系统调用退出程序。
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() -> !
{
    clear_bss();
    exit(main());   // sys_exit 会run_next_app
    panic!("unreachable after sys_exit");
}

/// 这里的weak main是为了让用户程序没有定义main时可以让编译通过
#[unsafe(no_mangle)]
#[linkage = "weak"]     /* 这里的main是个弱符号 */
fn main() -> i32
{
    panic!("Cannot find main!");
}

fn clear_bss()
{
    unsafe extern "C" {
        safe fn start_bss();
        safe fn end_bss();
    }
    let bss_start = start_bss as *const() as usize;
    let bss_end = end_bss as *const() as usize;
    for addr in bss_start..bss_end {
        unsafe { (addr as *mut u8).write_volatile(0) };
    }
}


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
pub fn exit(exit_code: i32) -> isize
{
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}

/// 功能：打印任务信息
/// 参数：`` 表示应用程序的返回值。
/// 返回值：该系统调用返回0表示成功，-1表示失败
/// syscall ID：94
#[inline(always)]
pub fn sys_get_taskinfo(task_id: usize) -> isize
{
    syscall(SYSCALL_GET_TASKINFO, [task_id as usize, 0, 0])
}
