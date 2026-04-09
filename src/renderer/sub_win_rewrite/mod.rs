use crate::{
    errors::TextError,
    renderer::{
        rewrite::{self, Text},
        window,
    },
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

fn is_nested(text: &Vec<TextType>) -> bool {
    text.iter().any(|x| match x {
        TextType::Text(_) => false,
        TextType::SubWindow(_) => true,
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

        for line_number in self.line_number+1..=(self.window.height + self.line_number-1) {
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
                crate::renderer::TextType::SubWindow(_) => {
                    return Err(TextError::UnhandledError(-1));
                }
                crate::renderer::TextType::Text(text) => {
                    dbg!(&text);
                    texts.push(Text::new_unchecked(
                        &text.text,
                        text.line_number + self.line_number+1,
                        text.column + self.column+1,
                        &text.style,
                    ));
                }
            }
        }
        dbg!(&texts);

        Ok(texts)
    }
}
