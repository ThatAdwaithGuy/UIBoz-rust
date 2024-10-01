use thiserror::Error;

use crate::sub_win::SubWindow;

#[derive(Error, Debug)]
pub enum TextError {
    #[error("\x1b[91mERROR\x1b[0m: Unhandled error, found some values to be bad and found out when calculating \n(P.S It is always negative ): {0}")]
    UnhandledError(i32),
    #[error("\x1b[91mERROR\x1b[0m: Max depth exceeded for sub window {0:#?}")]
    DepthLimitExceeded(SubWindow),
    #[error("\x1b[91mERROR\x1b[0m: looks like {0} and {1} are overlapping with each other.")]
    TextOverlayed(String, String),
    #[error("\x1b[91mERROR\x1b[0m: {0} is occupying another text.")]
    DuplicateText(String),
    #[error("\x1b[91mERROR\x1b[0m: {0} is leaving the bounds of the screen.")]
    LeftBounds(String),
}
