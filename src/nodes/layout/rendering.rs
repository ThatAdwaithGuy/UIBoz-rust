use super::*;

#[derive(Clone)]
pub struct Pane<'a> {
    val: &'a Splits,
    _widgets: [usize; 1024],
    dir: Option<&'a Splits>,
    size: Option<&'a Splits>,
}

fn default_size<const T: usize>() -> Size<T> {
    let p = (100 / T) as u8;
    let arr: [u8; T] = [p; T];
    Size::Percentage(arr)
}

impl std::fmt::Debug for Pane<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[derive(Debug)]
        struct DebugStruct<'a> {
            _val: &'a Splits,
            _dir: Option<&'a Splits>,
            _size: Option<&'a Splits>,
        }

        let debug_struct = DebugStruct {
            _val: self.val,
            _dir: self.dir,
            _size: self.size,
        };

        std::fmt::Debug::fmt(&debug_struct, f)
    }
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
                        _widgets: *widgets,
                        dir: None,
                        size: None,
                    };
                } else {
                    curr_chuck = Some(Pane {
                        val: &split,
                        _widgets: *widgets,
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

pub(super) fn _render_layout(layout: &Layout) {
    let mut panes = group_split(layout.panes.iter().map(|x| x).collect());
    panes.reverse();

    for pane in &panes {
        match pane.val {
            Splits::Vertical(w) => todo!(),
            Splits::Horizontal(w) => todo!(),
            _ => {
                panic!("AHHHHH");
            }
        }
    }

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
            Size::Percentage(p) => assert_eq!(p, [50, 50]),
            Size::Chars(_) => assert!(false),
        }
    }

    #[test]
    fn my_test() {
        let label_2 = Label::new("Hello", &[]);
        let mut layout = Layout::new(10, 10);
        let binding: [Option<&dyn widgets::Widget>; 2] = [None, Some(&label_2)];
        let layout = &(*layout.vsplit(2, &binding).left().split(2, &binding).left());
        dbg!(&layout.panes);
        let _ = _render_layout(layout);
    }
}
