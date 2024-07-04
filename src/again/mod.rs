use itertools::Itertools;

use crate::{errors::TextError, style};
mod utils;

#[derive(Clone, Debug)]
pub struct Text {
    pub text: String,
    pub line_number: u32,
    pub column: u32,
    pub style: &'static [style::TextStyle],
    pub no_of_ansi: u32,
}

impl Text {
    pub fn new(
        text: &str,
        line_number: u32,
        column: u32,
        style: &'static [style::TextStyle],
    ) -> Text {
        Text {
            text: text.to_string(),
            line_number,
            column,
            style,
            no_of_ansi: 1,
        }
    }

    pub fn no_of_ansi(&mut self, no_of_ansi: u32) -> Self {
        self.no_of_ansi = no_of_ansi;
        self.to_owned()
    }
}

#[derive(Clone, Debug)]
pub struct Window {
    pub texts: Vec<Text>,
    pub height: u32,
    pub width: u32,
    pub type_of_border: TypeOfBorder,
}

impl Window {
    pub fn new(texts: Vec<Text>, height: u32, width: u32, type_of_border: TypeOfBorder) -> Window {
        Window {
            texts: texts.into(),
            height,
            width,
            type_of_border,
        }
    }

    pub fn render(&self, dbg_mode: bool) -> Result<String, TextError> {
        //dbg!(&self);
        let res =
            utils::replace_none_with_line_numbers(self.height, &utils::handle(self.texts.clone())?);
        //dbg!(&res);
        let texts: String = res
            .iter()
            .map(|x| match x {
                None => match self.type_of_border {
                    TypeOfBorder::CurvedBorders | TypeOfBorder::SquareBorders => {
                        format!("│{}│\n", " ".repeat(self.width as usize))
                    }
                    TypeOfBorder::NoBorders => "\n".to_string(),
                },
                Some(text) => {
                    if dbg_mode {
                        dbg!(
                            text,
                            //text.text.chars().collect::<Vec<char>>().len(),
                            //self.width,
                            //text.text.chars().collect::<Vec<char>>().len(),
                            //78 * text.no_of_ansi as isize,
                            //(text.text.chars().collect::<Vec<char>>().len() as isize)
                            //    - (78 * text.no_of_ansi as isize),
                            self.width as isize
                                - ((text.text.chars().collect::<Vec<char>>().len() as isize)
                                    - (78 * text.no_of_ansi as isize)),
                        );
                    }
                    let calc = self.width as i32
                        - ((text.text.chars().collect::<Vec<char>>().len() as i32)
                            - (78 * text.no_of_ansi as i32));
                    if calc < 0 {
                        panic!("ERROR");
                    }
                    match self.type_of_border {
                        TypeOfBorder::CurvedBorders | TypeOfBorder::SquareBorders => {
                            format!("│{}{}│\n", text.text, " ".repeat(calc as usize))
                        }
                        TypeOfBorder::NoBorders => text.text.clone(),
                    }
                }
            })
            .join("");
        let top_border = match self.type_of_border {
            TypeOfBorder::NoBorders => "\n".to_string(),
            TypeOfBorder::CurvedBorders => {
                format!("╭{}╮\n", "─".repeat(self.width as usize))
            }
            TypeOfBorder::SquareBorders => {
                format!("┌{}┐\n", "─".repeat(self.width as usize))
            }
        };

        let bottom_border = match self.type_of_border {
            TypeOfBorder::NoBorders => "\n".to_string(),
            TypeOfBorder::CurvedBorders => format!("╰{}╯\n", "─".repeat(self.width as usize)),
            TypeOfBorder::SquareBorders => format!("└{}┘\n", "─".repeat(self.width as usize)),
        };

        Ok([top_border, texts, bottom_border].join(""))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TypeOfBorder {
    NoBorders,
    CurvedBorders,
    SquareBorders,
}
#[cfg(test)]
mod tests {
    use crate::errors::TextError;

    use super::*;
    use std::rc::Rc;
    #[ignore]
    #[test]
    fn test() -> Result<(), TextError> {
        let test = vec![Text::new("Hello", 1, 1, &[]), Text::new("World", 1, 7, &[])];
        dbg!(utils::handle(test.clone()));

        let win = Window::new(test.into(), 20, 100, TypeOfBorder::CurvedBorders);
        let res = win.render(false)?;
        println!("{}", res);

        dbg!(style::parse_text_style(Rc::new([])).len());

        Ok(())
    }
}
