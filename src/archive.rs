use std::{sync::Arc, collections::HashMap};

use crate::{block_info::BlockInfo, vm_config::VMConfig};

pub struct Archive {
    pub block_info: Option<Arc<BlockInfo>>,
    pub files: HashMap<String, Box<[u8]>>,
    pub code: Box<[u8]>,
    pub conf: VMConfig,
}