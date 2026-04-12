mod inline;
use crate::renderer;

#[derive(Debug, Clone)]
enum ElementType {
    Text(renderer::Text),
    SubWindow(renderer::SubWindow),
}

impl ElementType {
    pub fn line_number(&self) -> u32 {
        match self {
            ElementType::Text(text) => text.line_number,
            ElementType::SubWindow(sub_window) => sub_window.line_number,
        }
    }

    pub fn column(&self) -> u32 {
        match self {
            ElementType::Text(text) => text.column,
            ElementType::SubWindow(sub_window) => sub_window.column,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Layouts {
    Inline,
    Block,
    Flexbox,
    Grid,
    Columns,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy)]
pub enum Align {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, Copy)]
pub enum Justify {
    Start,
    End,
    Center,
    SpaceBetween,
}

#[derive(Debug, Clone)]
struct Element {
    element: ElementType,
    padding: u32,
    left_padding: u32,
    right_padding: u32,
    top_padding: u32,
    bottom_padding: u32,
}

impl Element {
    fn new(element: ElementType, padding: u32) -> Self {
        Self {
            element,
            padding,
            left_padding: padding,
            right_padding: padding,
            top_padding: padding,
            bottom_padding: padding,
        }
    }

    pub fn left_padding(&mut self, left_padding: u32) {
        self.left_padding = left_padding;
    }

    pub fn right_padding(&mut self, right_padding: u32) {
        self.right_padding = right_padding;
    }

    pub fn top_padding(&mut self, top_padding: u32) {
        self.top_padding = top_padding;
    }

    pub fn bottom_padding(&mut self, bottom_padding: u32) {
        self.bottom_padding = bottom_padding;
    }
}

#[derive(Debug, Clone)]
pub struct Layout {
    elements: Vec<Element>,
    justify: Justify,
    direction: Direction,
    layout: Layouts,
    width: u32,
    height: u32,
}

impl Layout {
    pub fn new(
        elements: Vec<Element>,
        justify: Justify,
        direction: Direction,
        layout: Layouts,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            elements,
            justify,
            direction,
            layout,
            width,
            height,
        }
    }
}
