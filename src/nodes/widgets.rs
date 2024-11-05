use crate::renderer::window_renderer::Text;
use crate::storage::Node;

pub trait Widget: Node + std::fmt::Debug {
    fn render(&self) -> Vec<Text>;
}

pub struct WidgetRenderer {
    widgets: Vec<Box<dyn Widget>>,
}

impl Node for WidgetRenderer {}

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
