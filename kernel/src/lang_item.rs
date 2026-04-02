use core::panic::PanicInfo;

#[panic_handler]    // 使用panic函数来对接panic！宏
fn panic(_info: &PanicInfo) -> ! 
{
    loop {}
}

