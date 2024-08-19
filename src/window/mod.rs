use crate::errors::TextError;
use crate::raw_window;
use crate::raw_window::TypeOfBorder;
use crate::sub_win::{self, NestedWindow};
use crate::sub_win::{SubWindow, TextType};

struct Window {
    texts: Vec<TextType>,
    width: u32,
    height: u32,
    type_of_border: TypeOfBorder,
}

impl Window {
    fn new(texts: Vec<TextType>, width: u32, height: u32, type_of_border: TypeOfBorder) -> Self {
        Self {
            texts: texts,
            width: width,
            height: height,
            type_of_border: type_of_border,
        }
    }
    fn render(&self) -> Result<String, TextError> {
        let nested_window = NestedWindow::new(
            self.texts.clone(),
            self.height,
            self.width,
            self.type_of_border,
        );
        dbg!(&nested_window);
        let sub_window = SubWindow::new(nested_window, 0, 0);
        dbg!(&sub_window);
        let collapsed = sub_win::collapse_subwindow(sub_window)?;
        dbg!(&collapsed);
        let window = raw_window::NonNestableWindow::new(
            collapsed,
            self.height,
            self.width,
            self.type_of_border,
        );
        dbg!(&window);
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
