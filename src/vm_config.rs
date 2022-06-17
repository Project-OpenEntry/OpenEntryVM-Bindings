#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExecutorKind {
    Atomic = 0,
    SysLockInst = 1,
    SpinLockInst = 2,
    SysLockBlock = 3,
    SpinLockBlock = 4,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThreadingKind {
    Single = 0,
    Managed = 1,
    Unmanaged = 2,
}

pub struct VMConfig {
    pub executor_kind: ExecutorKind,
    pub threading_kind: ThreadingKind,
    pub max_threads: u16,
    pub stack_size: u64,
}