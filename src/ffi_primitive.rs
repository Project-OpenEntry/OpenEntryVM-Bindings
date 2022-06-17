use crate::virtual_thread::VirtualThread;

macro_rules! ffi_primitive {
    ($ty: ident) => {
        paste::paste! {
            unsafe impl FfiPrimitive for $ty {
                unsafe fn get_mem(thread: &VirtualThread, addr: usize) -> Self { crate::[<VirtualThread__get_mem_ $ty>].unwrap_unchecked()(thread, addr) }
                unsafe fn get_mem_absolute(thread: &VirtualThread, addr: usize) -> Self { crate::[<VirtualThread__get_mem_absolute_ $ty>].unwrap_unchecked()(thread, addr) }
                unsafe fn set_mem_absolute(thread: &VirtualThread, addr: usize, data: Self) { crate::[<VirtualThread__set_mem_absolute_ $ty>].unwrap_unchecked()(thread, addr, data) }
                unsafe fn set_reg(thread: &VirtualThread, register: u8, data: Self) { crate::[<VirtualThread__set_reg_ $ty>].unwrap_unchecked()(thread, register, data) }
                unsafe fn get_reg(thread: &VirtualThread, register: u8) -> Self { crate::[<VirtualThread__get_reg_ $ty>].unwrap_unchecked()(thread, register) }
            }
        }
    };
}

pub unsafe trait FfiPrimitive: Copy {
    unsafe fn get_mem(thread: &VirtualThread, addr: usize) -> Self;
    unsafe fn get_mem_absolute(thread: &VirtualThread, addr: usize) -> Self;
    unsafe fn set_mem_absolute(thread: &VirtualThread, addr: usize, data: Self);
    unsafe fn set_reg(thread: &VirtualThread, register: u8, data: Self);
    unsafe fn get_reg(thread: &VirtualThread, register: u8) -> Self;
}

ffi_primitive!(u8);
ffi_primitive!(u16);
ffi_primitive!(u32);
ffi_primitive!(u64);