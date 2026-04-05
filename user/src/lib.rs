/**
 *
 * user/src/lib.rs 
 *
 */

#![feature(linkage)]
  
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> !
{
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit");
}

#[no_mangle]
#[linkage = "weak"]
fn main() -> i32
{
    panic!("Cannot find main!");
}


use syscall::*

#[inline(always)]
pub fn write(fd: usize, buffer: &[u8]) -> isize
{
    sys_write(fd, buffer);
}

#[inline(always)]
pub fn exit(exit_code: u32) -> isize
{
    sys_exit(exit_code);
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

