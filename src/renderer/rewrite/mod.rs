use core::fmt;
use itertools::Itertools;
use std::collections::HashSet;
mod util;
use crate::{
    errors::TextError,
    style::{self, TextStyle},
};

// Describes the type of border for a window.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypeOfBorder {
    No,
    Curved,
    Square,
}

// Text is the most primitive part of a window, it contains the data for the individual text and
// lines that is rendered to the final terminal.
#[derive(Clone, PartialEq)]
pub struct Text {
    pub text: String,
    pub line_number: u32,
    pub column: u32,
    pub style: [TextStyle; 12],
    // NOTE: This is deprecated, Please refactor the code to remove this.
    pub no_of_ansi: u32,
}

// Implemented this custom debug because the text styles display unnessasary data, which culters
// the screen for me.
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
    // Creates a new Text, and returns None if error.
    pub fn new(
        text: &str,
        line_number: u32,
        column: u32,
        style: &[style::TextStyle],
    ) -> Option<Self> {
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

    // Creates a new Text, and panics if a error is detected.
    pub fn new_unchecked(
        text: &str,
        line_number: u32,
        column: u32,
        style: &[style::TextStyle],
    ) -> Self {
        if style.len() > 12 && style.len() != 0 {
            panic!("Style is longer than expected, len: {}", style.len());
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

    // Finds the length of the visible part of the text, if the text contains ANSI codes.
    fn text_len(&self) -> usize {
        let mut len = 0;
        let mut in_escape = false;
        for c in self.text().chars() {
            match c {
                '\x1b' => in_escape = true,
                'm' if in_escape => in_escape = false,
                _ if !in_escape => len += 1,
                _ => {}
            }
        }
        len
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

    pub fn no_of_ansi(&mut self, no_of_ansi: u32) -> &mut Self {
        self.no_of_ansi = no_of_ansi;
        self
    }
}

// Window containing only texts, which can be rendered to the terminal.
#[derive(Clone, Debug)]
pub struct NonNestableWindow {
    pub texts: Vec<Text>,
    pub height: u32,
    pub width: u32,
    pub type_of_border: TypeOfBorder,
}

impl NonNestableWindow {
    pub fn render(&self) -> Result<String, TextError> {
        // Checks any invalid input and exits early.
        Self::check_errors(&self)?;
        // Applies ANSI styles to the text.
        let applied_style: Vec<Text> = util::apply_style(&self.texts);
        // Chunks the text according to line number
        let chunks = util::chunk_texts(&applied_style);
        // Adds the whitespace between text in the same, line. So if the joined like String1 +
        // String2 + String3 = the expected string.
        let padded: Vec<Vec<Text>> = chunks.iter().map(util::find_padding_size).collect();
        // This combines the lines in the window into a single string.
        //        Text ,  Line_no, numbers of texts in the line
        let combined: Vec<(String, usize, usize)> = padded
            .iter()
            .map(|line: &Vec<Text>| {
                let (string, line_number) = util::apply_padding_size_and_combine(&line);
                (string, line_number, line.len())
            })
            .collect();
        // The body of the entire window, split to lines
        let mut body: Vec<String> = vec![];

        for line_number in 0..=self.height {
            if let Some(line) = combined.iter().find(|line| line.1 == line_number as usize) {
                // This part calculates the white space between the last text of the line to the
                // right border character.
                let total_length = line.0.chars().count();
                let ansi_length = line.2 * 234;
                let text_length = total_length - ansi_length;
                let left_pad: usize = self.width as usize - text_length;

                let body_line = match self.type_of_border {
                    TypeOfBorder::Curved | TypeOfBorder::Square => {
                        format!("│{}{}│\n", line.0, " ".repeat(left_pad),)
                    }
                    TypeOfBorder::No => line.0.clone(),
                };

                body.push(body_line);
            } else {
                let line = match self.type_of_border {
                    TypeOfBorder::Curved | TypeOfBorder::Square => {
                        format!("│{}│\n", " ".repeat(self.width as usize))
                    }
                    TypeOfBorder::No => "\n".to_string(),
                };

                body.push(line);
            }
        }
        let top_border = match self.type_of_border {
            TypeOfBorder::No => "\n".to_string(),
            TypeOfBorder::Curved => format!("╭{}╮\n", "─".repeat(self.width as usize)),
            TypeOfBorder::Square => format!("┌{}┐\n", "─".repeat(self.width as usize)),
        };
        let bottom_border = match self.type_of_border {
            TypeOfBorder::No => "\n".to_string(),
            TypeOfBorder::Curved => format!("╰{}╯\n", "─".repeat(self.width as usize)),
            TypeOfBorder::Square => format!("└{}┘\n", "─".repeat(self.width as usize)),
        };

        Ok([top_border, body.join(""), bottom_border].join(""))
    }

    fn overlaps(a: &Text, b: &Text) -> bool {
        if a.line_number != b.line_number {
            return false;
        }

        let a_start = a.column;
        let a_end = a.column + a.text_len() as u32;

        let b_start = b.column;
        let b_end = b.column + b.text_len() as u32;

        a_start < b_end && b_start < a_end
    }

    fn overlap_check(&self) -> Result<(), TextError> {
        let chunked = util::chunk_texts(&self.texts);
        for line in chunked {
            for pair in line.windows(2) {
                let a = pair[0];
                let b = pair[1];

                dbg!(&line, &pair, Self::overlaps(a, b));
                if Self::overlaps(a, b) {
                    return Err(TextError::TextOverlaid(a.text.clone(), b.text.clone()));
                }
            }
        }
        Ok(())
    }

    fn bounds_check(&self) -> Result<(), TextError> {
        for text in &self.texts {
            let end = text.text_len() + text.column as usize;
            if end > self.width as usize {
                return Err(TextError::LeftBounds(text.text.clone()));
            }
        }
        Ok(())
    }

    fn check_errors(&self) -> Result<(), TextError> {
        self.overlap_check()?;
        self.bounds_check()?;
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
            type_of_border: TypeOfBorder::Curved,
        };
        if let Err(err) = window.check_errors() {
            assert!(false, "err {:#?}", err);
        }
        assert!(true);
    }

    #[test]
    fn error_check_overlaps_function() {
        let a = Text::new_unchecked("hi", 1, 0, &TextStyleBuilder::new().build());
        let b = Text::new_unchecked("bye", 1, 1, &TextStyleBuilder::new().build());
        assert!(NonNestableWindow::overlaps(&a, &b));
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
            type_of_border: TypeOfBorder::Curved,
        };
        let err = window.check_errors();
        assert!(err.is_err(), "Expected error but got Ok");
    }

    #[test]
    fn check_for_bounds_overlap() {
        let texts = vec![Text::new_unchecked("hi you idiot", 0, 50, &[])];
        let window = NonNestableWindow {
            texts,
            height: 12,
            width: 56,
            type_of_border: TypeOfBorder::Curved,
        };
        let err = window.check_errors();

        assert!(err.is_err(), "Expected error but got Ok");
    }

    #[test]
    fn error_check_for_bounds_overlap() {
        let texts = vec![Text::new_unchecked("hi you idiot", 0, 40, &[])];
        let window = NonNestableWindow {
            texts,
            height: 12,
            width: 56,
            type_of_border: TypeOfBorder::Curved,
        };
        let err = window.check_errors();

        assert!(err.is_ok(), "Expected Ok but got Err");
    }
}
