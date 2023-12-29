use crate::draw_screen;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum TextError {
    #[error("\x1b[91mERROR\x1b[0m: looks like {0} and {1} are overlaping with each other.")]
    TextOverlayed(String, String),
    #[error("\x1b[91mERROR\x1b[0m: {0} is occpuying another text.")]
    DuplicateText(String),
    #[error("\x1b[91mERROR\x1b[0m: {0} is leaving the bounds of the screen.")]
    LeftBounds(String),
    #[error("\x1b[91mERROR\x1b[0m: {0:#?} is overlaping with a inner Boz")]
    OverlapBoz(draw_screen::Text),
    //#[error("\x1b[91mERROR\x1b[0m: {0:#?} is nested 4 or more times which is the limit.")]
    //OverNestedBoz(Vec<draw_screen::LineType>),
    #[error("\x1b[91mERROR\x1b[0m: This is a error with my internal function convert_one_boz_to_text. Please report this issue to the issue tab in \n the github reop")]
    InternalConvertOneBozToTextError(),
}
