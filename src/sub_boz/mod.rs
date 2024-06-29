use super::again;
use crate::errors;
use core::panic;
use std::{collections::HashMap, hash::Hash, rc::Rc, usize};
// use crate::draw_again::opts;

type Texts = Vec<TextType>;

#[derive(Clone)]
struct NestedWindow {
    texts: Texts,
    height: u32,
    width: u32,
    type_of_border: again::TypeOfBorder,
}
impl NestedWindow {
    fn new(texts: Texts, height: u32, width: u32, type_of_border: again::TypeOfBorder) -> Self {
        Self {
            texts,
            height,
            width,
            type_of_border,
        }
    }
}

#[derive(Clone)]
struct SubWindow {
    window: NestedWindow,
    start_line_number: u32,
    column: u32,
}
impl SubWindow {
    fn new(window: NestedWindow, start_line_number: u32, column: u32) -> Self {
        Self {
            window,
            start_line_number,
            column,
        }
    }
}

#[derive(Clone)]
enum TextType {
    SubWindow(SubWindow),
    Text(again::Text),
}

fn has_nested_again(window: NestedWindow) -> bool {
    window.texts.iter().any(|tt| match tt {
        TextType::SubWindow(_) => true,
        TextType::Text(_) => false,
    })
}

fn render_lines(
    window: again::Window,
    start_line_number: u32,
    column: u32,
) -> Result<Vec<again::Text>, errors::TextError> {
    let rendered_again = window.render()?;
    let splited: Vec<&str> = rendered_again.split("\n").into_iter().collect();
    let mut res: Vec<again::Text> = splited
        .into_iter()
        .enumerate()
        .map(|(i, v)| again::Text::new(v, i as u32 + start_line_number, column, &[]))
        .collect();
    Ok(res)
}

// Nestedagain to Boz
fn nb_to_b(window: NestedWindow) -> Option<again::Window> {
    let mut texts = vec![];
    for tt in window.texts {
        match tt {
            TextType::Text(text) => texts.push(text),
            TextType::SubWindow(_) => {
                return None;
            }
        }
    }
    Some(again::Window::new(
        texts,
        window.height,
        window.width,
        window.type_of_border,
    ))
}

fn sort_hashmap_by_key<K, V>(map: &HashMap<K, V>) -> Vec<(K, V)>
where
    K: Ord + Clone + Hash,
    V: Clone,
{
    let mut vec: Vec<(K, V)> = map.iter().map(|(i, v)| (i.clone(), v.clone())).collect();
    vec.sort_by(|a, b| a.0.cmp(&b.0));
    vec
}

// Absolute Hell
fn word_indices(input: &str) -> Vec<(usize, String)> {
    let mut result: Vec<(usize, String)> = Vec::new();
    let string = String::from(input)
        .chars()
        .enumerate()
        .collect::<HashMap<usize, char>>();

    let splited: HashMap<usize, char> = sort_hashmap_by_key(&string)
        .iter()
        .filter(|(_, x)| *x != ' ')
        .map(|(i, v)| (*i, *v))
        .collect();
    dbg!(input, sort_hashmap_by_key(&splited));
    let mut word = String::new();
    let mut word_start_index = 0usize;
    let mut in_word = false;
    for (i, c) in sort_hashmap_by_key(&splited) {
        match c {
            '│' => {
                if in_word {
                    result.push((word_start_index, word.clone()));
                    word.clear();
                    in_word = false;
                    word_start_index = 0;
                }
                result.push((i, c.to_string()));
            }
            _ => {
                in_word = true;
                if word_start_index == 0 {
                    word_start_index = i;
                }
                word.push(c);
            }
        }
    }
    dbg!(result.clone());
    if !word.is_empty() && result.is_empty() {
        result.push((word_start_index, word.clone()))
    }

    result
}

fn partition_line(text: again::Text) -> Vec<again::Text> {
    word_indices(&text.text)
        .iter()
        .map(|(i, v)| again::Text::new(v, text.line_number, *i as u32 + text.column, &[]))
        .collect()
}

fn collapse_nested_again(nested_windows: SubWindow) -> Result<Vec<again::Text>, errors::TextError> {
    let mut res: Vec<again::Text> = vec![];
    for text_type in nested_windows.window.texts {
        match text_type {
            TextType::Text(text) => res.push(text),
            TextType::SubWindow(sub_window) => {
                let mut res1: Vec<again::Text> = vec![];
                for text_type1 in sub_window.window.texts {
                    match text_type1 {
                        TextType::Text(text1) => res1.push(text1.clone()),
                        TextType::SubWindow(_sub_boz) => {
                            panic!("SHIT")
                        }
                    }
                }
                res.extend(render_lines(
                    again::Window::new(
                        res1,
                        sub_window.window.height,
                        sub_window.window.width,
                        sub_window.window.type_of_border,
                    ),
                    sub_window.start_line_number,
                    sub_window.column,
                )?)
            }
        }
    }
    //Ok(render_lines(
    //    again::Boz::new(
    //        res,
    //        nested_again.boz.height,
    //        nested_again.boz.width,
    //        nested_again.boz.type_of_border,
    //    ),
    //    nested_again.start_line_number,
    //    nested_again.column,
    //)?)
    Ok(res)
}

#[cfg(test)]
mod tests {
    use self::again::{Text, TypeOfBorder};

    use super::*;
    #[test]
    fn test_name() -> Result<(), errors::TextError> {
        ////////////////////////////////////////
        // Adwaith, no_of_ansi_codes. Got it? //
        ////////////////////////////////////////

        ////////////////////////////////////////
        // Past Adwaith, You're wrong, Got it //
        ////////////////////////////////////////

        //let mut texts: Vec<TextType> = vec![];
        //texts.push(TextType::Text(again::Text::new("Hello", 1, 4, Rc::new([]))));
        //texts.push(TextType::Text(again::Text::new("world", 1, 11, Rc::new([]))));
        //texts.push(TextType::Text(again::Text::new("Love to", 2, 4, Rc::new([]))));
        //texts.push(TextType::Text(again::Text::new(
        //    "Catpuccin",
        //    2,
        //    11,
        //    Rc::new([]),
        //)));
        //let again = NestedBoz::new(
        //    vec![TextType::Text(again::Text::new("hello", 5, 5, Rc::new([])))],
        //    5,
        //    10,
        //    TypeOfBorder::CurvedBorders,
        //);
        //texts.push(TextType::Subagain(SubBoz::new(boz, 3, 1)));
        //let nested_again = SubBoz::new(
        //    Nestedagain::new(texts, 5, 40, TypeOfBorder::CurvedBorders),
        //    1,
        //    1,
        //);
        //let collapsed = collapse_nested_again(nested_boz.clone())?;
        //let text = &collapsed[2];
        //dbg!(word_indices(&text.text));
        //dbg!(partition_line(text.clone()));
        //let av = collapsed
        //    .clone()
        //    .iter()
        //    .map(|x| partition_line(x.clone()))
        //    .flatten()
        //    .collect::<Vec<Text>>();
        //let a = render_lines(
        //    again::Boz::new(
        //        collapsed.clone(),
        //        nested_again.clone().boz.height,
        //        nested_again.clone().boz.width,
        //        nested_again.clone().boz.type_of_border,
        //    ),
        //    nested_again.clone().start_line_number,
        //    nested_again.clone().column,
        //)?;
        //dbg!(av.clone());
        //let ragain = boz::Boz::new(collapsed, 20, 100, TypeOfBorder::CurvedBorders);
        //let b = ragain.render_string()?;
        //println!("{}", b);
        //dbg!(a);
        //let te = Text::new("╭──────────╮", 1, 0, Rc::new([]));
        //let tagain = boz::Boz::new(vec![te], 12, 52, TypeOfBorder::CurvedBorders);
        //println!("{}", tagain.render_string()?);

        //let texts = vec![
        //    again::Text::new("Hello", 1, 0, &[]),
        //    again::Text::new("To", 1, 7, &[]),
        //];

        //let b = again::Window::new(texts, 12, 56, again::TypeOfBorder::CurvedBorders);
        //println!("{}", b.render()?);
        let mut texts: Vec<TextType> = vec![TextType::Text(Text::new("Hello", 1, 0, &[]))];
        let sub_window = SubWindow::new(
            NestedWindow::new(
                vec![TextType::Text(Text::new("@", 1, 1, &[]))],
                3,
                5,
                TypeOfBorder::CurvedBorders,
            ),
            1,
            1,
        );

        texts.push(TextType::SubWindow(sub_window));

        let sub_win = SubWindow::new(
            NestedWindow::new(texts, 3, 5, TypeOfBorder::CurvedBorders),
            1,
            1,
        );
        let text = collapse_nested_again(sub_win)?;

        let win = again::Window::new(text, 10, 100, TypeOfBorder::CurvedBorders);

        println!("{}", win.render()?);

        Ok(())
    }
}
