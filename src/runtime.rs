use tokio::{runtime::Runtime as TokioRuntime, sync::mpsc::UnboundedReceiver};
use std::{sync::{Arc, atomic::AtomicBool}, collections::HashSet};

use tokio::sync::{Mutex, RwLock};

use crate::{shared_memory::SharedMemory, extensions::Extensions, archive::Archive, shutdown_type::ShutdownType, thread_counter::ThreadCounter, executor::Executor, event::EventType, virtual_thread::VThread, extension_data::ExtensionData, ffi::FfiBindings};

pub struct Runtime {
    pub temp_vmstr: Arc<Mutex<HashSet<(u64, usize)>>>,
    pub memory: RwLock<SharedMemory>,
    pub tokio_rt: Arc<TokioRuntime>,
    pub extensions: Extensions,
    pub archive: Arc<Archive>,
    pub shutdown: AtomicBool,
    pub initial_inst: u64,
    pub base: u64,

    pub ffi: FfiBindings,
    pub extension_data: ExtensionData,

    _shutdown_rx: Mutex<UnboundedReceiver<ShutdownType>>,
    _threads: ThreadCounter,
    pub stack_size: usize,

    pub executor: Executor
}

impl Runtime {
    #[inline(always)] pub fn dispatch_extension_event(self: &Arc<Self>, event: EventType) {
        unsafe { crate::Runtime__dispatch_extension_event.unwrap_unchecked()(self, event) }
    }
    
    #[inline(always)] pub fn send_extension_event(self: &Arc<Self>, target: u32, event: EventType) {
        unsafe { crate::Runtime__send_extension_event.unwrap_unchecked()(self, target, event) }
    }
    
    #[inline(always)] pub fn dispose_thread(&self, thread: VThread) {
        unsafe { crate::Runtime__dispose_thread.unwrap_unchecked()(self, thread) }
    }
    
    #[inline(always)] pub fn shutdown(&self, shutdown_type: ShutdownType) {
        unsafe { crate::Runtime__shutdown.unwrap_unchecked()(self, shutdown_type) }
    }

    #[inline(always)] pub async fn set_error_data(&self, data: String) {
        unsafe { crate::Runtime__set_error_data.unwrap_unchecked()(self, data).await; }
    }

    #[inline(always)] pub async fn spawn(self: &Arc<Self>, addr: u64) {
        unsafe { crate::Runtime__spawn.unwrap_unchecked()(self, addr).await; }
    }
}