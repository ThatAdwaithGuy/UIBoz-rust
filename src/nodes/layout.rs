use super::widgets;
use crate::errors;
use std::collections::HashMap;

type WindowId = usize;

/*
    Just a layout for one page.
*/
struct LayoutHandler<'a> {
    widgets: HashMap<WindowId, Box<(dyn widgets::Widget)>>,
    layout: Layout<'a>,
}
#[derive(Debug)]
enum Splits<'a> {
    Up,
    Down,
    Left,
    Right,
    Vertical {
        num: usize,
        splits: &'a [Option<&'a dyn widgets::Widget>],
    },
    Horizontal {
        num: usize,
        splits: &'a [Option<&'a dyn widgets::Widget>],
    },
}

#[derive(Debug)]
struct Layout<'a> {
    splits: Vec<Splits<'a>>,
    width: u32,
    height: u32,
}

macro_rules! direction_method {
    ($name:ident,$dir:expr) => {
        pub fn $name(&mut self) -> &mut Self {
            self.splits.push($dir);
            self
        }
    };
}

impl<'a> Layout<'a> {
    pub fn new(    width: u32,
    height: u32,) -> Self {

        Self { splits: vec![], width, height }
    }

    pub fn vsplit(
        &mut self,
        splits: usize,
        widgets: &'a [Option<&'a dyn widgets::Widget>],
    ) -> &mut Self {
        assert!(splits == widgets.len());
        self.splits.push(Splits::Vertical {
            num: splits,
            splits: widgets,
        });
        self
    }

    pub fn split(
        &mut self,
        splits: usize,
        widgets: &'a [Option<&'a dyn widgets::Widget>],
    ) -> &mut Self {
        assert!(splits as usize == widgets.len());
        self.splits.push(Splits::Horizontal {
            num: splits,
            splits: widgets,
        });
        self
    }

    pub fn check(&self) -> Result<(), errors::LayoutErrors> {
        for idx in 0..self.splits.len() - 1 {
            let element = &self.splits[idx];
            let next_element = &self.splits[idx + 1];

            match element {
                Splits::Up => match next_element {
                    Splits::Up => {
                        return Err(errors::LayoutErrors::Up);
                    }
                    Splits::Down => {
                        return Err(errors::LayoutErrors::Up);
                    }
                    Splits::Left => {
                        return Err(errors::LayoutErrors::Up);
                    }
                    Splits::Right => {
                        return Err(errors::LayoutErrors::Up);
                    }
                    Splits::Vertical { .. } => {}
                    Splits::Horizontal { .. } => {}
                },
                Splits::Down => match next_element {
                    Splits::Up => {
                        return Err(errors::LayoutErrors::Down);
                    }
                    Splits::Down => {
                        return Err(errors::LayoutErrors::Down);
                    }
                    Splits::Left => {
                        return Err(errors::LayoutErrors::Down);
                    }
                    Splits::Right => {
                        return Err(errors::LayoutErrors::Down);
                    }
                    Splits::Vertical { .. } => {}
                    Splits::Horizontal { .. } => {}
                },
                Splits::Left => match next_element {
                    Splits::Up => {
                        return Err(errors::LayoutErrors::Left);
                    }
                    Splits::Down => {
                        return Err(errors::LayoutErrors::Left);
                    }
                    Splits::Left => {
                        return Err(errors::LayoutErrors::Left);
                    }
                    Splits::Right => {
                        return Err(errors::LayoutErrors::Left);
                    }
                    Splits::Vertical { .. } => {}
                    Splits::Horizontal { .. } => {}
                },
                Splits::Right => match next_element {
                    Splits::Up => {
                        return Err(errors::LayoutErrors::Right);
                    }
                    Splits::Down => {
                        return Err(errors::LayoutErrors::Right);
                    }
                    Splits::Left => {
                        return Err(errors::LayoutErrors::Right);
                    }
                    Splits::Right => {
                        return Err(errors::LayoutErrors::Right);
                    }
                    Splits::Vertical { .. } => {}
                    Splits::Horizontal { .. } => {}
                },
                Splits::Vertical { .. } => match next_element {
                    Splits::Up => {}
                    Splits::Down => {}
                    Splits::Left => {}
                    Splits::Right => {}
                    Splits::Vertical { .. } => {
                        return Err(errors::LayoutErrors::Vsplit);
                    }
                    Splits::Horizontal { .. } => {
                        return Err(errors::LayoutErrors::Vsplit);
                    }
                },
                Splits::Horizontal { .. } => match next_element {
                    Splits::Up => {}
                    Splits::Down => {}
                    Splits::Left => {}
                    Splits::Right => {}

                    Splits::Vertical { .. } => {
                        return Err(errors::LayoutErrors::Split);
                    }
                    Splits::Horizontal { .. } => {
                        return Err(errors::LayoutErrors::Split);
                    }
                },
            }
        }
        Ok(())
    }

    direction_method!(down, Splits::Down);
    direction_method!(up, Splits::Up);
    direction_method!(left, Splits::Left);
    direction_method!(right, Splits::Right);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right() {
        let mut layout = Layout::new(10, 10);
        assert_eq!(
            layout.right().right().check().unwrap_err(),
            errors::LayoutErrors::Right
        );
    }

    #[test]
    fn left() {
        let mut layout = Layout::new(10, 10);
        assert_eq!(
            layout.left().left().check().unwrap_err(),
            errors::LayoutErrors::Left
        );
    }

    #[test]
    fn up() {
        let mut layout = Layout::new(10, 10);
        assert_eq!(
            layout.up().up().check().unwrap_err(),
            errors::LayoutErrors::Up
        );
    }

    #[test]
    fn down() {
        let mut layout = Layout::new(10, 10);
        assert_eq!(
            layout.down().down().check().unwrap_err(),
            errors::LayoutErrors::Down
        );
    }

    #[test]
    fn general_test() {
        let mut layout = Layout::new(10, 10);
        let layout = layout
            .vsplit(2, &[None, None])
            .left()
            .split(3, &[None, None, None]);
        assert_eq!((), layout.check().unwrap())
    }
    #[test]
    fn my_test() {
        let mut layout = Layout::new(10, 10);
        let layout = layout
            .vsplit(2, &[None, None])
            .left()
            .split(3, &[None, None, None]);
    }
}
