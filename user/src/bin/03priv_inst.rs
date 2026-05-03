use core::arch::asm;

#[no_mangle]
fn main() -> i32
{
    println!("try to excute privileged instruction in U mode");
    println!("尝试执行特权级指令在用户模式中");
    unsafe {
        asm!("sret");
    }
    0
}
