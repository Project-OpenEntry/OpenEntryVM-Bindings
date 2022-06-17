#![allow(non_upper_case_globals)]

pub mod extension_data;
pub mod virtual_thread;
pub mod shared_memory;
pub mod shutdown_type;
pub mod block_info;
pub mod extensions;
pub mod vm_config;
pub mod executor;
pub mod register;
pub mod vm_value;
pub mod archive;
pub mod runtime;
pub mod string;
pub mod stack;
pub mod event;
pub mod ffi;

mod thread_counter;

pub mod ffi_primitive;

mod macros;

use std::{sync::{Arc, atomic::{AtomicU32, Ordering}}, collections::{hash_map::{Values, Iter}, HashSet, HashMap}};
use extension_data::ExtensionData;
use fast_async_mutex::mutex::Mutex as SpinMutex;
use async_ffi::{BorrowingFfiFuture, FfiFuture};
use tokio::sync::{Mutex, MutexGuard, OwnedMutexGuard};

use executor::{ExecutorLock, Lock, ExecutorBehaviour};
use virtual_thread::{VThread, VirtualThread};
use block_info::{UnlockInfo, BlockInfo};
use extensions::{Extension, Extensions};
use shutdown_type::ShutdownType;
use shared_memory::Memory;
use vm_value::VMValue;
use event::EventType;
use runtime::Runtime;
use string::VMStr;
use stack::Stack;

use macros::{gen_bindings, load_bindings, async_binding};

gen_bindings![
    fn_bind![fn(&Arc<Runtime>, EventType), Runtime::dispatch_extension_event];
    fn_bind![fn(&Arc<Runtime>, u32, EventType), Runtime::send_extension_event];
    fn_bind![fn(&Runtime, VThread), Runtime::dispose_thread];
    fn_bind![fn(&Runtime, ShutdownType), Runtime::shutdown];
    
    fn_bind![fn(&Memory) -> *const u8, Memory::ptr];

    fn_bind![fn(&Extensions) -> Values<'_, u32, Arc<Extension>>, Extensions::iter];
    fn_bind![fn(&Extensions) -> Iter<'_, u32, Arc<Extension>>, Extensions::all];
    fn_bind![fn(&Extensions, u32) -> Arc<Extension>, Extensions::get];

    fn_bind![fn(&Extension, VThread, Lock, u32, bool) -> (Lock, ExecutorBehaviour), Extension::function_call];
    fn_bind![fn(&Extension, VThread, Lock, u32, bool) -> (Lock, ExecutorBehaviour), Extension::interrupt_call];
    fn_bind![fn(&Extension, Arc<Runtime>, EventType), Extension::dispatch_event];
    fn_bind![fn(&Extension, Arc<Runtime>, u32), Extension::init];

    fn_bind![fn(&BlockInfo, u64) -> Option<&UnlockInfo>, BlockInfo::get];
    fn_bind![fn(&Stack, usize), Stack::dispose];
    fn_bind![fn(&Stack) -> u64, Stack::ptr];

    fn_bind![fn(&ExecutorLock) -> &Arc<Mutex<()>>, ExecutorLock::sys];
    fn_bind![fn(&ExecutorLock) -> &Arc<SpinMutex<()>>, ExecutorLock::spin];

    fn_bind![fn(&VirtualThread, u32) -> Arc<Extension>, VirtualThread::get_extension];
    fn_bind![fn(&VirtualThread) -> Arc<BlockInfo>, VirtualThread::get_block_info];
    fn_bind![fn(&VirtualThread) -> bool, VirtualThread::should_stop];
    fn_bind![fn(&VirtualThread, u64, bool), VirtualThread::set_flag];
    fn_bind![fn(&VirtualThread, u64) -> bool, VirtualThread::get_flag];
    fn_bind![fn(&VirtualThread, u8, u32), VirtualThread::sub32];
    fn_bind![fn(&VirtualThread, u8, u32), VirtualThread::add32];
    fn_bind![fn(&VirtualThread, u64), VirtualThread::inc_inst];
    fn_bind![fn(&VirtualThread, u64), VirtualThread::push];
    fn_bind![fn(&VirtualThread) -> u64, VirtualThread::pop];
    fn_bind![fn(VThread, ShutdownType), VirtualThread::shutdown];
    fn_bind![fn(VThread), VirtualThread::dispose];
    fn_bind![fn(&VirtualThread, usize) -> u64, VirtualThread::get_mem_u64];
    fn_bind![fn(&VirtualThread, usize) -> u32, VirtualThread::get_mem_u32];
    fn_bind![fn(&VirtualThread, usize) -> u16, VirtualThread::get_mem_u16];
    fn_bind![fn(&VirtualThread, usize) -> u8, VirtualThread::get_mem_u8];
    fn_bind![fn(&VirtualThread, usize) -> u64, VirtualThread::get_mem_absolute_u64];
    fn_bind![fn(&VirtualThread, usize) -> u32, VirtualThread::get_mem_absolute_u32];
    fn_bind![fn(&VirtualThread, usize) -> u16, VirtualThread::get_mem_absolute_u16];
    fn_bind![fn(&VirtualThread, usize) -> u8, VirtualThread::get_mem_absolute_u8];
    fn_bind![fn(&VirtualThread, usize, u64), VirtualThread::set_mem_absolute_u64];
    fn_bind![fn(&VirtualThread, usize, u32), VirtualThread::set_mem_absolute_u32];
    fn_bind![fn(&VirtualThread, usize, u16), VirtualThread::set_mem_absolute_u16];
    fn_bind![fn(&VirtualThread, usize, u8), VirtualThread::set_mem_absolute_u8];
    fn_bind![fn(&VirtualThread, u8, u64), VirtualThread::set_reg_u64];
    fn_bind![fn(&VirtualThread, u8, u32), VirtualThread::set_reg_u32];
    fn_bind![fn(&VirtualThread, u8, u16), VirtualThread::set_reg_u16];
    fn_bind![fn(&VirtualThread, u8, u8), VirtualThread::set_reg_u8];
    fn_bind![fn(&VirtualThread, u8) -> u64, VirtualThread::get_reg_u64];
    fn_bind![fn(&VirtualThread, u8) -> u32, VirtualThread::get_reg_u32];
    fn_bind![fn(&VirtualThread, u8) -> u16, VirtualThread::get_reg_u16];
    fn_bind![fn(&VirtualThread, u8) -> u8, VirtualThread::get_reg_u8];

    fn_bind![fn(u64, VThread) -> VMValue, VMValue::from];
    fn_bind![fn(&mut VMValue) -> Option<(&mut VMStr, bool)>, VMValue::as_str];
    fn_bind![fn(&VMValue) -> Option<f64>, VMValue::as_f64];
    
    fn_bind![fn(&VMStr, &VMStr) -> bool, VMStr::str_eq];
    fn_bind![fn(&VMStr) -> Option<f64>, VMStr::parse];
    fn_bind![fn(u64, VThread) -> VMStr, VMStr::from];
    fn_bind![fn(&VMStr) -> u64, VMStr::as_vm_value];
    fn_bind![fn(&VMStr) -> *const u8, VMStr::ptr];
    fn_bind![fn(&VMStr) -> &str, VMStr::as_str];
    fn_bind![fn(&VMStr) -> u64, VMStr::len];
];

pub(crate) static mut Runtime__set_error_data: Option<fn(&Runtime, String) -> BorrowingFfiFuture<'static, ()>> = None;
pub(crate) static mut Runtime__spawn: Option<fn(&Arc<Runtime>, u64) -> BorrowingFfiFuture<'static, ()>> = None;

pub(crate) static mut VirtualThread__get_temp_vmstrs: Option<fn(&VirtualThread) -> BorrowingFfiFuture<'static, MutexGuard<'_, HashSet<(u64, usize)>>>> = None;
pub(crate) static mut VirtualThread__set_error_data: Option<fn(&VirtualThread, String) -> BorrowingFfiFuture<'static, ()>> = None;
pub(crate) static mut VirtualThread__spawn: Option<fn(&VirtualThread, u64) -> BorrowingFfiFuture<'static, ()>> = None;

pub(crate) static mut VMStr__cloned_push: Option<fn(&VMStr, VMStr) -> BorrowingFfiFuture<'static, VMStr>> = None;
pub(crate) static mut VMStr__push: Option<fn(&mut VMStr, VMStr) -> BorrowingFfiFuture<'static, ()>> = None;
pub(crate) static mut VMStr__from_str: Option<fn(String, VThread) -> FfiFuture<VMStr>> = None;
pub(crate) static mut VMStr__drop: Option<fn(&VMStr) -> BorrowingFfiFuture<'static, ()>> = None;

pub(crate) static mut ExtensionData__lock: Option<fn(&ExtensionData) -> FfiFuture<OwnedMutexGuard<HashMap<u32, usize>>>> = None;

#[cfg(target_pointer_width = "32")]
compile_error!("OpenEntry is only for 64-bit or higher operating system.");

#[cfg(all(not(unix), not(windows)))]
compile_error!("Unsupported Operating System");

static EXTENSION_ID: AtomicU32 = AtomicU32::new(0);

pub fn init(runtime: &Arc<Runtime>, id: u32) {
    if std::mem::size_of::<usize>() < 8 { panic!("OpenEntry is only for 64-bit or higher operating system.") }

    EXTENSION_ID.store(id, Ordering::Relaxed);

    #[allow(unused_assignments)] // shutup it is used
    let mut idx: usize = 0;

    load_bindings![
        idx,
        runtime,

        fn_bind![fn(&Arc<Runtime>, EventType), Runtime::dispatch_extension_event];
        fn_bind![fn(&Arc<Runtime>, u32, EventType), Runtime::send_extension_event];
        fn_bind![fn(&Runtime, VThread), Runtime::dispose_thread];
        fn_bind![fn(&Runtime, ShutdownType), Runtime::shutdown];

        fn_bind![fn(&Memory) -> *const u8, Memory::ptr];

        fn_bind![fn(&Extensions) -> Values<'_, u32, Arc<Extension>>, Extensions::iter];
        fn_bind![fn(&Extensions) -> Iter<'_, u32, Arc<Extension>>, Extensions::all];
        fn_bind![fn(&Extensions, u32) -> Arc<Extension>, Extensions::get];

        fn_bind![fn(&Extension, VThread, Lock, u32, bool) -> (Lock, ExecutorBehaviour), Extension::function_call];
        fn_bind![fn(&Extension, VThread, Lock, u32, bool) -> (Lock, ExecutorBehaviour), Extension::interrupt_call];
        fn_bind![fn(&Extension, Arc<Runtime>, EventType), Extension::dispatch_event];
        fn_bind![fn(&Extension, Arc<Runtime>, u32), Extension::init];

        fn_bind![fn(&BlockInfo, u64) -> Option<&UnlockInfo>, BlockInfo::get];
        fn_bind![fn(&Stack, usize), Stack::dispose];
        fn_bind![fn(&Stack) -> u64, Stack::ptr];

        fn_bind![fn(&ExecutorLock) -> &Arc<Mutex<()>>, ExecutorLock::sys];
        fn_bind![fn(&ExecutorLock) -> &Arc<SpinMutex<()>>, ExecutorLock::spin];

        fn_bind![fn(&VirtualThread, u32) -> Arc<Extension>, VirtualThread::get_extension];
        fn_bind![fn(&VirtualThread) -> Arc<BlockInfo>, VirtualThread::get_block_info];
        fn_bind![fn(&VirtualThread) -> bool, VirtualThread::should_stop];
        fn_bind![fn(&VirtualThread, u64, bool), VirtualThread::set_flag];
        fn_bind![fn(&VirtualThread, u64) -> bool, VirtualThread::get_flag];
        fn_bind![fn(&VirtualThread, u8, u32), VirtualThread::sub32];
        fn_bind![fn(&VirtualThread, u8, u32), VirtualThread::add32];
        fn_bind![fn(&VirtualThread, u64), VirtualThread::inc_inst];
        fn_bind![fn(&VirtualThread, u64), VirtualThread::push];
        fn_bind![fn(&VirtualThread) -> u64, VirtualThread::pop];
        fn_bind![fn(VThread, ShutdownType), VirtualThread::shutdown];
        fn_bind![fn(VThread), VirtualThread::dispose];
        
        fn_bind![fn(u64, VThread) -> VMValue, VMValue::from];
        fn_bind![fn(&mut VMValue) -> Option<(&mut VMStr, bool)>, VMValue::as_str];
        fn_bind![fn(&VMValue) -> Option<f64>, VMValue::as_f64];

        fn_bind![fn(&VMStr, &VMStr) -> bool, VMStr::str_eq];
        fn_bind![fn(&VMStr) -> Option<f64>, VMStr::parse];
        fn_bind![fn(u64, VThread) -> VMStr, VMStr::from];
        fn_bind![fn(&VMStr) -> u64, VMStr::as_vm_value];
        fn_bind![fn(&VMStr) -> *const u8, VMStr::ptr];
        fn_bind![fn(&VMStr) -> &str, VMStr::as_str];
        fn_bind![fn(&VMStr) -> u64, VMStr::len];
    ];

    println!("{idx}");

    unsafe {
        async_binding!(idx, runtime, Runtime::set_error_data, (), [data: String]);
        async_binding!(idx, runtime, &Arc, Runtime::spawn, (), [addr: u64]);

        async_binding![idx, runtime, VirtualThread::get_temp_vmstrs, MutexGuard<'_, HashSet<(u64, usize)>>, []];
        async_binding![idx, runtime, VirtualThread::set_error_data, (), [data: String]];

        async_binding![idx, runtime, VirtualThread::spawn, (), [addr: u64]];

        async_binding![idx, runtime, noself, VMStr::from_str, VMStr, [value: String, thread: VThread]];

        async_binding![idx, runtime, VMStr::drop, (), []];
    }

    println!("{idx}");

    unsafe {
        VirtualThread__get_mem_u64 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx] as *const ()));
        VirtualThread__get_mem_u32 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 1] as *const ()));
        VirtualThread__get_mem_u16 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 2] as *const ()));
        VirtualThread__get_mem_u8 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 3] as *const ()));
        VirtualThread__get_mem_absolute_u64 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 4] as *const ()));
        VirtualThread__get_mem_absolute_u32 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 5] as *const ()));
        VirtualThread__get_mem_absolute_u16 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 6] as *const ()));
        VirtualThread__get_mem_absolute_u8 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 7] as *const ()));
        VirtualThread__set_mem_absolute_u64 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 8] as *const ()));
        VirtualThread__set_mem_absolute_u32 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 9] as *const ()));
        VirtualThread__set_mem_absolute_u16 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 10] as *const ()));
        VirtualThread__set_mem_absolute_u8 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 11] as *const ()));
        VirtualThread__set_reg_u64 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 12] as *const ()));
        VirtualThread__set_reg_u32 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 13] as *const ()));
        VirtualThread__set_reg_u16 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 14] as *const ()));
        VirtualThread__set_reg_u8 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 15] as *const ()));
        VirtualThread__get_reg_u64 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 16] as *const ()));
        VirtualThread__get_reg_u32 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 17] as *const ()));
        VirtualThread__get_reg_u16 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 18] as *const ()));
        VirtualThread__get_reg_u8 = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 19] as *const ()));        
        
        VMStr__push = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 20] as *const ()));
        VMStr__cloned_push = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 21] as *const ()));
        ExtensionData__lock = Some(std::mem::transmute::<*const (), _>(runtime.ffi.0[idx + 22] as *const ()));
    }
}

pub fn id() -> u32 {
    EXTENSION_ID.load(Ordering::Relaxed)
}

pub use tokio; // Re-export tokio