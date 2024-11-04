use std::{collections::HashMap, marker::PhantomData};

use crate::widgets;


type WindowId = usize;


/*
    Just a layout for one page.
*/
struct LayoutHandler<'a> {
    widgets: HashMap<WindowId, Box<(dyn widgets::Widget)>>,
    layout: Layout<'a>,
}

enum Splits<'a> {
    Up,
    Down,
    Left,
    Right,
    Vertical {
        num: usize,
        splits: &'a [Option<&'a dyn widgets::Widget>]
    },
    Horizontal {
        num: usize,
        splits: &'a [Option<&'a dyn widgets::Widget>]
    }
}

struct Vertical;
struct Horizontal;
struct Start;

struct Layout<'a, S = Start> {
    splits: Vec<Splits<'a>>,
    marker: PhantomData<S>
}

macro_rules! direction_method {
    ($name:ident,$dir:expr) => {
        pub fn $name(&mut self) -> &mut Self {
            self.splits.push($dir);
            self
        }  
    } 
}

impl<'a> Layout<'a> {
    pub fn new() -> Self {
        Self {
            splits: vec![],
            marker: PhantomData,
        }
    }

    pub fn vsplit(&mut self, splits: usize, widgets: &'a [Option<&'a dyn widgets::Widget>]) -> &mut Self {
        assert!(splits  == widgets.len());
        self.splits.push(Splits::Vertical {
            num: splits,
            splits: widgets,
        });
        self
    }

    pub fn split(&mut self, splits: usize, widgets: &'a [Option<&'a dyn widgets::Widget>]) -> &mut Self {
        assert!(splits as usize == widgets.len());
        self.splits.push(Splits::Horizontal{
            num: splits,
            splits: widgets,
        });
        self
    }

    pub fn check(&self) -> Result<(), errors::LayoutErrors> {

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
        let layout = Layout::new().right();

    }
}
