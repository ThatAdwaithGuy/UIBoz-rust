use node_proc_macro::Node;
use serde::{Deserialize, Serialize};
use storage::Node;
use renderer::window_renderer::Text;

pub trait Widget: storage::Node {
    fn render(&self) -> Vec<Text>;
}

#[derive(Node)]
pub struct WidgetRenderer {
    widgets: Vec<Box<dyn Widget>>,
    lastest_id: u32,
}

impl WidgetRenderer {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            lastest_id: 0,
        }
    }

    pub fn add_widget(&mut self, widget: impl Widget + 'static) {
        self.widgets.push(Box::new(widget));
    }
}
