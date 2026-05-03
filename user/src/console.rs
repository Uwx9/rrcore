use core::fmt::Write;

const STDOUT: usize = 1;

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



struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result
    {
        write(STDOUT, s.as_bytes());    // 这里的write应该是用户库最终调用sys_write的封装, 就像C标准库
        Ok(())
    }
}

pub fn print(args: core::fmt::Arguments)
{
    Stdout.write_fmt(args).unwrap();
}
