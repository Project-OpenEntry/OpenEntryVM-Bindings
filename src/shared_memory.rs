use std::sync::Arc;

pub type SharedMemory = Arc<Memory>;

#[repr(transparent)]
pub struct Memory {
    _memory: [u8]
}

impl Memory {
    #[inline(always)] pub fn ptr(&self) -> *const u8 {
        unsafe { crate::Memory__ptr.unwrap_unchecked()(self) }
    }
}