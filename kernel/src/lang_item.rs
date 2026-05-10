/**
 *
 * lang_items表示语言项,这是rust要求实现的 
 *
 */

use core::panic::PanicInfo;
use crate::sbi::shutdown;
use crate::println;
use crate::tool;

/// 使用panic函数来对接panic！宏
#[panic_handler]   
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

    unsafe {
        tool::print_stack_trace();
    }
    shutdown(true);
}

