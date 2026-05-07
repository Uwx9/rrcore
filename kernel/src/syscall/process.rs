use crate::println;
use crate::batch::run_next_app;

pub fn sys_exit(exit_code: isize) -> !
{
    println!("[Kernel] app exit with exit_code: {}", exit_code);
    run_next_app();
}
