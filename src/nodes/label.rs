use super::super::style::TextStyle;
use super::widgets;
use crate::renderer::window_renderer::Text;
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

impl widgets::Widget for Label {
    fn render(&self) -> Vec<Text> {
        vec![Text {
            text: self.text.clone(),
            line_number: 1,
            column: 0,
            style: self.style,
            no_of_ansi: 0,
        }]
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
