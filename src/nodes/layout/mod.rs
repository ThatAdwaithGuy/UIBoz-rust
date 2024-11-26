use super::widgets;
use crate::{renderer::Window, TextType};
use std::collections::HashMap;

pub struct Layout {
    widgets: HashMap<u32, Box<dyn widgets::Widget>>,
}

impl Layout {
    fn new() -> Self {
        Layout {
            widgets: HashMap::new(),
        }
    }
    
    fn add_widget(&mut self, widget: impl widgets::Widget + 'static) {
        let max_id = self.widgets.keys().map(|x| *x as i32).max().unwrap_or(-1);
        self.widgets.insert((max_id + 1) as u32, Box::new(widget));
    }

    // Returns None if the rect's window_id is invalid
    fn render_rect(&self, rect: Rect) -> Option<Window> {
        let widget = &self.widgets.get(&rect.widget_id)?;
        let texts = widget
            .render()
            .iter()
            .map(|x| TextType::Text(x.clone()))
            .collect::<Vec<TextType>>();
        Some(Window {
            texts,
            width: rect.width,
            height: rect.height,
            type_of_border: crate::TypeOfBorder::NoBorders,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rect {
    height: u32,
    width: u32,
    widget_id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Split {
    NoSplit(Option<Rect>),
    Vertical((Option<Rect>, Option<Rect>)),
    Horizontal((Option<Rect>, Option<Rect>)),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::label::Label;
    #[test]
    fn rect_test_neg() {
        let rect = Rect {
            height: 0,
            width: 0,
            widget_id: 0,
        };
        let layout = Layout::new();
        assert!(matches!(layout.render_rect(rect), None))
    }

    #[test]
    fn rect_test_pos() {
        let rect = Rect {
            height: 1,
            width: 3,
            widget_id: 0,
        };
        let label = Label::new("Hi", &[]);
        let mut layout = Layout::new();
        layout.add_widget(label);
    }
}
