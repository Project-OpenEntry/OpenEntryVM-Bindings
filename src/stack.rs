#[repr(transparent)]
pub struct Stack(usize);

impl Stack {
    #[inline(always)] pub fn dispose(&self, stack_size: usize) {
        unsafe { crate::Stack__dispose.unwrap_unchecked()(self, stack_size) }
    }

    #[inline(always)] pub fn ptr(&self) -> u64 {
        unsafe { crate::Stack__ptr.unwrap_unchecked()(self) }
    }
}