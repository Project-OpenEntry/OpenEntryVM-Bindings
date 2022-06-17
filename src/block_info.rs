use std::collections::HashMap;

pub enum UnlockInfo {
    Current,
    Addr(u64)
}

pub struct BlockInfo(HashMap<u64, UnlockInfo>);

impl BlockInfo {
    #[inline(always)] pub fn get(&self, inst: u64) -> Option<&UnlockInfo> {
        unsafe { crate::BlockInfo__get.unwrap_unchecked()(self, inst) }
    }
}