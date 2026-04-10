use crate::renderer::rewrite::NonNestableWindow;
use crate::{errors, renderer::sub_win_rewrite};
// use crate::renderer::sub_win_rewrite::TextType;

use super::rewrite;
use super::sub_win_rewrite::{SubWindow, TextType};
use crate::renderer::sub_win;
use errors::TextError;

#[derive(Debug, Clone, PartialEq)]
pub struct Window {
    pub texts: Vec<TextType>,
    pub width: u32,
    pub height: u32,
    pub type_of_border: rewrite::TypeOfBorder,
}

// impl From<Window> for NestedWindow {
//     fn from(value: Window) -> Self {
//         Self {
//             texts: value.texts,
//             width: value.width,
//             height: value.height,
//             type_of_border: value.type_of_border,
//         }
//     }
// }

impl From<NonNestableWindow> for Window {
    fn from(value: NonNestableWindow) -> Self {
        Self {
            texts: value
                .texts
                .iter()
                .map(|text| TextType::Text(text.clone()))
                .collect::<Vec<TextType>>(),
            width: value.width,
            height: value.height,
            type_of_border: value.type_of_border,
        }
    }
}

impl Window {
    pub fn render(&self) -> Result<String, TextError> {
        let collapsed: Vec<rewrite::Text> =
            sub_win_rewrite::collapse_window(self.texts.clone(), 0)?;
        dbg!(&collapsed);
        let window = rewrite::NonNestableWindow {
            texts: collapsed,
            height: self.height,
            width: self.width,
            type_of_border: self.type_of_border,
        };
        Ok(window.render()?)
    }
}
