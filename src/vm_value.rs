use crate::{string::VMStr, virtual_thread::VThread};

pub enum VMValue {
    ConstStr(VMStr),
    VarStr(VMStr),
    Float(f64),
}

impl VMValue {
    #[inline(always)] pub fn from(value: u64, thread: VThread) -> VMValue {
        unsafe { crate::VMValue__from.unwrap_unchecked()(value, thread) }
    }

    #[inline(always)] pub fn as_str(&mut self) -> Option<(&mut VMStr, bool)> {
        unsafe { crate::VMValue__as_str.unwrap_unchecked()(self) }
    }

    #[inline(always)] pub fn as_f64(&self) -> Option<f64> {
        unsafe { crate::VMValue__as_f64.unwrap_unchecked()(self) }
    }
}