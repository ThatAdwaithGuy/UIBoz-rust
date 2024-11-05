use std::collections::VecDeque;
use std::vec;

use itertools::Itertools;

use super::{is_nested, SubWindow, TextType};
use crate::errors::TextError;
use crate::raw_window::{self, NonNestableWindow};
use crate::sub_win::NestedWindow;

pub fn collapse_one_deep_sub_window_new(
    win: SubWindow,
) -> Result<Vec<raw_window::Text>, TextError> {
    let mut texts: Vec<raw_window::Text> = Vec::new();
    // Depth check
    if win.window.texts.iter().any(|x| match x {
        TextType::Text(text) => {
            texts.push(text.clone());
            false
        }
        TextType::SubWindow(_) => true,
    }) {
        return Err(TextError::DuplicateText(
            "if you see this. I made a mistake here".to_string(),
        ));
    }
    let window = NonNestableWindow::new(
        texts,
        win.window.height,
        win.window.width,
        win.window.type_of_border,
    );
    let mut return_val: Vec<raw_window::Text> = vec![];
    let rendered_string = window.render(true)?;
    let split = rendered_string.split("\n");
    for (idx, line) in split.enumerate() {
        let count = (line.matches("\x1b").count() / 8) as u32;
        return_val
            .push(raw_window::Text::new(line, (idx + 1) as u32, 0, &[]).no_of_ansi(count + 1));
    }
    Ok(return_val)
}
const MAX_DEPTH: u32 = 128;
fn collapse_sub_window_new(win: SubWindow, depth: u32) -> Result<Vec<raw_window::Text>, TextError> {
    let mut res: Vec<raw_window::Text> = Vec::new();
    if depth >= MAX_DEPTH {
        return Err(TextError::DepthLimitExceeded(win));
    }

    for text_type in win.window.texts {
        match text_type {
            TextType::Text(t) => res.push(t.to_owned()),
            TextType::SubWindow(sub_win) => match is_nested(&sub_win.window.texts) {
                true => {
                    let collapsed = collapse_sub_window_new(sub_win, depth + 1)?;

                    let window = NonNestableWindow::new(
                        collapsed,
                        win.window.height,
                        win.window.width,
                        win.window.type_of_border,
                    );
                    let mut return_val: Vec<raw_window::Text> = vec![];
                    let rendered_string = window.render(true)?;
                    let split = rendered_string.split("\n");
                    for (idx, line) in split.enumerate() {
                        let count = (line.matches("\x1b").count() / 8) as u32;
                        return_val.push(
                            raw_window::Text::new(line, (idx + 1) as u32, 0, &[])
                                .no_of_ansi(count + 1),
                        );
                    }
                    res.extend(return_val);
                }
                false => res.extend(collapse_one_deep_sub_window_new(sub_win)?),
            },
        }
    }

    Ok(res)
}

#[test]
fn feature() -> Result<(), TextError> {
    //let texts: Vec<TextType> = vec![TextType::Text(raw_window::Text::new("hel", 1, 0, &[]))];
    //let outer_window = NestedWindow::new(texts, 1, 3, raw_window::TypeOfBorder::CurvedBorders);
    //let outer_sub = SubWindow::new(outer_window, 0, 0);
    //let stuff = collapse_one_deep_sub_window_new(outer_sub)?;

    //let outer_windoww = NestedWindow::new(
    //    stuff
    //        .iter()
    //        .map(|x| TextType::Text(x.clone()))
    //        .collect::<Vec<TextType>>(),
    //    5,
    //    10,
    //    raw_window::TypeOfBorder::CurvedBorders,
    //);
    //let mut outer_subb = SubWindow::new(outer_windoww, 0, 0);
    //outer_subb
    //    .window
    //    .texts
    //    .remove(outer_subb.window.texts.len() - 1);
    //let more_stuff = collapse_sub_window_new(outer_subb, 0)?;
    //let fourth_dimension_window = NestedWindow::new(
    //    more_stuff
    //        .iter()
    //        .map(|x| TextType::Text(x.clone()))
    //        .collect_vec(),
    //    12,
    //    56,
    //    raw_window::TypeOfBorder::CurvedBorders,
    //);
    //let fourth_dimension_sub = SubWindow::new(fourth_dimension_window, 1, 1);
    //dbg!(&fourth_dimension_sub);
    //dbg!(collapse_sub_window_new(fourth_dimension_sub, 0));

    let texts: Vec<TextType> = vec![TextType::Text(raw_window::Text::new("@", 1, 0, &[]))];
    let first_window = NestedWindow::new(texts, 1, 1, raw_window::TypeOfBorder::CurvedBorders);
    let first_sub = SubWindow::new(first_window, 1, 0);
    let second_window = NestedWindow::new(
        vec![TextType::SubWindow(first_sub)],
        5,
        5,
        raw_window::TypeOfBorder::CurvedBorders,
    );
    let second_sub = SubWindow::new(second_window, 1, 0);
    let third_window = NestedWindow::new(
        vec![TextType::SubWindow(second_sub)],
        15,
        15,
        raw_window::TypeOfBorder::CurvedBorders,
    );
    let third_sub = SubWindow::new(third_window, 1, 0);

    //let fourth_window = NestedWindow::new(
    //    vec![TextType::SubWindow(third_sub)],
    //    12,
    //    56,
    //    raw_window::TypeOfBorder::CurvedBorders,
    //);
    //let fourth_sub = SubWindow::new(second_window, 1, 0);
    dbg!(&third_sub);
    dbg!(collapse_sub_window_new(third_sub, 0));
    Ok(())
}
