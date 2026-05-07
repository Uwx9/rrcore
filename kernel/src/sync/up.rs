/*
 *
 * kernel/src/sync/up.rs 
 *
 */

use core::cell::{RefCell, RefMut};

/// UP = Unique Processor
/// 仅用于实现对RefCell的封装, 并为其实现sync trait以应对编译器检查
pub struct UPSafeCell<T> {
    inner: RefCell<T>,      // 注意！这里inner是私有的, 外部函数只能调用我们暴露的下面的接口
}

// unsafe 表示程序员保证这个类型可以安全地跨线程共享
unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    // 这里unsafe表明需要调用者自己保证安全前提, 即按照要求使用这个变量
    // 就是为了让调用者也加上unsafe以显示声明这是一个unsafe，实际没有约束力,
    // 纯看程序员自己是否按规矩使用, 即使声明unsafe调用后, 不按要求使用就可能出错
    pub const unsafe fn new(value: T) -> Self   // 这里的const表示这个函数可以在编译时调用
    {
        Self {inner: RefCell::new(value)}
    }

    // 外面只能通过这个pub的函数拿到可变引用
    pub fn exclusive_acess(&self) -> RefMut<'_, T>
    {
        self.inner.borrow_mut()
    }
}

