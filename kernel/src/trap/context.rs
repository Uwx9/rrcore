use riscv::register::sstatus::{self, Sstatus, SPP};

/// TrapContext
#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    /// CSR sstatus
    pub sstatus: Sstatus,
    /// CSR sepc
    pub sepc: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize)
    {
        self.x[2] = sp;
    }
     
    /// 从内核第一次切换到用户态需构造一个用户的上下文并利用其退出到用户态
    pub fn app_init_context(entry: usize, sp: usize) -> Self
    {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };
        cx.set_sp(sp);

        cx
    }
}
