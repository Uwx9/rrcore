//
//  kernel/src/batch.rs 
//

struct AppManager {
    app_num: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}
