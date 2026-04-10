use super::widgets;
use crate::nodes::flex::Flex;
use crate::renderer::SubWindow;
use crate::renderer::TextType;
use crate::renderer::Window;
use crate::rewrite::TypeOfBorder;
use std::collections::HashMap;

pub struct Layout {
    widgets: HashMap<u32, Box<dyn widgets::Widget>>,
}

impl Layout {
    fn new() -> Self {
        Layout {
            widgets: HashMap::new(),
        }
    }

    fn add_widget(&mut self, widget: impl widgets::Widget + 'static) {
        let max_id = self.widgets.keys().map(|x| *x as i32).max().unwrap_or(-1);
        self.widgets.insert((max_id + 1) as u32, Box::new(widget));
    }

    // Returns None if the rect's window_id is invalid
    fn render_rect(&self, rect: Rect) -> Option<Window> {
        let widget = &self.widgets.get(&rect.widget_id)?;
        let texts = widget.render().texts;
        Some(Window {
            texts,
            width: rect.width,
            height: rect.height,
            type_of_border: TypeOfBorder::No,
        })
    }

    fn render_split(&self, split: Split) -> Option<Window> {
        match split {
            Split::NoSplit(rect) => self.render_rect(rect),
            Split::Veritcal {
                left,
                right,
                height,
                width,
            } => {
                let mut texts = Vec::new();
                let left_win = match left {
                    Some(l) => {
                        let rect = self.render_rect(l)?;
                        rect.flex(width / 2, height)?
                    }
                    None => Window {
                        texts: Vec::new(),
                        width: width / 2,
                        height,
                        type_of_border: TypeOfBorder::No,
                    },
                };

                let right_win = match right {
                    Some(rect) => {
                        let rect = self.render_rect(rect)?;
                        rect.flex(width / 2, height)?
                    }
                    None => Window {
                        texts: Vec::new(),
                        width: width / 2,
                        height,
                        type_of_border: TypeOfBorder::No,
                    },
                };

                dbg!(&right_win, &left_win);

                let left_sub_win = SubWindow::new(left_win.into(), 1, 0);
                let right_sub_win = SubWindow::new(right_win.into(), 1, width / 2);
                texts.push(TextType::SubWindow(right_sub_win));
                texts.push(TextType::SubWindow(left_sub_win));

                Some(Window {
                    texts,
                    width,
                    height,
                    type_of_border: TypeOfBorder::No,
                })
            }

            Split::Horizontal {
                left,
                right,
                height,
                width,
            } => {
                let mut texts = Vec::new();
                let left_win = match left {
                    Some(l) => {
                        let mut rect = self.render_rect(l)?;
                        rect.width = width;
                        rect.height = height / 2;
                        rect
                    }
                    None => Window {
                        texts: Vec::new(),
                        width,
                        height: height / 2,
                        type_of_border: TypeOfBorder::No,
                    },
                };
                let left_sub_win = SubWindow::new(left_win.into(), 1, 0);

                let right_win = match right {
                    Some(rect) => {
                        let mut rect = self.render_rect(rect)?;
                        rect.width = width;
                        rect.height = height / 2;
                        rect
                    }
                    None => Window {
                        texts: Vec::new(),
                        width,
                        height: height / 2,
                        type_of_border: TypeOfBorder::No,
                    },
                };
                let right_sub_win = SubWindow::new(right_win.into(), (height / 2) + 1, width);

                dbg!(&right_sub_win, &left_sub_win);
                texts.push(TextType::SubWindow(right_sub_win));
                texts.push(TextType::SubWindow(left_sub_win));

                Some(Window {
                    texts,
                    width,
                    height,
                    type_of_border: TypeOfBorder::No,
                })
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rect {
    height: u32,
    width: u32,
    widget_id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Split {
    NoSplit(Rect),
    Veritcal {
        left: Option<Rect>,
        right: Option<Rect>,
        height: u32,
        width: u32,
    },
    Horizontal {
        left: Option<Rect>,
        right: Option<Rect>,
        height: u32,
        width: u32,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{nodes::label::Label, rewrite::TypeOfBorder, Text};
    #[test]
    fn rect_test_neg() {
        let rect = Rect {
            height: 0,
            width: 0,
            widget_id: 0,
        };
        let layout = Layout::new();
        assert!(matches!(layout.render_rect(rect), None))
    }

    #[test]
    fn rect_test_pos() {
        let rect = Rect {
            height: 1,
            width: 3,
            widget_id: 0,
        };
        let label = Label::new("Hi", &[]);
        let mut layout = Layout::new();
        layout.add_widget(label);
        let rend_rect = layout.render_rect(rect).unwrap();
        let win = Window {
            texts: vec![TextType::Text(Text::new_unchecked("Hi", 1, 0, &[]))],
            width: 3,
            height: 1,
            type_of_border: TypeOfBorder::No,
        };
        assert_eq!(rend_rect, win);
    }

    #[test]
    #[ignore]
    fn mine_test() {
        let rect = Rect {
            height: 1,
            width: 3,
            widget_id: 0,
        };

        let bect = Rect {
            height: 1,
            width: 3,
            widget_id: 1,
        };

        let label = Label::new("Hi", &[]);
        let mut layout = Layout::new();

        layout.add_widget(label.clone());
        layout.add_widget(label.clone());

        let split = Split::Veritcal {
            left: Some(rect),
            right: Some(bect),
            height: 12,
            width: 56,
        };

        let mut texts: Vec<TextType> = Vec::new();
        texts.push(TextType::SubWindow(SubWindow::new(
            layout.render_rect(rect).unwrap().into(),
            1,
            0,
        )));

        texts.push(TextType::SubWindow(SubWindow::new(
            {
                let mut win = layout.render_rect(bect).unwrap().into();
                dbg!(&win);

                win
            },
            1,
            0,
        )));

        let win = Window {
            texts,
            width: 56,
            height: 12,
            type_of_border: TypeOfBorder::Curved,
        };
        println!("{}", win.render().unwrap());
    }
}
