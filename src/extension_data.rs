use std::{collections::HashMap, sync::{Arc, atomic::Ordering}};

use tokio::sync::{Mutex, OwnedMutexGuard};

use crate::EXTENSION_ID;

pub struct ExtensionData(Arc<Mutex<HashMap<u32, usize>>>);

impl ExtensionData {
    #[inline(always)] async fn lock(&self) -> OwnedMutexGuard<HashMap<u32, usize>> {
        unsafe { crate::ExtensionData__lock.unwrap_unchecked()(self).await }
    }

    // This can transmute something internally
    pub async unsafe fn set<T>(&self, data: T) {
        let arc = Arc::into_raw(Arc::new(data));

        Arc::increment_strong_count(arc);

        if let Some(bef) = self.lock().await.insert(EXTENSION_ID.load(Ordering::Relaxed), arc as usize) {
            Arc::decrement_strong_count(bef as *const T);
        }
    }

    // This can transmute something internally
    pub async unsafe fn get<T>(&self) -> Option<Arc<T>> {
        Some(Arc::from_raw(*self.lock().await.get(&EXTENSION_ID.load(Ordering::Relaxed))? as *const T).clone())
    }
}