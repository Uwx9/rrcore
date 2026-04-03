use core::panic::PanicInfo;
use crate::sbi::shutdown;
use crate::println;

#[panic_handler]    // 使用panic函数来对接panic！宏
fn panic(panic_info: &PanicInfo) -> ! 
{
    if let Some(location) = panic_info.location() {
        println!(
                "Panicked at {}:{} {}",
                location.file(),
                location.line(),
                panic_info.message()
            );
    } else {
        println!("Panicked: {}", panic_info.message())
    }
    shutdown(true);
}

