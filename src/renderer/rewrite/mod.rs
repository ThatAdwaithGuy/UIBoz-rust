use std::{collections::HashSet, ops::Range};

use itertools::Itertools;

use crate::{errors::TextError, style::{self, TextStyle}};


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypeOfBorder {
    NoBorders,
    CurvedBorders,
    SquareBorders,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Text {
    text: String,
    line_number: u32,
    column: u32,
    style: [TextStyle; 12],
    no_of_ansi: u32,
}

impl Text {
    fn new(text: &str, line_number: u32, column: u32, style: &[style::TextStyle]) -> Option<Self> {
        if style.len() <= 12 {
            return None;
        }
        let mut new_style: [style::TextStyle; 12] = [style::TextStyle::Bold(false); 12];
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

    fn new_unchecked(text: &str, line_number: u32, column: u32, style: &[style::TextStyle]) -> Self {
        if style.len() <= 12 {
            panic!("Style is out of bounds");
        }
        let mut new_style: [style::TextStyle; 12] = [style::TextStyle::Bold(false); 12];
        if new_style.len() == 12 {
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

    fn len(&self) -> usize {
        self.text.chars().try_len().unwrap_or(0)
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
    fn check_errors(&self) -> Result<(), TextError> {
        self.texts.iter().chunk_by(|x| x.line_number()).into_iter().map(|x| x.1.into_iter()).map(|x| x.map(|y| dbg!(y)));
        
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn rewrite_test() {
        let mut texts: Vec<Text> = Vec::new();
        texts.push(Text::new_unchecked("Hi", 1, 0, &[]));
        let window = NonNestableWindow {
            texts,
            height: 12,
            width: 56, 
            type_of_border: TypeOfBorder::CurvedBorders,
        };
        window.check_errors().expect("HAHA");
    }
}   
