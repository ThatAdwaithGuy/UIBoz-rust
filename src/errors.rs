use thiserror::Error;
#[derive(Error, Debug, PartialEq)]
pub enum LayoutErrors {
    #[error("\x1b[91mERROR\x1b[0m, Looks like you use the down() method wrong")]
    Down,
    #[error("\x1b[91mERROR\x1b[0m, Looks like you use the up() method wrong")]
    Up,
    #[error("\x1b[91mERROR\x1b[0m, Looks like you use the left() method wrong")]
    Left,
    #[error("\x1b[91mERROR\x1b[0m, Looks like you use the right() method wrong ")]
    Right,
    #[error("\x1b[91mERROR\x1b[0m, You used the vsplit() method wrong")]
    Vsplit,

    #[error("\x1b[91mERROR\x1b[0m, You used the split() method wrong")]
    Split,
}

#[derive(Error, Debug)]
pub enum TextError {
    #[error("\x1b[91mERROR\x1b[0m: Unhandled error, found some values to be bad and found out when calculating \n(P.S It is always negative ): {0}")]
    UnhandledError(i32),
    #[error("\x1b[91mERROR\x1b[0m: Max depth exceeded for sub window")]
    DepthLimitExceeded(),
    #[error("\x1b[91mERROR\x1b[0m: looks like {0} and {1} are overlapping with each other.")]
    TextOverlaid(String, String),
    #[error("\x1b[91mERROR\x1b[0m: {0} is occupying another text.")]
    DuplicateText(String),
    #[error("\x1b[91mERROR\x1b[0m: {0} is leaving the bounds of the screen.")]
    LeftBounds(String),
}
