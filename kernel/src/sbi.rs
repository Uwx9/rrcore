pub fn console_putchar(c: usize)
{
    #[allow(deprecated)]    // 关闭已弃用警告
    sbi_rt::legacy::console_putchar(c);
}

pub fn shutdown(failure: bool) -> !
{
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!("unable to reach")     // 类似panic！
}
