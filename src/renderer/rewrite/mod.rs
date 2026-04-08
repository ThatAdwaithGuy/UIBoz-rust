use core::fmt;
use itertools::Itertools;
use std::collections::HashSet;
mod util;
use crate::{
    errors::TextError,
    style::{self, TextStyle},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypeOfBorder {
    NoBorders,
    CurvedBorders,
    SquareBorders,
}

#[derive(Clone, PartialEq)]
pub struct Text {
    text: String,
    line_number: u32,
    column: u32,
    style: [TextStyle; 12],
    no_of_ansi: u32,
}
impl fmt::Debug for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Start the struct formatting
        let mut debug_struct = f.debug_struct("Text");

        // Add regular fields
        debug_struct
            .field("text", &self.text)
            .field("line_number", &self.line_number)
            .field("column", &self.column)
            .field("no_of_ansi", &self.no_of_ansi);

        // Filter and display only non-blank styles
        let non_blank_styles: Vec<&TextStyle> = self
            .style
            .iter()
            .filter(|&style| style != &TextStyle::Blank)
            .collect();

        // Add the filtered styles
        debug_struct.field("style", &non_blank_styles);

        // Finish the formatting
        debug_struct.finish()
    }
}

impl Text {
    fn new(text: &str, line_number: u32, column: u32, style: &[style::TextStyle]) -> Option<Self> {
        if style.len() > 12 && style.len() != 0 {
            return None;
        }
        let mut new_style: [style::TextStyle; 12] = [style::TextStyle::Blank; 12];
        if new_style.len() == 12 {
            new_style = style.try_into().ok()?;
        } else {
            new_style[..style.len()].copy_from_slice(style);
        }

        return Some(Self {
            text: text.to_string(),
            line_number,
            column,
            style: new_style,
            no_of_ansi: 1,
        });
    }

    fn new_unchecked(
        text: &str,
        line_number: u32,
        column: u32,
        style: &[style::TextStyle],
    ) -> Self {
        if style.len() > 12 && style.len() != 0 {
            panic!("Style is out of bounds, len: {}", style.len());
        }
        let mut new_style: [style::TextStyle; 12] = [style::TextStyle::Blank; 12];
        if style.len() == 12 {
            new_style = style.try_into().expect("Rebounds error");
        } else {
            new_style[..style.len()].copy_from_slice(style);
        }

        return Self {
            text: text.to_string(),
            line_number,
            column,
            style: new_style,
            no_of_ansi: 1,
        };
    }

    // With ANSI codes, normal text.len()'s value is wrong, that's the reason for this function.
    pub fn len(&self) -> usize {
        self.text.chars().collect::<Vec<char>>().len()
    }

    // Length of the absolute text, without the style
    pub(super) fn text_len(&self) -> usize {
        self.text().matches("\x1b").count() / 8
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn line_number(&self) -> u32 {
        self.line_number
    }
    pub fn column(&self) -> u32 {
        self.column
    }

    pub fn style(&self) -> [TextStyle; 12] {
        self.style
    }
}

#[derive(Clone, Debug)]
pub(super) struct NonNestableWindow {
    pub texts: Vec<Text>,
    pub height: u32,
    pub width: u32,
    pub type_of_border: TypeOfBorder,
}

impl NonNestableWindow {
    // TODO: Complete this function
    fn duplicate_check(&self) -> Result<(), TextError> {
        let binding = self.texts.iter().chunk_by(|x| x.line_number());
        let flat = binding
            .into_iter()
            .map(|x| {
                x.1.map(|t| ((t.column())..(t.column() + (t.len() as u32))).collect_vec())
                    .collect::<Vec<Vec<u32>>>()
            })
            .collect_vec();
        dbg!(&flat);
        for nums in flat {
            let mut set = HashSet::new();
            for (idx, num) in nums.iter().enumerate() {
                if !set.insert(num) {
                    return Err(TextError::DuplicateText(format!(
                        "duplicate overlaps at line {} and values {:#?}",
                        idx, num
                    )));
                }
            }
        }

        Ok(())
    }
    fn check_errors(&self) -> Result<(), TextError> {
        if let Err(err) = self.duplicate_check() {
            return Err(err);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use style::TextStyleBuilder;

    use super::*;
    #[test]
    fn error_check_for_window_postive() {
        let texts = vec![
            Text::new_unchecked("hi", 1, 0, &TextStyleBuilder::new().build()),
            Text::new_unchecked("bye", 1, 5, &TextStyleBuilder::new().build()),
            Text::new_unchecked("hi", 2, 0, &TextStyleBuilder::new().build()),
            Text::new_unchecked("bye", 2, 5, &TextStyleBuilder::new().build()),
        ];
        let window = NonNestableWindow {
            texts,
            height: 12,
            width: 56,
            type_of_border: TypeOfBorder::CurvedBorders,
        };
        if let Err(err) = window.check_errors() {
            assert!(false, "err {:#?}", err);
        }
        assert!(true);
    }

    #[test]
    fn error_check_for_window_negative() {
        let texts = vec![
            Text::new_unchecked("hi", 1, 0, &TextStyleBuilder::new().build()),
            Text::new_unchecked("bye", 1, 1, &TextStyleBuilder::new().build()),
            Text::new_unchecked("hi", 2, 0, &TextStyleBuilder::new().build()),
            Text::new_unchecked("bye", 2, 5, &TextStyleBuilder::new().build()),
        ];
        let window = NonNestableWindow {
            texts,
            height: 12,
            width: 56,
            type_of_border: TypeOfBorder::CurvedBorders,
        };
        if let Err(_) = window.check_errors() {
            assert!(true);
        }
        assert!(false);
    }
}
