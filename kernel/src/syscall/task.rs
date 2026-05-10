use crate::batch::APP_MANAGER;

pub fn sys_get_taskinfo(task_id: usize) -> isize
{
    let mut guard = APP_MANAGER.exclusive_acess();
    if guard.is_none() {
        return -1;
    }

    let app_manager = guard.as_mut().unwrap();
    app_manager.print_app_info_specify(task_id);

    0
}
