use crate::errors;

use super::sub_win::{self, NestedWindow, SubWindow, TextType};
use super::window_renderer::TypeOfBorder;
use super::{window_renderer, Text};
use errors::TextError;
#[derive(Debug, Clone, PartialEq)]
pub struct Window {
    pub texts: Vec<TextType>,
    pub width: u32,
    pub height: u32,
    pub type_of_border: TypeOfBorder,
}

impl From<Window> for NestedWindow {
    fn from(value: Window) -> Self {
        Self {
            texts: value.texts,
            width: value.width,
            height: value.height,
            type_of_border: value.type_of_border,
        }
    }
}

impl Window {
    pub fn render(&self) -> Result<String, TextError> {
        let nested_window = self.clone().into();
        let sub_window = SubWindow::new(nested_window, 0, 0);
        let collapsed = sub_win::collapse_sub_window(sub_window, 0)?;
        dbg!(&collapsed);
        dbg!(collapsed
            .iter()
            .filter(|x| x.text != "")
            .cloned()
            .collect::<Vec<Text>>());
        let window = window_renderer::NonNestableWindow::new(
            collapsed
                .iter()
                .filter(|x| x.text != "")
                .cloned()
                .collect::<Vec<Text>>(),
            self.height,
            self.width,
            self.type_of_border,
        );
        Ok(window.render(true)?)
    }
}
