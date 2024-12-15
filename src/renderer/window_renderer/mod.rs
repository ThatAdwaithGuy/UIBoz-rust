use itertools::Itertools;
use std::collections::HashSet;

use crate::errors::TextError;
mod utils;
use crate::style;

#[derive(Clone, PartialEq)]
pub struct Text {
    pub text: String,
    pub line_number: u32,
    pub column: u32,
    pub style: [style::TextStyle; 12],
    pub no_of_ansi: u32,
}

impl std::fmt::Debug for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Text")
            .field("text", &self.text)
            .field("line_number", &self.line_number)
            .field("column", &self.column)
            //.field("style", &format_args!("{:?}", &self.no_of_ansi))
            .finish()
    }
}

pub fn empty_styles() -> [style::TextStyle; 12] {
    [style::TextStyle::Bold(false); 12]
}

impl Text {
    pub fn new(text: &str, line_number: u32, column: u32, style: &[style::TextStyle]) -> Text {
        assert!(
            style.len() <= 12,
            "The styles argument execded its limit of 12."
        );
        let mut formatted_style = [style::TextStyle::Bold(false); 12];
        if style.len() == 12 {
            formatted_style = style.try_into().unwrap();
        } else {
            formatted_style[..style.len()].copy_from_slice(style);
        }

        Text {
            text: text.to_string(),
            line_number,
            column,
            style: formatted_style,
            no_of_ansi: 1,
        }
    }

    pub fn no_of_ansi(&mut self, no_of_ansi: u32) -> Self {
        self.no_of_ansi = no_of_ansi;
        self.to_owned()
    }

    pub fn len(&self) -> usize {
        self.text.chars().collect::<Vec<char>>().len()
    }
}

#[derive(Clone, Debug)]
pub(super) struct NonNestableWindow {
    pub texts: Vec<Text>,
    pub height: u32,
    pub width: u32,
    pub type_of_border: TypeOfBorder,
}

fn check_errors(window: &NonNestableWindow) -> Result<(), TextError> {
    let mut seen: HashSet<(u32, u32)> = HashSet::new();

    for text in window.texts.iter() {
        // Check 1
        if seen.contains(&(text.line_number, text.column)) {
            return Err(TextError::TextOverlaid(
                format!(
                    "The conflict is at {} {} and one of the text is {}",
                    text.line_number, text.column, text.text
                ),
                "".to_string(),
            ));
        } else {
            seen.insert((text.line_number, text.column));
        }
        // Check 2
        if (text.column as usize) + text.text.chars().collect::<Vec<char>>().len()
            > (window.width as usize)
        {
            return Err(TextError::LeftBounds(text.text.clone()));
        }
    }

    Ok(())
}

impl NonNestableWindow {
    pub fn new(
        texts: Vec<Text>,
        height: u32,
        width: u32,
        type_of_border: TypeOfBorder,
    ) -> NonNestableWindow {
        NonNestableWindow {
            texts: texts.into(),
            height,
            width,
            type_of_border,
        }
    }

    pub fn render(&self, dbg_mode: bool) -> Result<String, TextError> {
        let handled = utils::handle(self.texts.clone())?;
        let mut texts = String::new();
        let mut up_indicies: Vec<u32> = Vec::new();
        let mut down_indicies: Vec<u32> = Vec::new();

        for item in utils::replace_none_with_line_numbers(self.height, &handled).iter() {
            let line = match item {
                None => match self.type_of_border {
                    TypeOfBorder::CurvedBorders | TypeOfBorder::SquareBorders => {
                        format!("│{}│\n", " ".repeat(self.width as usize))
                    }
                    TypeOfBorder::NoBorders => "\n".to_string(),
                },
                Some(text) => {
                    if dbg_mode {
                        let text_length = text.text.chars().count();
                        let esc_seq_count = text.text.matches("\x1b").count() / 8;
                        let visible_length = text_length - (78 * esc_seq_count) as usize;
                        let _calc = self.width as i32 - visible_length as i32;

                        //dbg!(
                        //    text,
                        //    text_length,
                        //    self.width,
                        //    visible_length,
                        //    78 * esc_seq_count as isize,
                        //    calc,
                        //    self.width as i32 - (visible_length as i32)
                        //);
                    }
                    let text_length = text.text.chars().count();
                    let esc_seq_count = text.text.matches("\x1b").count() / 8;
                    let visible_length = text_length - (78 * esc_seq_count) as usize;
                    let calc = self.width as i32 - visible_length as i32;

                    if calc < 0 {
                        return Err(TextError::UnhandledError(calc));
                    }

                    let mut left_char = "│".to_string();
                    let mut right_char = "│".to_string();
                    for style in text.style {
                        match style {
                            style::TextStyle::LeftSideConnect(true) => {
                                left_char = "├".to_string();
                            }
                            style::TextStyle::RightSideConnect(true) => {
                                right_char = "┤".to_string();
                            }

                            style::TextStyle::UpSideConnect(val) => {
                                if val != -1 {
                                    up_indicies.push(text.column);
                                }
                            }
                            style::TextStyle::DownSideConnect(val) => {
                                if val != -1 {
                                    down_indicies.push(text.column);
                                }
                            }
                            _ => {}
                        }
                    }
                    match self.type_of_border {
                        TypeOfBorder::CurvedBorders | TypeOfBorder::SquareBorders => {
                            format!(
                                "{}{}{}{}\n",
                                left_char,
                                text.text,
                                " ".repeat(calc as usize),
                                right_char
                            )
                        }
                        TypeOfBorder::NoBorders => text.text.clone(),
                    }
                }
            };

            texts.push_str(&line);
        }
        let top_border = match self.type_of_border {
            TypeOfBorder::NoBorders => "\n".to_string(),
            TypeOfBorder::CurvedBorders => {
                let mut border: Vec<String> = vec![];
                for idx in 0..self.width {
                    if up_indicies.contains(&idx) {
                        border.push("┬".to_string());
                    } else {
                        border.push("─".to_string());
                    }
                }
                format!("╭{}╮\n", border.join(""))
            }
            TypeOfBorder::SquareBorders => {
                let mut border: Vec<String> = vec![];
                for idx in 0..self.width {
                    if up_indicies.contains(&idx) {
                        border.push("┬".to_string());
                    } else {
                        border.push("─".to_string());
                    }
                }
                format!("┌{}┐\n", border.join(""))
            }
        };
        dbg!(&down_indicies, &up_indicies);

        let bottom_border = match self.type_of_border {
            TypeOfBorder::NoBorders => "\n".to_string(),
            TypeOfBorder::CurvedBorders => {
                let mut border: Vec<String> = vec![];
                for idx in 0..self.width {
                    if down_indicies.contains(&idx) {
                        border.push("┴".to_string());
                    } else {
                        border.push("─".to_string());
                    }
                }
                format!("╰{}╯\n", border.join(""))
            }
            TypeOfBorder::SquareBorders => {
                let mut border: Vec<String> = vec![];
                for idx in 0..self.width {
                    dbg!(idx);
                    if down_indicies.contains(&idx) {
                        border.push("┴".to_string());
                    } else {
                        border.push("─".to_string());
                    }
                }
                format!("└{}┘\n", border.join(""))
            }
        };
        /*
        let bottom_border = match self.type_of_border {
            TypeOfBorder::NoBorders => "\n".to_string(),
            TypeOfBorder::CurvedBorders => format!("╰{}╯\n", "─".repeat(self.width as usize)),
            TypeOfBorder::SquareBorders => format!("└{}┘\n", "─".repeat(self.width as usize)),
        };*/

        Ok([top_border, texts, bottom_border].join(""))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypeOfBorder {
    NoBorders,
    CurvedBorders,
    SquareBorders,
}
#[cfg(test)]
mod tests {
    use crate::errors::TextError;

    use super::*;
    // OMG THIS SUCKS
    #[ignore = "…"]
    #[test]
    fn window_test() -> Result<(), TextError> {
        let _string = "╭────────────────────╮\n│ \x1b[0000000000000022m\x1b[0000000000000022m\x1b[022m\u{1b}[022m\x1b[022m\x1b[022m\x1b[022m\x1b[022mHello\x1b[0m \x1b[0000000000000022m\x1b[0000000000000022m\x1b[022m\x1b[022m\x1b[022m\u{1b}[022m\x1b[022m\x1b[\
022mWorld\x1b[0m        │\n│                    │\n│                    │\n│                    │\n│                    │\n╰────────────────────╯\n".to_string();

        let test = vec![
            Text::new("Hello", 1, 1, &empty_styles()),
            Text::new("World", 1, 7, &empty_styles()),
        ];
        let window = NonNestableWindow::new(test, 5, 20, TypeOfBorder::CurvedBorders);
        let contents = window.render(false)?;
        assert_eq!(_string, contents);

        Ok(())
    }
}
