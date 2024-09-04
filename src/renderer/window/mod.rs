use crate::errors::TextError;
use crate::renderer::sub_win::{self, NestedWindow};
use crate::renderer::sub_win::{SubWindow, TextType};
use crate::window_renderer;
use crate::window_renderer::TypeOfBorder;
#[derive(Debug)]
pub struct Window {
    texts: Vec<TextType>,
    width: u32,
    height: u32,
    type_of_border: TypeOfBorder,
}

impl Window {
    pub fn new(
        texts: Vec<TextType>,
        width: u32,
        height: u32,
        type_of_border: TypeOfBorder,
    ) -> Self {
        Self {
            texts: texts,
            width: width,
            height: height,
            type_of_border: type_of_border,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[ignore = "Stuff"]
    #[test]
    fn stuff() {
        let texts: Vec<TextType> = vec![];
        let window = Window::new(texts, 12, 56, TypeOfBorder::CurvedBorders);
        if let Ok(string) = window.render() {
            println!("{}", string);
        }
    }
}
