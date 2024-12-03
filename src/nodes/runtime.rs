use storage::*;

use crate::{pages, storage};

pub struct Runtime {
    pub is_running: bool,
    pub current_page: pages::PageId,
}

impl Node for Runtime {}

impl Runtime {
    fn new(storage: Storage<Immutable>) -> Option<Self> {
        Some(Self {
            is_running: true,
            current_page: pages::PageId(0),
        })
    }
}
