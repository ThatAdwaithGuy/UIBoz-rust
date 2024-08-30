use std::{ops::Sub, vec};

use raw_window::Text;
use sub_win::{NestedWindow, SubWindow, TextType};

//use crate::window::opts::{parse_text_opts, Colors};
pub mod errors;
pub mod raw_window;
pub mod style;
pub mod sub_win;
pub mod window;
fn main() -> Result<(), errors::TextError> {
    let inner_window = NestedWindow::new(
        vec![TextType::Text(Text::new("!", 1, 0, &[]))],
        1,
        1,
        raw_window::TypeOfBorder::CurvedBorders,
    );
    let inner_sub = SubWindow::new(inner_window, 1, 0);
    let outer = window::Window::new(
        vec![
            TextType::SubWindow(inner_sub),
            TextType::Text(Text::new("Hello", 1, 10, &[])),
        ],
        56,
        12,
        raw_window::TypeOfBorder::CurvedBorders,
    );
    let string = outer.render()?;
    println!("{}", string);

    Ok(())
}
