use pages::Page;
use storage::*;

#[derive(node_proc_macro::Node)]
pub struct Runtime {
    pub is_running: bool,
    pub current_page: pages::PageId,
}

impl Runtime {
    fn new(storage: Storage<Immutable>) -> Option<Self> {
        Some(Self {
            is_running: true,
            current_page: pages::PageId(0),
        })
    }
}
