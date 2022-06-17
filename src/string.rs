use crate::virtual_thread::VThread;

pub struct VMStr(*const u64, VThread);

impl VMStr {
    #[inline(always)] pub async fn from_str(value: String, thread: VThread) -> VMStr {
        unsafe { crate::VMStr__from_str.unwrap_unchecked()(value, thread).await }
    }

    #[inline(always)] pub fn str_eq(a: &VMStr, b: &VMStr) -> bool {
        unsafe { crate::VMStr__str_eq.unwrap_unchecked()(a, b) }
    }

    #[inline(always)] pub fn parse(&self) -> Option<f64> {
        unsafe { crate::VMStr__parse.unwrap_unchecked()(self) }
    }

    #[inline(always)] pub fn from(value: u64, thread: VThread) -> VMStr {
        unsafe { crate::VMStr__from.unwrap_unchecked()(value, thread) }
    }
    
    #[inline(always)] pub fn as_vm_value(&self) -> u64 {
        unsafe { crate::VMStr__as_vm_value.unwrap_unchecked()(self) }
    }
    
    #[inline(always)] pub fn ptr(&self) -> *const u8 {
        unsafe { crate::VMStr__ptr.unwrap_unchecked()(self) }
    }
    
    #[inline(always)] pub fn as_str(&self) -> &str {
        unsafe { crate::VMStr__as_str.unwrap_unchecked()(self) }
    }
    
    #[inline(always)] pub fn len(&self) -> u64 {
        unsafe { crate::VMStr__len.unwrap_unchecked()(self) }
    }

    #[inline(always)] pub async fn drop(&self) {
        unsafe { crate::VMStr__drop.unwrap_unchecked()(self).await }
    }

    #[inline(always)] pub async fn push(&mut self, other: &VMStr) {
        unsafe { crate::VMStr__push.unwrap_unchecked()(self, VMStr(other.0, other.1.clone())).await }
    }

    #[inline(always)] pub async fn cloned_push(&mut self, other: &VMStr) -> VMStr {
        unsafe { crate::VMStr__cloned_push.unwrap_unchecked()(self, VMStr(other.0, other.1.clone())).await }
    }
}