use thiserror::Error;

use crate::draw_boz::Text;

#[derive(Error, Debug)]
pub enum TextError {
    #[error("\x1b[91mERROR\x1b[0m: looks like {0} and {1} are overlaping with each other.")]
    TextOverlayed(String, String),
    #[error("\x1b[91mERROR\x1b[0m: {0} is occpuying another text.")]
    DuplicateText(String),
    #[error("\x1b[91mERROR\x1b[0m: {0} is leaving the bounds of the screen.")]
    LeftBounds(String),
    #[error("\x1b[91mERROR\x1b[0m: {0:?} is overlaping with a inner Boz")]
    OverlapBoz(Text<'static>),
}
