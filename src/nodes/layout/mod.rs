use super::widgets;
pub mod rendering;
use crate::errors;
use std::collections::HashMap;

/*
    Just a layout for one page.
*/
pub struct LayoutHandler<'a> {
    layout: Layout<'a>,
}

#[derive(Debug, Clone)]
enum Size<const T: usize> {
    Percentage( [u8; T]),
    Chars([u8; T]),
}

#[derive(Clone)]
enum Splits {
    Up,
    Down,
    Left,
    Right,
    Vertical { num: usize, widgets: [usize; 1024] },
    Horizontal { num: usize, widgets: [usize; 1024] },
    Size(Size<  1024  >), // wish rust had enums inside enums. This will be the perfect situation
}

impl std::fmt::Debug for Splits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[derive(Clone, Debug)]
        enum DebugEnum {
            Up,
            Down,
            Left,
            Right,
            Vertical { num: usize },
            Horizontal { num: usize },
            Size(Size<1024>),
        }
        let debug_enum = match self {
            Splits::Up => DebugEnum::Up,
            Splits::Down => DebugEnum::Down,
            Splits::Left => DebugEnum::Left,
            Splits::Right => DebugEnum::Right,
            Splits::Vertical { num, .. } => DebugEnum::Vertical { num: *num },
            Splits::Horizontal { num, .. } => DebugEnum::Horizontal { num: *num },
            Splits::Size(size) => match size {
                Size::Percentage(p) => DebugEnum::Size(Size::Percentage(*p)),
                Size::Chars(p) => DebugEnum::Size(Size::Chars(*p)),
            },
        };

        std::fmt::Debug::fmt(&debug_enum, f)
    }
}

#[derive(Debug)]
pub struct Layout<'a> {
    splits: Vec<Splits>,
    widgets: HashMap<u32, &'a dyn widgets::Widget>,
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

fn val_to_err(split: &Splits) -> errors::LayoutErrors {
    match split {
        Splits::Up => errors::LayoutErrors::Up,
        Splits::Down => errors::LayoutErrors::Down,
        Splits::Left => errors::LayoutErrors::Left,
        Splits::Right => errors::LayoutErrors::Right,
        Splits::Vertical { .. } | Splits::Horizontal { .. } | Splits::Size { .. } => {
            panic!("INTERNAL ERROR")
        }
    }
}

impl<'a> Layout<'a> {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            splits: vec![],
            widgets: HashMap::new(),
            width,
            height,
        }
    }

    fn add_widget(&mut self, widget: &'a dyn widgets::Widget) -> u32 {
        use std::borrow::Borrow;
        let latest: u32 = match self.widgets.keys().max() {
            Some(n) => n + 1,
            None => 0,
        };
        self.widgets.insert(latest, widget);
        latest
    }

    fn extend_widgets(&mut self, widgets: Vec<&'a dyn widgets::Widget>) -> Vec<usize> {
        let mut ret: Vec<usize> = vec![];
        for widget in widgets {
            let idx = self.add_widget(widget);
            ret.push(idx as usize);
        }
        ret
    }

    pub fn vsplit(
        &mut self,
        splits: usize,
        widgets: &'a [Option<&'a dyn widgets::Widget>],
    ) -> &mut Self {
        assert!(splits == widgets.len());
        let vect: Vec<&'a dyn widgets::Widget> = widgets.iter().filter_map(|x| x.clone()).collect();
        let vector = self.extend_widgets(vect);
        let mut indices: [usize; 1024] = [0; 1024];
        indices[0..vector.len().min(1024)].copy_from_slice(&vector);
        self.splits.push(Splits::Vertical {
            num: splits,
            widgets: indices,
        });
        self
    }

    pub fn split(
        &mut self,
        splits: usize,
        widgets: &'a [Option<&'a dyn widgets::Widget>],
    ) -> &mut Self {
        assert!(splits as usize == widgets.len());
        let vect: Vec<&'a dyn widgets::Widget> = widgets.iter().filter_map(|x| x.clone()).collect();
        let vector = self.extend_widgets(vect);
        let mut indices: [usize; 1024] = [0; 1024];
        indices[0..vector.len().min(1024)].copy_from_slice(&vector);
        self.splits.push(Splits::Horizontal {
            num: splits,
            widgets: indices,
        });
        self
    }

    pub fn check(&self) -> Result<(), errors::LayoutErrors> {
        for idx in 0..self.splits.len() - 1 {
            let element = &self.splits[idx];
            let next_element = &self.splits[idx + 1];

            match element {
                Splits::Up | Splits::Down | Splits::Right | Splits::Left => match next_element {
                    Splits::Up | Splits::Down | Splits::Right | Splits::Left => {
                        return Err(val_to_err(element));
                    }
                    Splits::Size { .. } => {}
                    Splits::Vertical { .. } => {}
                    Splits::Horizontal { .. } => {}
                },
                Splits::Vertical { .. } => match next_element {
                    Splits::Up => {}
                    Splits::Down => {}
                    Splits::Left => {}
                    Splits::Right => {}
                    Splits::Size { .. } => {}
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
                    Splits::Size { .. } => {}

                    Splits::Vertical { .. } => {
                        return Err(errors::LayoutErrors::Split);
                    }
                    Splits::Horizontal { .. } => {
                        return Err(errors::LayoutErrors::Split);
                    }
                },
                Splits::Size { .. } => {}
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
}
