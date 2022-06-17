use std::{collections::{HashMap, hash_map::{Values, Iter}}, sync::Arc};

use libloading::Library;

use crate::{virtual_thread::VThread, executor::{Lock, ExecutorBehaviour}, event::EventType, runtime::Runtime};

pub struct Extensions(HashMap<u32, Arc<Extension>>);

impl Extensions {
    #[inline(always)] pub fn iter(&self) -> Values<'_, u32, Arc<Extension>> {
        unsafe { crate::Extensions__iter.unwrap_unchecked()(self) }
    }

    #[inline(always)] pub fn all(&self) -> Iter<'_, u32, Arc<Extension>> {
        unsafe { crate::Extensions__all.unwrap_unchecked()(self) }
    }

    #[inline(always)] pub fn get(&self, id: u32) -> Arc<Extension> {
        unsafe { crate::Extensions__get.unwrap_unchecked()(self, id) }
    }
}

pub struct Extension {
    _lib: Library, 
    _env_fn: Option<usize>, // Function Pointer 
    _envj_fn: Option<usize>, // Function Pointer
    _event: usize, // Function Pointer
    _init: usize, // Function Pointer
}

impl Extension {
    #[inline(always)] pub fn function_call(&self, vthread: VThread, lock: Lock, id: u32, drop: bool) -> (Lock, ExecutorBehaviour) {
        unsafe { crate::Extension__function_call.unwrap_unchecked()(self, vthread, lock, id, drop) }
    }

    #[inline(always)] pub fn interrupt_call(&self, vthread: VThread, lock: Lock, id: u32, drop: bool) -> (Lock, ExecutorBehaviour) {
        unsafe { crate::Extension__interrupt_call.unwrap_unchecked()(self, vthread, lock, id, drop) }
    }

    #[inline(always)] pub fn dispatch_event(&self, runtime: Arc<Runtime>, event: EventType) {
        unsafe { crate::Extension__dispatch_event.unwrap_unchecked()(self, runtime, event) }
    }
}