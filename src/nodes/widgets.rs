use crate::renderer::*;
use crate::storage::Node;
use crate::nodes::flex;

pub trait Widget: Node + std::fmt::Debug + flex::Flex {
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
