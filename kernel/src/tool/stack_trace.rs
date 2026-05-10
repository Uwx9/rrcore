use crate::println;

pub unsafe fn print_stack_trace() -> ()
{
    unsafe {
        let mut fp: *const usize; 
        core::arch::asm!("mv {}, fp", out(reg) fp); // out(reg) fp 表示把寄存器的值赋给fp

        println!("========= print stack trace start =========");
        while fp != core::ptr::null() {
            let saved_ra = *fp.sub(1);
            let saved_fp = *fp.sub(2);

            // 016x表示16宽度的16进制数表示, 不足的补0
            println!("saved_ra = 0x{:016x},  saved_fp = 0x{:016x}", saved_ra, saved_fp);

            fp = saved_fp as *const usize;
        }
        println!("========= print stack trace end =========");
    }
}
