use node_proc_macro::Node;
use renderer::window_renderer::Text;
use serde::{Deserialize, Serialize};
use storage::Node;

pub trait Widget: storage::Node + std::fmt::Debug {
    fn render(&self) -> Vec<Text>;
}

#[derive(Node)]
pub struct WidgetRenderer {
    widgets: Vec<Box<dyn Widget>>,
}

impl WidgetRenderer {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: impl Widget + 'static) {
        self.widgets.push(Box::new(widget));
    }
}
