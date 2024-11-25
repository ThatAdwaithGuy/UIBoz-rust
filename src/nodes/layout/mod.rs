use super::widgets;
use rendering::Pane;
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
    Percentage([u8; T]),
    Chars([u8; T]),
}

#[derive(Clone, Debug)]
enum Splits {
    Up,
    Down,
    Left,
    Right,
    Vertical([Option<u32>; 2]),
    Horizontal([Option<u32>; 2]),
    Size(Size<1024>), // wish rust had enums inside enums. This will be the perfect situation
}
#[derive(Debug)]
pub struct Layout<'a> {
    panes: Vec<Splits>,
    widgets: HashMap<u32, &'a dyn widgets::Widget>,
    width: u32,
    height: u32,
}

macro_rules! direction_method {
    ($name:ident,$dir:expr) => {
        pub fn $name(&mut self) -> &mut Self {
            self.panes.push($dir);
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
            panes: vec![],
            widgets: HashMap::new(),
            width,
            height,
        }
    }

    fn add_widget(&mut self, widget: &'a dyn widgets::Widget) -> u32 {
        let latest: u32 = match self.widgets.keys().max() {
            Some(n) => n + 1,
            None => 0,
        };
        self.widgets.insert(latest, widget);
        latest
    }

    pub fn vsplit(&mut self, widgets: [Option<&'a dyn widgets::Widget>; 2]) -> &mut Self {
        let one = match widgets[0] {
            Some(inner) => Some(self.add_widget(inner)),
            None => None,
        };

        let two = match widgets[1] {
            Some(inner) => Some(self.add_widget(inner)),
            None => None,
        };
        self.panes.push(Splits::Vertical([one, two]));
        self
    }

    pub fn split(&mut self, widgets: [Option<&'a dyn widgets::Widget>; 2]) -> &mut Self {
        let one = match widgets[0] {
            Some(inner) => Some(self.add_widget(inner)),
            None => None,
        };

        let two = match widgets[1] {
            Some(inner) => Some(self.add_widget(inner)),
            None => None,
        };

        self.panes.push(Splits::Horizontal([one, two]));
        self
    }

    pub fn check(&self) -> Result<(), errors::LayoutErrors> {
        for idx in 0..self.panes.len() - 1 {
            let element = &self.panes[idx];
            let next_element = &self.panes[idx + 1];

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
        let layout = layout.vsplit([None, None]).left().split([None, None]);
        assert_eq!((), layout.check().unwrap())
    }
}
