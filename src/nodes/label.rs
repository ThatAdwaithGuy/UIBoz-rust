use super::super::style::TextStyle;
use super::widgets;
use crate::renderer::{self, *};
use crate::storage::Node;
use node_proc_macro::Node;
#[derive(Node, Clone)]
pub struct Label {
    text: String,
    style: [TextStyle; 12],
}

impl std::fmt::Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Label").field("text", &self.text).finish()
    }
}

impl Label {
    pub fn new(text: &str, style: &[TextStyle]) -> Label {
        assert!(
            style.len() <= 12,
            "The styles argument execded its limit of 12."
        );
        let mut formatted_style = [TextStyle::Bold(false); 12];
        if style.len() == 12 {
            formatted_style = style.try_into().unwrap();
        } else {
            formatted_style[..style.len()].copy_from_slice(style);
        }
        Label {
            text: text.to_string(),
            style: formatted_style,
        }
    }
}

impl crate::nodes::flex::Flex for Label {
    fn flex(&self, width: u32, height: u32) -> Option<Window> {
        if self.text.len() as u32 >= width {
            return None;
        }

        if height == 0 {
            return None;
        }

        Some(Window {
            texts: vec![TextType::Text(Text::new(&self.text, 1, 0, &self.style))],
            width,
            height,
            type_of_border: TypeOfBorder::NoBorders,
        })
    }
}

impl widgets::Widget for Label {
    fn render(&self) -> Window {
        Window {
            texts: vec![TextType::Text(Text {
                text: self.text.clone(),
                line_number: 1,
                column: 0,
                style: self.style,
                no_of_ansi: 0,
            })],
            width: self.text.len() as u32,
            height: 1,
            type_of_border: TypeOfBorder::NoBorders,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::widgets::Widget;
    #[test]
    fn label_test() {
        let label = Label::new("Hello, World!", &[]);
        let rendered = label.render();
        dbg!(rendered);
    }
}
