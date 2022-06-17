use std::{fmt::Debug, sync::Arc, marker::PhantomData};

use crate::shutdown_type::ShutdownType;

#[derive(Debug)]
pub enum EventType {
    VMRun,
    VMEnd,
    VMShutdown(ShutdownType),
    Foreign {
        from: u32, 
        event: u32, 
        payload: UnsafeEventArgs
    },
}

#[repr(transparent)]
pub struct UnsafeEventArgs(pub(crate) usize);

impl UnsafeEventArgs {
    pub(crate) fn new<T: Sized>(data: T) -> UnsafeEventArgs {
        UnsafeEventArgs(Arc::into_raw(Arc::new(data)) as usize)
    }

    // This can transmute something internally
    #[inline(always)]
    pub(crate) unsafe fn get<T: Sized>(&self) -> Arc<T> {
        Arc::from_raw(self.0 as *const T).clone()
    }

    // Unsafe because of heap corruption
    #[inline(always)]
    pub(crate) unsafe fn drop<T: Sized>(&self) {
        Arc::decrement_strong_count(self.0 as *const T)
    }
}

#[repr(transparent)]
pub struct EventArgs<T: Sized>(UnsafeEventArgs, PhantomData<T>);

impl<T: Sized> EventArgs<T> {
    pub fn new(data: T) -> EventArgs<T> {
        EventArgs(UnsafeEventArgs::new(data), PhantomData)
    }

    pub fn get(&self) -> Arc<T> {
        // Safe because we use always same generic type. No transmute happens
        unsafe { self.0.get::<T>() }
    }
}

impl<T: Sized> Debug for EventArgs<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("EventArgs(0x{:016x})", self.0.0))
    }
}

impl<T: Sized> Into<UnsafeEventArgs> for EventArgs<T> {
    fn into(self) -> UnsafeEventArgs {
        let ptr = self.0.0;

        // Make sure UnsafeEventArgs should not be dropped
        std::mem::forget(self);

        UnsafeEventArgs(ptr)
    }
}

impl<T: Sized> Into<EventArgs<T>> for UnsafeEventArgs {
    fn into(self) -> EventArgs<T> {
        EventArgs(self, PhantomData)
    }
}

impl<T: Sized> Drop for EventArgs<T> {
    fn drop(&mut self) {
        // Safe because we use always same generic type. No heap corruption happens
        unsafe { self.0.drop::<T>() }
    }
}

impl Debug for UnsafeEventArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("UnsafeEventArgs(0x{:016x})", self.0))
    }
}