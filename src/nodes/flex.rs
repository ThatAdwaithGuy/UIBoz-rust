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
        TextType::SubWindow(sub_window) => sub_window.column + sub_window.window.width,
        TextType::Text(text) => text.column + text.text.chars().collect::<Vec<char>>().len() as u32,
    }
}

fn get_vector<H: PartialEq, T: Clone>(vector: &Vec<(H, T)>, index: H) -> T {
    vector.iter().find(|x| x.0 == index).unwrap().1.clone()
}

fn shorten_height(texts: Vec<TextType>) -> Option<Vec<TextType>> {
    let height_min: u32 = texts.iter().map(get_line_number).min()?;
    let height_max: u32 = texts.iter().map(get_line_number).max()?;
    let mut hashmap_height: Vec<(u32, Vec<TextType>)> = (height_min..height_max + 1)
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
    let mut is_empty = usize::MAX;
    for (line_number, vector) in &hashmap_height[1..] {
        if vector.is_empty() {
            is_empty = *line_number as usize;
        }
    }

    let mut res: Vec<TextType> = vec![];
    for idx in &hashmap_height {
        if (is_empty as u32) < idx.0 {
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
                    no_of_ansi: 0,
                }),
            }));
        } else {
            res.extend(idx.1.clone());
        }
    }

    Some(res) 
}

pub trait Flex {
    fn flex(&self, width: u32, height: u32) -> Option<Window>;
}

impl Flex for Window {
    fn flex(&self, width: u32, height: u32) -> Option<Window> {
        let length = self.texts.len() - 1;

        let mut sorted_texts_by_width = self.texts.clone();
        sorted_texts_by_width.sort_by_key(get_width);

        let mut sorted_texts_by_height = self.texts.clone();
        sorted_texts_by_height.sort_by_key(get_line_number);
        let max_width_element = &sorted_texts_by_width[length];
        let max_width = get_width(max_width_element);

        let max_height_element = &sorted_texts_by_height[length];

        let max_height = get_line_number(max_width_element);

        if width >= max_width && height >= max_height {
            return Some(Window {
                texts: self.texts.clone(),
                width,
                height,
                type_of_border: self.type_of_border,
            });
        }

        let mut desired_height = height;
        let mut desired_width = width;

        // Height check
        if width < max_width {
            if self.height <= height {
                desired_height = height;
            }

            for idx in self.texts.len() - 1..0 {
                for line in self
                    .texts
                    .iter()
                    .filter(|x| get_line_number(x) == idx as u32)
                {}
            }
        }

        None
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
    #[ignore = "..."]
    fn flex_test_compression() {
        let texts = vec![TextType::Text(Text::new(
            "@, hello world hehe, lololololololololololololololololol",
            1,
            1,
            &[],
        ))];
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
    fn shorten_height_test() {
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
}
