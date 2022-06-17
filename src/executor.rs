use std::{sync::Arc, any::Any, pin::Pin, future::Future};

use fast_async_mutex::mutex::Mutex as SpinMutex;
use tokio::sync::Mutex;

use crate::{shutdown_type::ShutdownType, virtual_thread::VThread};

pub enum ExecutorLock {
    None,
    Sys(Arc<Mutex<()>>),
    Spin(Arc<SpinMutex<()>>),
}

impl ExecutorLock {
    #[inline(always)] pub fn sys(&self) -> &Arc<Mutex<()>> {
        unsafe { crate::ExecutorLock__sys.unwrap_unchecked()(self) }
    }

    #[inline(always)] pub fn spin(&self) -> &Arc<SpinMutex<()>> {
        unsafe { crate::ExecutorLock__spin.unwrap_unchecked()(self) }
    }
}

pub enum ExecutorBehaviour {
    None,
    Shutdown(ShutdownType)
}

pub type Lock = Option<Box<dyn Any + Send + Sync>>;

// It isnt dead code huh
#[allow(dead_code)] type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
pub type Executor = Arc<dyn ExecutorFunc + Send + Sync>;

pub trait ExecutorFunc {
    fn call(&self, thread: VThread) -> BoxFuture<'static, ()>;
}