//
//
//  kernel/src/sync/up.rs 
//
//

pub struct UPSafeCell<T> {
    inner: RefCell<T>,
}

// 当编译器确定适当时，会自动实现此 trait
unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    // 因为返回一个可能导致未定义行为的类型, 该函数是unsafe的
    pub unsafe fn new(value: T) -> Self
    {
        Self {inner: RefCell::new(value)}
    }

    pub fn exclusive_acess(&self) -> RefMut<'_, T>
    {
        self.inner.brrow_mut()
    }
}

