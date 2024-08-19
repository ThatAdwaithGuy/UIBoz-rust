//use crate::window::opts::{parse_text_opts, Colors};
pub mod errors;
pub mod raw_window;
pub mod style;
pub mod sub_win;
pub mod window;
fn main() -> Result<(), errors::TextError> {
    let texts = vec![];
    let window = sub_win::NestedWindow::new(texts, 12, 12, raw_window::TypeOfBorder::CurvedBorders);
    Ok(())
}
