/*
 *
 *  kernel/src/batch.rs 
 *
 */

const USER_STACK_SIZE: usize =  4096 * 2;
const KERNEL_STACK_SIZE: usize =  4096 * 2;

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
}

static USER_STACK = UserStack {data: [0; USER_STACK_SIZE]};
static KERNEL_STACK = KernelStack {data: [0; USER_STACK_SIZE]};


static APP_MANAGER: UPSafeCell<Option<AppManager>> = UPsafeCell::new(None);

struct AppManager {
    app_num: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM],
    app_end: [usize; MAX_APP_NUM],
}

impl AppManager {
    pub fn get_current_app(&self) -> usize
    {
        self.current_app
    }

    pub fn print_app_info(&self)
    {
        let app_id = self.get_current_app();
        let current_app_start = app_start[app_id];
        let current_app_end = app_end[app_id];
        println!("currnet_app_id: {app_id}");
        println!("currnet_app_start: {current_app_start}");
        println!("currnet_app_end: {current_app_end}");
    }

    unsafe pub fn load_app(&self, app_id: usize)
    {
        if app_id >= self.app_num {
            panic("All applications completed!");
        }

        println!("[kernel] load app_id: {app_id}");

        // 清空app所需空间
        core::slice::from_raw_parts_mut(APP_BASE_ADDR as usize as *mut u8, APP_SIZE_LIMIT).fill(0);

        let app_src = from_raw_parts(self.app_start[app_id] as *const u8, app_end[app_id] - app_start[app_id]);
        let app_dst = from_raw_parts_mut(APP_BASE_ADDR as usize as *mut u8, app_sec.len());
        app_dst.copy_from_slice(app_src);

        // 在加载完应用程序后需要跳转过去执行, 为了确保取指正常需要下面这句
        // 它的功能是保证 在它之后的取指过程必须能够看到在它之前的所有对于取指内存区域的修改
        // 这里应该是保证能看见对APP_BASE_ADDR的修改, 因为即将去那里取指令
        asm!("fence.i");
    }
}


unsafe fn build_app_manager() -> AppManager
{
    extern "C" {fn _num_app();}
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = num_app_ptr.read_volatile();

    let app_start: [usize; MAX_APP_NUM] = [0; MAX_APP_NUM];
    let app_end: [usize; MAX_APP_NUM] = [0; MAX_APP_NUM];

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

/// 初始化全局变量APP_MANAGER
pub fn init_app_manager()
{
    let cell = APP_MANAGER.exclusive_acess();
    if cell.is_none() {
        *cell = Some(unsafe {build_app_manager()});
    }
}

pub fn run_next_app()
{
    
}

