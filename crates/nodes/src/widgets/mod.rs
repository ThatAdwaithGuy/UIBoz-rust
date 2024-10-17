use std::collections::HashMap;

use storage::Node;
use node_proc_macro::Node;
use renderer::sub_win::SubWindow;

trait Widget : storage::Node {
    fn render(&self) -> SubWindow;
}

#[derive(Node)]
struct WidgetRenderer {
    widgets: HashMap<u32, Box<dyn Widget>>,
    lastest_id: u32
}

impl WidgetRenderer {
    fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            lastest_id: 0,
        }
    }

    fn add_widget(&mut self, widget: impl Widget + 'static) {
        self.widgets.insert(self.lastest_id + 1, Box::new(widget));
    } 
}
