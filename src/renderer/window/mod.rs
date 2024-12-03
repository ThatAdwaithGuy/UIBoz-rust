use crate::errors;

use super::sub_win::{self, NestedWindow, SubWindow, TextType};
use super::window_renderer;
use super::window_renderer::TypeOfBorder;
use errors::TextError;
#[derive(Debug, PartialEq)]
pub struct Window {
    pub texts: Vec<TextType>,
    pub width: u32,
    pub height: u32,
    pub type_of_border: TypeOfBorder,
}

impl Window {
    pub fn render(&self) -> Result<String, TextError> {
        let nested_window = NestedWindow::new(
            self.texts.clone(),
            self.height,
            self.width,
            self.type_of_border,
        );
        let sub_window = SubWindow::new(nested_window, 0, 0);
        let collapsed = sub_win::collapse_sub_window(sub_window, 0)?;
        let window = window_renderer::NonNestableWindow::new(
            collapsed,
            self.height,
            self.width,
            self.type_of_border,
        );
        Ok(window.render(false)?)
    }
}
