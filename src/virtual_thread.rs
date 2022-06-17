use std::{pin::Pin, sync::{Arc, atomic::AtomicU64}, marker::PhantomPinned, collections::HashSet};

use tokio::sync::MutexGuard;

use crate::{runtime::Runtime, shared_memory::SharedMemory, executor::ExecutorLock, stack::Stack, register::Register, extensions::Extension, block_info::BlockInfo, shutdown_type::ShutdownType, ffi_primitive::FfiPrimitive, extension_data::ExtensionData};

pub type VThread = Pin<Arc<VirtualThread>>;

pub struct VirtualThread {
    pub runtime: Arc<Runtime>,
    pub memory: SharedMemory,
    pub lock: ExecutorLock,
    pub stack: Stack,

    pub extension_data: ExtensionData,

    pub registers: [Register; 16],
    _stack_size: usize,
    pub flags: AtomicU64,

    _phantom: PhantomPinned
}

impl VirtualThread {
    #[inline(always)] pub fn get_extension(&self, id: u32) -> Arc<Extension> {
        unsafe { crate::VirtualThread__get_extension.unwrap_unchecked()(self, id) }
    }
    
    #[inline(always)] pub async fn get_temp_vmstrs(&self) -> MutexGuard<'_, HashSet<(u64, usize)>> {
        unsafe { crate::VirtualThread__get_temp_vmstrs.unwrap_unchecked()(self).await }
    }

    #[inline(always)] pub async fn set_error_data(&self, data: impl Into<String>) {
        unsafe { crate::VirtualThread__set_error_data.unwrap_unchecked()(self, data.into()).await }
    }

    #[inline(always)] pub fn get_block_info(&self) -> Arc<BlockInfo> {
        unsafe { crate::VirtualThread__get_block_info.unwrap_unchecked()(self) }
    }

    #[inline(always)] pub fn should_stop(&self) -> bool {
        unsafe { crate::VirtualThread__should_stop.unwrap_unchecked()(self) }
    }
    
    #[inline(always)] pub async fn spawn(&self, addr: u64) {
        unsafe { crate::VirtualThread__spawn.unwrap_unchecked()(self, addr).await }
    }
    
    #[inline(always)] pub fn set_flag(&self, id: u64, value: bool) {
        unsafe { crate::VirtualThread__set_flag.unwrap_unchecked()(self, id, value) }
    }

    #[inline(always)] pub fn get_flag(&self, id: u64) -> bool {
        unsafe { crate::VirtualThread__get_flag.unwrap_unchecked()(self, id) }
    }

    #[inline(always)] pub fn sub32(&self, register: u8, amount: u32) {
        unsafe { crate::VirtualThread__sub32.unwrap_unchecked()(self, register, amount) }
    }
    
    #[inline(always)] pub fn add32(&self, register: u8, amount: u32) {
        unsafe { crate::VirtualThread__add32.unwrap_unchecked()(self, register, amount) }
    }

    #[inline(always)] pub fn inc_inst(&self, amount: u64) {
        unsafe { crate::VirtualThread__inc_inst.unwrap_unchecked()(self, amount) }
    }

    #[inline(always)] pub fn push(&self, value: u64) {
        unsafe { crate::VirtualThread__push.unwrap_unchecked()(self, value) }
    }

    #[inline(always)] pub fn pop(&self) -> u64 {
        unsafe { crate::VirtualThread__pop.unwrap_unchecked()(self) }
    }
    
    #[inline(always)] pub fn shutdown(self: VThread, shutdown_type: ShutdownType) {
        unsafe { crate::VirtualThread__shutdown.unwrap_unchecked()(self, shutdown_type) }
    }
    
    #[inline(always)] pub fn dispose(self: VThread) {
        unsafe { crate::VirtualThread__dispose.unwrap_unchecked()(self) }
    }
    
    #[inline(always)] pub fn get_mem<T: FfiPrimitive>(&self, addr: usize) -> T {
        unsafe { T::get_mem(self, addr) }
    }

    #[inline(always)] pub fn get_mem_absolute<T: FfiPrimitive>(&self, addr: usize) -> T {
        unsafe { T::get_mem_absolute(self, addr) }
    }

    #[inline(always)] pub fn set_mem_absolute<T: FfiPrimitive>(&self, addr: usize, data: T) {
        unsafe { T::set_mem_absolute(self, addr, data) }
    }

    #[inline(always)] pub fn set_reg<T: FfiPrimitive>(&self, register: u8, data: T) {
        unsafe { T::set_reg(self, register, data) }
    }

    #[inline(always)] pub fn get_reg<T: FfiPrimitive>(&self, register: u8) -> T {
        unsafe { T::get_reg(self, register) }
    }
}