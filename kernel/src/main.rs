#![no_std]
#![no_main]
// #![feature(panic_info_message)] 

mod lang_item;
mod console;
mod sbi;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

// static mut LOVE: i32 = 32;

#[unsafe(no_mangle)]    // 将 rust_main 标记为 #[no_mangle] 以免编译器对它的名字进行混淆
fn rust_main() -> !
{
    clear_bss();
    println!("\nKernel Output");
    console::show_kernel_img();

    panic!("shutdowm machine...");
}

fn clear_bss()
{
    unsafe extern "C" {
        // 伪装成函数，不伪装成数据是不希望误写它
        fn sbss();
        fn ebss();
    }

    (sbss as *const() as usize .. ebss as *const() as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    })
}

