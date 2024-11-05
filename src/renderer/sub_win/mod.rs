//pub mod new_mod;
use super::window_renderer;
use crate::errors::TextError;
use std::collections::HashMap;
use std::vec;
type Texts = Vec<TextType>;
#[derive(Clone, Debug)]
pub struct NestedWindow {
    texts: Texts,
    height: u32,
    width: u32,
    type_of_border: window_renderer::TypeOfBorder,
}

impl NestedWindow {
    pub fn new(
        texts: Texts,
        height: u32,
        width: u32,
        type_of_border: window_renderer::TypeOfBorder,
    ) -> Self {
        Self {
            texts,
            height,
            width,
            type_of_border,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubWindow {
    window: NestedWindow,
    start_line_number: u32,
    column: u32,
    // {Line_number: No_Of_Ansi_Codes}
    ansi_codes_map: HashMap<u32, u32>,
}
impl SubWindow {
    pub fn new(window: NestedWindow, start_line_number: u32, column: u32) -> Self {
        Self {
            window,
            start_line_number,
            column,
            ansi_codes_map: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum TextType {
    SubWindow(SubWindow),
    Text(window_renderer::Text),
}

fn is_nested(text: &Vec<TextType>) -> bool {
    text.iter().any(|x| match x {
        TextType::Text(_) => false,
        TextType::SubWindow(_) => true,
    })
}

pub fn collapse_one_deep_sub_window(
    win: SubWindow,
) -> Result<Vec<window_renderer::Text>, TextError> {
    let mut texts: Vec<window_renderer::Text> = Vec::new();
    // Depth check
    if win.window.texts.iter().any(|x| match x {
        TextType::Text(text) => {
            texts.push(text.clone());
            false
        }
        TextType::SubWindow(_) => true,
    }) {
        return Err(TextError::UnhandledError(-1));
    }
    let window = window_renderer::NonNestableWindow::new(
        texts,
        win.window.height,
        win.window.width,
        win.window.type_of_border,
    );
    let mut return_val: Vec<window_renderer::Text> = vec![];
    let rendered_string = window.render(false)?;
    let split = rendered_string.split("\n");
    for (idx, line) in split.enumerate() {
        let count = (line.matches("\x1b").count() / 8) as u32;
        return_val.push(
            {
                let line_number = (idx + 1) as u32;
                let style: &[style::TextStyle] = &window_renderer::empty_styles();
                assert!(
                    style.len() <= 12,
                    "The styles argument execded its limit of 12."
                );
                let mut formatted_style = [style::TextStyle::Bold(false); 12];
                if style.len() == 12 {
                    formatted_style = style.try_into().unwrap();
                } else {
                    formatted_style[..style.len()].copy_from_slice(style);
                }

                Text {
                    text: line.to_string(),
                    line_number,
                    column: 0,
                    style: formatted_style,
                    no_of_ansi: 1,
                }
            }
                .no_of_ansi(count + 1),
        );
    }
    Ok(return_val)
}
const MAX_DEPTH: u32 = 128;
pub fn collapse_sub_window(
    win: SubWindow,
    depth: u32,
) -> Result<Vec<window_renderer::Text>, TextError> {
    let mut res: Vec<window_renderer::Text> = Vec::new();
    if depth >= MAX_DEPTH {
        return Err(TextError::DepthLimitExceeded());
    }

    for text_type in win.window.texts {
        match text_type {
            TextType::Text(t) => res.push(t.to_owned()),
            TextType::SubWindow(sub_win) => match is_nested(&sub_win.window.texts) {
                true => {
                    let collapsed = collapse_sub_window(sub_win, depth + 1)?;

                    let window = window_renderer::NonNestableWindow::new(
                        collapsed,
                        win.window.height,
                        win.window.width,
                        win.window.type_of_border,
                    );
                    let mut return_val: Vec<window_renderer::Text> = vec![];
                    let rendered_string = window.render(false)?;
                    let split = rendered_string.split("\n");
                    for (idx, line) in split.enumerate() {
                        let count = (line.matches("\x1b").count() / 8) as u32;
                        return_val.push(
                            {
                                let line_number = (idx + 1) as u32;
                                let style: &[style::TextStyle] = &window_renderer::empty_styles();
                                assert!(
                                    style.len() <= 12,
                                    "The styles argument execded its limit of 12."
                                );
                                let mut formatted_style = [style::TextStyle::Bold(false); 12];
                                if style.len() == 12 {
                                    formatted_style = style.try_into().unwrap();
                                } else {
                                    formatted_style[..style.len()].copy_from_slice(style);
                                }

                                Text {
                                    text: line.to_string(),
                                    line_number,
                                    column: 0,
                                    style: formatted_style,
                                    no_of_ansi: 1,
                                }
                            }
                            .no_of_ansi(count + 1),
                        );
                    }
                    res.extend(return_val);
                }
                false => res.extend(collapse_one_deep_sub_window(sub_win)?),
            },
        }
    }

    Ok(res)
}
#[cfg(test)]
mod tests {
    use core::hash;

    use crate::errors::TextError;

    use super::*;
    use window_renderer::empty_styles;
    #[test]
    fn some() -> Result<(), TextError> {
        let bob = vec![
            window_renderer::Text::new("@", 1, 0, &empty_styles()),
            window_renderer::Text::new("@", 2, 0, &empty_styles()),
            window_renderer::Text::new("@", 2, 0, &empty_styles()),
        ];
        let children = TextType::Text({
            let style: &[style::TextStyle] = &empty_styles();
            assert!(
                style.len() <= 12,
                "The styles argument execded its limit of 12."
            );
            let mut formatted_style = [style::TextStyle::Bold(false); 12];
            if style.len() == 12 {
                formatted_style = style.try_into().unwrap();
            } else {
                formatted_style[..style.len()].copy_from_slice(style);
            }

            Text {
                text: "".to_string(),
                line_number: 1,
                column: 0,
                style: formatted_style,
                no_of_ansi: 1,
            }
        });
        let children1 = TextType::Text({
            let style: &[style::TextStyle] = &empty_styles();
            assert!(
                style.len() <= 12,
                "The styles argument execded its limit of 12."
            );
            let mut formatted_style = [style::TextStyle::Bold(false); 12];
            if style.len() == 12 {
                formatted_style = style.try_into().unwrap();
            } else {
                formatted_style[..style.len()].copy_from_slice(style);
            }

            Text {
                text: "@".to_string(),
                line_number: 2,
                column: 0,
                style: formatted_style,
                no_of_ansi: 1,
            }
        });
        let children2 = TextType::Text({
            let style: &[style::TextStyle] = &empty_styles();
            assert!(
                style.len() <= 12,
                "The styles argument execded its limit of 12."
            );
            let mut formatted_style = [style::TextStyle::Bold(false); 12];
            if style.len() == 12 {
                formatted_style = style.try_into().unwrap();
            } else {
                formatted_style[..style.len()].copy_from_slice(style);
            }

            Text {
                text: "@".to_string(),
                line_number: 2,
                column: 2,
                style: formatted_style,
                no_of_ansi: 1,
            }
        });

        let texts = vec![children.clone()];
        let child1 = SubWindow::new(
            NestedWindow::new(texts, 10, 10, window_renderer::TypeOfBorder::CurvedBorders),
            1,
            1,
        );

        let child2 = SubWindow::new(
            NestedWindow::new(
                vec![children.clone(), children.clone()],
                5,
                10,
                window_renderer::TypeOfBorder::CurvedBorders,
            ),
            4,
            1,
        );

        let root = SubWindow::new(
            NestedWindow::new(
                vec![
                    //TextType::SubWindow(child1.clone()),
                    //TextType::SubWindow(child2.clone()),
                    //TextType::SubWindow(child2.clone()),
                ],
                20,
                10,
                window_renderer::TypeOfBorder::CurvedBorders,
            ),
            1,
            1,
        );

        let t = vec![
            window_renderer::Text::new("!", 1, 0, empty_styles()),
            window_renderer::Text::new("@", 2, 0, empty_styles()),
            window_renderer::Text::new("#", 2, 1, empty_styles()),
        ];
        let mut hashmap: HashMap<u32, u32> = HashMap::new();
        hashmap.insert(2, 2);

        //let windows = window::Window::new(t, 10, 80, again::TypeOfBorder::CurvedBorders);

        let a = collapse_sub_window(root, 0)?;
        let b = window_renderer::NonNestableWindow::new(
            a.clone(),
            20,
            100,
            window_renderer::TypeOfBorder::CurvedBorders,
        );
        println!("{}", b.render(false)?);

        Ok(())
    }
}
