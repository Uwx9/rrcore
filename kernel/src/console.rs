//
//  kernel/src/console.rs
//

use core::fmt::{ self, Write, Result};
use crate::sbi::console_putchar;

// 这个宏导出到 crate 根
#[macro_export]
macro_rules! print 
{
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?))
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}


// \x1b 表示16进制esc键(终端命令开始), [31m  表示变红, [0m 表示恢复默认
#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[34m", "[INFO]  ", $fmt, "\x1b[0m\n") $(, $($arg)+)?))
    }
}

#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[93m", "[WARN]  ", $fmt, "\x1b[0m\n") $(, $($arg)+)?))
    }
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[31m", "[ERROR] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?))
    }
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result
    {
        for c in s.chars() {
            console_putchar(c as usize) 
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments)
{
    Stdout.write_fmt(args).unwrap();
}

pub fn show_kernel_img()
{
    unsafe extern "C" {
        fn skernel();
        fn ekernel();
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbsstack();
        fn ebsstack();
        fn sbss();
        fn ebss();
    }
    info!("kernel [{:#x}, {:#x})", skernel as *const() as usize, ekernel as *const() as usize);
    warn!(".text [{:#x}, {:#x})", stext as *const() as usize, etext as *const() as usize);
    error!(".rodata [{:#x}, {:#x})", srodata as *const() as usize, erodata as *const() as usize);
    warn!(".data [{:#x}, {:#x})", sdata as *const() as usize, edata as *const() as usize);
    error!(".bss.stack [{:#x}, {:#x})", sbsstack as *const() as usize, ebsstack as *const() as usize);
    error!(".bss [{:#x}, {:#x})", sbss as *const() as usize, ebss as *const() as usize);
}
