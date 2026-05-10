/*
 *
 *  kernel/src/batch.rs 
 *
 *
 */


use crate::println;
use crate::trap::TrapContext;
use crate::sync;

const USER_STACK_SIZE: usize =  4096 * 2;
const KERNEL_STACK_SIZE: usize =  4096 * 2;
const APP_BASE_ADDR: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x100000;
const MAX_APP_NUM: usize = 16;

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    pub fn get_sp(&self) -> usize
    {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    pub fn get_sp(&self) -> usize
    {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    /// 该函数用于构造从内核到用户态所需的上下文
    pub fn push_context(&self, cx: TrapContext) -> usize
    {
        let mut ksp = self.get_sp();
        ksp -= core::mem::size_of::<TrapContext>();
        let cx_ptr = ksp as *mut TrapContext;
        unsafe {
            core::ptr::write(cx_ptr, cx);
        }
        ksp
    }
}

static USER_STACK: UserStack = UserStack {data: [0; USER_STACK_SIZE]};
static KERNEL_STACK: KernelStack = KernelStack {data: [0; USER_STACK_SIZE]};


pub static APP_MANAGER: sync::up::UPSafeCell<Option<AppManager>> = unsafe { sync::up::UPSafeCell::new(None) };

pub struct AppManager {
    app_num: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM],
    app_end: [usize; MAX_APP_NUM],
}

impl AppManager {
    pub fn get_app_num(&self) -> usize
    {
        self.app_num
    }

    pub fn get_current_app(&self) -> usize
    {
        self.current_app
    }

    pub fn print_app_info_specify(&self, task_id: usize)
    {
        if task_id >= self.get_app_num() {
            panic!("no this app, id{}", task_id);
        }
        let app_start = self.app_start[task_id];
        let app_end = self.app_end[task_id];
        println!("app_id: {}", task_id);
        println!("app_start: {}", app_start);
        println!("app_end: {}", app_end);
    }


    pub fn print_app_info(&self)
    {
        let app_id = self.get_current_app();
        let current_app_start = self.app_start[app_id];
        let current_app_end = self.app_end[app_id];
        println!("current_app_id: {}", app_id);
        println!("current_app_start: {}", current_app_start);
        println!("current_app_end: {}", current_app_end);
    }

    pub fn move_to_next_app(&mut self)
    {
        self.current_app += 1;
        if self.current_app >= self.app_num {
            self.current_app = 0;
        }
    }

    pub unsafe fn load_app(&self, app_id: usize)
    {
        unsafe {
            if app_id >= self.app_num {
                panic!("All applications completed!");
            }

            println!("[kernel] load app_id: {}", app_id);

            // 清空app所需空间
            core::slice::from_raw_parts_mut(APP_BASE_ADDR as usize as *mut u8, APP_SIZE_LIMIT).fill(0);
            println!("[kernel] clear app space from {:#x} to {:#x}", APP_BASE_ADDR, APP_BASE_ADDR + APP_SIZE_LIMIT);

            let app_src = core::slice::from_raw_parts(self.app_start[app_id] as *const u8, self.app_end[app_id] - self.app_start[app_id]);
            let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDR as usize as *mut u8, app_src.len());
            app_dst.copy_from_slice(app_src);

            // 在加载完应用程序后需要跳转过去执行, 为了确保取指正常需要下面这句
            // 它的功能是保证 在它之后的取指过程必须能够看到在它之前的所有对于取指内存区域的修改
            // 这里应该是保证能看见对APP_BASE_ADDR的修改, 因为即将去那里取指令
            core::arch::asm!("fence.i");
            println!("[kernel] load app_id: {} finished!", app_id);
        }
        
    }
}


unsafe fn build_app_manager() -> AppManager
{
    unsafe extern "C" { fn _num_app(); }

    unsafe {
        let num_app_ptr = _num_app as *const() as usize as *const usize;
        let num_app = num_app_ptr.read_volatile();

        let mut app_start: [usize; MAX_APP_NUM] = [0; MAX_APP_NUM];
        let mut app_end: [usize; MAX_APP_NUM] = [0; MAX_APP_NUM];

        let app_start_raw: &[usize] = core::slice::from_raw_parts(num_app_ptr.add(1), num_app);
        let app_end_raw: &[usize] = core::slice::from_raw_parts(num_app_ptr.add(1 + num_app), num_app);

        app_start[..num_app].copy_from_slice(app_start_raw);
        app_end[..num_app].copy_from_slice(app_end_raw);

        AppManager {
            app_num: num_app,
            current_app: 0,
            app_start,
            app_end,
        }
    }

}

/// 初始化全局变量APP_MANAGER
pub fn init_app_manager()
{
    let mut cell = APP_MANAGER.exclusive_acess();
    if cell.is_none() {
        *cell = Some(unsafe {build_app_manager()});
    }
}

/// 每一次run_next_app都会重新在内核栈顶构造一个用户上下文并切换过去
pub fn run_next_app() -> !
{
    let mut guard = APP_MANAGER.exclusive_acess();
    let app_manager = guard.as_mut().unwrap();
    // let mut app_manager = APP_MANAGER.exclusive_acess().as_mut().unwrap();
    let current_app = app_manager.get_current_app();
    unsafe {
        app_manager.load_app(current_app);
    }
    println!("[kernel] load_app: {} ok!", current_app);
    app_manager.print_app_info();
    app_manager.move_to_next_app();
    drop(guard);

    unsafe extern "C" {fn __restore(cx_addr: usize);}
    
    // 这里先构造内核栈上的用户上下文再退出, 退出后sp-> userstack, sscratch->kernelstack
    unsafe {
        __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(APP_BASE_ADDR, USER_STACK.get_sp())));
    }
    panic!("Unreachable in batch::run_current_app!");
}
