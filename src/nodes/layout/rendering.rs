use super::*;
use crate::renderer::*;

#[derive(Clone)]
struct Pane<'a> {
    val: &'a Splits,
    widgets: [usize; 1024],
    dir: Option<&'a Splits>,
    size: Option<&'a Splits>,
}

fn default_size<const T: usize>() -> Size<T> {
    let p = (100 / T) as u8;
    let arr: [u8; T] = [p; T];
    Size::Percentage(arr)
}

fn default_dir() {}

impl std::fmt::Debug for Pane<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[derive(Debug)]
        struct DebugStruct<'a> {
            val: &'a Splits,
            dir: Option<&'a Splits>,
            size: Option<&'a Splits>,
        }

        let debug_struct = DebugStruct {
            val: self.val,
            dir: self.dir,
            size: self.size,
        };

        std::fmt::Debug::fmt(&debug_struct, f)
    }
}

fn render_pane(pane: Pane) -> Result<Window, errors::TextError> {
    Err(errors::TextError::LeftBounds("hi".to_string()))
}

fn group_split<'a>(splits: Vec<&'a Splits>) -> Vec<Pane> {
    let mut chunks: Vec<Pane> = vec![];
    let mut curr_chuck: Option<Pane> = None;
    for split in splits {
        match split {
            Splits::Up | Splits::Down | Splits::Left | Splits::Right => {
                if let Some(ref mut chuck) = curr_chuck {
                    chuck.dir = Some(split);
                }
            }

            Splits::Size { .. } => {
                if let Some(ref mut chuck) = curr_chuck {
                    chuck.size = Some(split);
                }
            }
            Splits::Vertical { widgets, .. } | Splits::Horizontal { widgets, .. } => {
                if let Some(ref mut chuck) = curr_chuck {
                    chunks.push(chuck.clone());
                    *chuck = Pane {
                        val: split,
                        widgets: *widgets,
                        dir: None,
                        size: None,
                    };
                } else {
                    curr_chuck = Some(Pane {
                        val: &split,
                        widgets: *widgets,
                        dir: None,
                        size: None,
                    });
                }
            }
        }
    }

    if let Some(chuck) = curr_chuck {
        chunks.push(chuck);
    }

    chunks
}

pub(super) fn render_layout(layout: &Layout) {
    let mut panes = group_split(layout.splits.iter().map(|x| x).collect());
    panes.reverse();
    

    
    dbg!(panes);
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::nodes::label::Label;
    #[test]
    fn test_default_values() {
        let value = default_size::<2>();
        match value {
            Size::Percentage(p) => assert_eq!(p, [50,50]),
            Size::Chars(_) => assert!(false),
        }
    }
    #[test]
    fn my_test() {
        let label_1 = Label::new("Hi", &[]);
        let label_2 = Label::new("Hello", &[]);
        let mut layout = Layout::new(10, 10);
        let binding: [Option<&dyn widgets::Widget>; 2] = [None, Some(&label_1)];
        let binding: [Option<&dyn widgets::Widget>; 2] = [None, Some(&label_2)];
        let layout = &(*layout.vsplit(2, &binding).left().split(2, &binding).left());
        dbg!(&layout.splits);
        let _ = render_layout(layout);
    }
}
