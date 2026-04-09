use crate::renderer::window;
use crate::{
    errors::TextError,
    renderer::rewrite::{self, Text},
};

#[derive(Debug, Clone, PartialEq)]
pub struct SubWindow {
    pub window: window::Window,
    pub line_number: u32,
    pub column: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TextType {
    SubWindow(SubWindow),
    Text(rewrite::Text),
}

pub fn is_nested(text: &Vec<TextType>) -> bool {
    text.iter().any(|x| match x {
        TextType::Text(_) => false,
        TextType::SubWindow(_) => true,
    })
}
pub fn is_one_deep(texts: &Vec<TextType>) -> bool {
    is_nested(texts)
        && !texts.iter().any(|text_type| match text_type {
            TextType::SubWindow(sub_window) => is_nested(&sub_window.window.texts),
            TextType::Text(_) => false,
        })
}

impl SubWindow {
    pub fn new(window: window::Window, line_number: u32, column: u32) -> Self {
        Self {
            window,
            line_number,
            column,
        }
    }

    pub fn convert_to_texts(&self) -> Result<Vec<rewrite::Text>, TextError> {
        let mut texts: Vec<rewrite::Text> = vec![];
        let top_border = match self.window.type_of_border {
            rewrite::TypeOfBorder::NoBorders => "".to_string(),
            rewrite::TypeOfBorder::CurvedBorders => {
                format!("╭{}╮", "─".repeat(self.window.width as usize))
            }
            rewrite::TypeOfBorder::SquareBorders => {
                format!("┌{}┐", "─".repeat(self.window.width as usize))
            }
        };
        texts.push(Text::new_unchecked(
            &top_border,
            self.line_number,
            self.column,
            &[],
        ));
        let bottom_border = match self.window.type_of_border {
            rewrite::TypeOfBorder::NoBorders => "".to_string(),
            rewrite::TypeOfBorder::CurvedBorders => {
                format!("╰{}╯", "─".repeat(self.window.width as usize))
            }
            rewrite::TypeOfBorder::SquareBorders => {
                format!("└{}┘", "─".repeat(self.window.width as usize))
            }
        };
        texts.push(Text::new_unchecked(
            &bottom_border,
            self.line_number + self.window.height,
            self.column,
            &[],
        ));

        for line_number in self.line_number + 1..=(self.window.height + self.line_number - 1) {
            texts.push(Text::new_unchecked("│", line_number, self.column, &[]));
            texts.push(Text::new_unchecked(
                "│",
                line_number,
                self.column + self.window.width + 1,
                &[],
            ));
        }

        for text in self.window.texts.clone() {
            match text {
                TextType::SubWindow(_) => {
                    return Err(TextError::UnhandledError(-1));
                }
                TextType::Text(text) => {
                    dbg!(&text);
                    texts.push(Text::new_unchecked(
                        &text.text,
                        text.line_number + self.line_number + 1,
                        text.column + self.column,
                        &text.style,
                    ));
                }
            }
        }

        Ok(texts)
    }
}

const DEPTH_LIMIT: u32 = 16;

pub fn collapse_window(text_types: Vec<TextType>, depth: u32) -> Result<Vec<Text>, TextError> {
    if depth > DEPTH_LIMIT {
        return Err(TextError::DepthLimitExceeded());
    }
    let mut texts: Vec<Text> = vec![];
    for text_type in text_types {
        match text_type {
            TextType::SubWindow(sub_window) => {
                if is_nested(&sub_window.window.texts) {
                    let collapsed = sub_window.convert_to_texts()?;
                    texts.extend(collapsed);
                } else {
                    let win = collapse_window(sub_window.window.texts, depth + 1)?;
                    texts.extend(win);
                }
            }
            TextType::Text(text) => texts.push(text),
        }
    }

    Ok(texts)
}

