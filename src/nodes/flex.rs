use std::collections::HashMap;

use crate::renderer::{sub_win::TextType, window::Window};

fn get_line_number(text_type: &TextType) -> u32 {
    match text_type {
        TextType::SubWindow(sub_window) => sub_window.start_line_number,
        TextType::Text(text) => text.line_number,
    }
}

fn get_width(text_type: &TextType) -> u32 {
    match text_type {
        TextType::SubWindow(sub_window) => sub_window.column + sub_window.window.width + 2,
        TextType::Text(text) => text.column + text.text.chars().collect::<Vec<char>>().len() as u32,
    }
}

fn get_column(text_type: &TextType) -> u32 {
    match text_type {
        TextType::SubWindow(sub_window) => sub_window.column,
        TextType::Text(text) => text.column,
    }
}

fn shorten_width(texts: Vec<TextType>, width: u32) -> Option<Vec<TextType>> {
    let min: u32 = texts.iter().map(get_width).min()?;
    let max: u32 = texts.iter().map(get_width).max()?;
    dbg!(max);
    if max < width - 1 {
        return Some(texts);
    }
    let mut hashmap: Vec<(u32, Vec<TextType>)> = (min..max + 1)
        .map(|i| {
            let matching_structs: Vec<_> = texts
                .iter()
                .filter(|s| get_width(s) == i)
                .cloned()
                .collect();
            if !matching_structs.is_empty() {
                (i, matching_structs)
            } else {
                (i, vec![])
            }
        })
        .collect();
    hashmap.sort_by_key(|x| x.0);
    let mut res: Vec<TextType> = vec![];
    for (width, texts) in hashmap {
        if width != max {
            res.extend(texts);
            continue;
        }

        for text in &texts {
            if get_column(&text) != 0 {
                res.push(match text {
                    TextType::SubWindow(sub_window) => {
                        TextType::SubWindow(crate::renderer::sub_win::SubWindow::new(
                            sub_window.window.clone(),
                            sub_window.start_line_number,
                            sub_window.column - 1,
                        ))
                    }
                    TextType::Text(text) => {
                        TextType::Text(crate::renderer::window_renderer::Text {
                            text: text.text.clone(),
                            line_number: text.line_number,
                            column: text.column - 1,
                            style: text.style,
                            no_of_ansi: 1,
                        })
                    }
                });
            } else {
                return None;
            }

        }
    }
    Some(res)
}


fn shorten_height(texts: Vec<TextType>) -> Option<Vec<TextType>> {
    let min: u32 = texts.iter().map(get_line_number).min()?;
    let max: u32 = texts.iter().map(get_line_number).max()?;
    let mut hashmap: Vec<(u32, Vec<TextType>)> = (min..max + 1)
        .map(|i| {
            let matching_structs: Vec<_> = texts
                .iter()
                .filter(|s| get_line_number(s) == i)
                .cloned()
                .collect();
            if !matching_structs.is_empty() {
                (i, matching_structs)
            } else {
                (i, vec![])
            }
        })
        .collect();
    let mut is_empty: Option<usize> = None;
    for (line_number, vector) in &hashmap[1..] {
        if vector.is_empty() {
            is_empty = Some(*line_number as usize);
        }
    }

    let mut res: Vec<TextType> = vec![];
    for idx in &hashmap {
        if (is_empty? as u32) < idx.0 {
            // BUG: They can be a bug if two texts are adjecent to each other with no spaces.
            res.extend(idx.1.clone().iter().map(|x| match x {
                TextType::SubWindow(sub_window) => {
                    TextType::SubWindow(crate::renderer::sub_win::SubWindow::new(
                        sub_window.window.clone(),
                        sub_window.start_line_number - 1,
                        sub_window.column,
                    ))
                }
                TextType::Text(text) => TextType::Text(crate::renderer::window_renderer::Text {
                    text: text.text.clone(),
                    line_number: text.line_number - 1,
                    column: text.column,
                    style: text.style,
                    no_of_ansi: 1,
                }),
            }));
        } else {
            res.extend(idx.1.clone());
        }
    }

    Some(res)
}


impl Window {
    fn flex(&self, width: u32, height: u32) -> Option<Window> {
        let width_dif = std::cmp::max(0,self.width - width);
        let height_dif = std::cmp::max(0,self.height - height);
        let mut res = self.texts.clone();
        for _ in 0..width_dif {
            res = shorten_width(res, width)?;
        }
        for _ in 0..height_dif {
            res = shorten_height(res)?;
        }
        Some(Window {
            texts: res,
            width,
            height,
            type_of_border: self.type_of_border,
        })
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    use crate::renderer::sub_win::{SubWindow, TextType};
    use crate::renderer::window::Window;
    use crate::renderer::window_renderer::Text;
    #[test]
    fn flex_test_no_compression() {
        let texts = vec![TextType::Text(Text::new("@, hello world hehe", 1, 1, &[]))];
        let window = Window {
            texts,
            width: 56,
            height: 12,
            type_of_border: crate::renderer::window_renderer::TypeOfBorder::CurvedBorders,
        };

        let flexed = window.flex(24, 12);

        assert!(flexed.is_some());
    }

    #[test]
    fn flex_test_compression() {
        let texts = vec![TextType::Text(Text::new(
            "@, hello world hehe",
            1,
            5,
            &[],
        ))];
        let window = Window {
            texts,
            width: 56,
            height: 12,
            type_of_border: crate::renderer::window_renderer::TypeOfBorder::CurvedBorders,
        };

        let flexed = window.flex(24, 12);
        dbg!(&flexed);
        assert!(flexed.is_some());
    }
    #[test]
    fn shorten_height_test_1() {
        let vector: Vec<TextType> = vec![
            TextType::Text(Text::new("!@#$", 1, 0, &[])),
            TextType::SubWindow(SubWindow::new(
                crate::renderer::sub_win::NestedWindow::new(
                    vec![TextType::Text(Text::new("@", 1, 0, &[]))],
                    1,
                    1,
                    crate::renderer::window_renderer::TypeOfBorder::CurvedBorders,
                ),
                3,
                1,
            )),
        ];

        let correct_answer: Vec<TextType> = vec![
            TextType::Text(Text::new("!@#$", 1, 0, &[])),
            TextType::SubWindow(SubWindow::new(
                crate::renderer::sub_win::NestedWindow::new(
                    vec![TextType::Text(Text::new("@", 1, 0, &[]))],
                    1,
                    1,
                    crate::renderer::window_renderer::TypeOfBorder::CurvedBorders,
                ),
                2,
                1,
            )),
        ];
        let output = shorten_height(vector);
        assert_eq!(output, Some(correct_answer));
    }

    #[test]
    fn shorten_height_test_2() {
        let vector: Vec<TextType> = vec![
            TextType::Text(Text::new("!@#$", 1, 0, &[])),
            TextType::SubWindow(SubWindow::new(
                crate::renderer::sub_win::NestedWindow::new(
                    vec![TextType::Text(Text::new("@", 1, 0, &[]))],
                    1,
                    1,
                    crate::renderer::window_renderer::TypeOfBorder::CurvedBorders,
                ),
                2,
                1,
            )),
        ];

        let shorten_height = shorten_height(vector);
        assert!(shorten_height.is_none())
    }

    #[test]
    fn shorten_height_test_3() {
        let vector: Vec<TextType> = vec![
            TextType::Text(Text::new("!@#$", 1, 0, &[])),
            TextType::SubWindow(SubWindow::new(
                crate::renderer::sub_win::NestedWindow::new(
                    vec![TextType::Text(Text::new("@", 1, 0, &[]))],
                    1,
                    1,
                    crate::renderer::window_renderer::TypeOfBorder::CurvedBorders,
                ),
                3,
                1,
            )),
            TextType::Text(Text::new("!@#$", 5, 0, &[])),
        ];

        let correct_answer: Vec<TextType> = vec![
            TextType::Text(Text::new("!@#$", 1, 0, &[])),
            TextType::SubWindow(SubWindow::new(
                crate::renderer::sub_win::NestedWindow::new(
                    vec![TextType::Text(Text::new("@", 1, 0, &[]))],
                    1,
                    1,
                    crate::renderer::window_renderer::TypeOfBorder::CurvedBorders,
                ),
                3,
                1,
            )),
            TextType::Text(Text::new("!@#$", 4, 0, &[])),
        ];
        let shorten_height = shorten_height(vector);
        assert_eq!(shorten_height, Some(correct_answer))
    }

    #[test]
    fn shorten_width_test() {
        let vector: Vec<TextType> = vec![
            TextType::Text(Text::new("!@#$", 1, 0, &[])),
            TextType::SubWindow(SubWindow::new(
                crate::renderer::sub_win::NestedWindow::new(
                    vec![TextType::Text(Text::new("@", 1, 0, &[]))],
                    1,
                    1,
                    crate::renderer::window_renderer::TypeOfBorder::CurvedBorders,
                ),
                3,
                1,
            )),
            TextType::Text(Text::new("!@#$", 5, 4, &[])),
        ];
        let answer = vec![
            TextType::Text(Text::new("!@#$", 1, 0, &[])),
            TextType::SubWindow(SubWindow::new(
                crate::renderer::sub_win::NestedWindow::new(
                    vec![TextType::Text(Text::new("@", 1, 0, &[]))],
                    1,
                    1,
                    crate::renderer::window_renderer::TypeOfBorder::CurvedBorders,
                ),
                3,
                1,
            )),
            TextType::Text(Text::new("!@#$", 5, 4, &[])),
        ];

        let shorten = shorten_width(vector, 10);
        dbg!(&shorten);
        assert_eq!(shorten, Some(answer));
    }
}
