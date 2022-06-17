use std::sync::{atomic::{AtomicU8, AtomicU32}, Arc};

use tokio::sync::{Mutex, mpsc::UnboundedSender};

use crate::shutdown_type::ShutdownType;

#[allow(dead_code)]
pub(crate) struct ThreadCounter {
    error_data: Arc<Mutex<Option<String>>>,
    ch: UnboundedSender<ShutdownType>,
    shutdown_type: AtomicU8,
    counter: AtomicU32,
}