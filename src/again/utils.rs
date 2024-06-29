use std::collections::HashSet;

use crate::{again, errors::TextError, style::parse_text_style};
use itertools::{self, Itertools};

use super::Text;

fn group_lines(texts: Vec<again::Text>) -> Vec<Vec<again::Text>> {
    texts
        .iter()
        .group_by(|x| x.line_number)
        .into_iter()
        .map(|(_, x)| x.into_iter().map(|y| y.clone()).collect())
        .collect()
}

fn format_column(string: &str, column: i32) -> String {
    let n = column as usize;
    let binding = " ".repeat(n);
    let fstring = binding.as_str();
    format!("{}{}", fstring, string)
}

fn make_lists_equal_length(list1: Vec<char>, list2: Vec<char>) -> (Vec<char>, Vec<char>) {
    let (bigger_list, mut smaler_list) = if list1.len() > list2.len() {
        (list1, list2)
    } else if list1.len() < list2.len() {
        (list2, list1)
    } else {
        (list1, list2)
    };

    let padding = bigger_list.len() - smaler_list.len();
    let add_pad = vec![' '; padding];
    smaler_list.extend(add_pad.iter());

    (bigger_list, smaler_list)
}

pub fn replace_none_with_line_numbers(
    width_of_line: u32,
    vec_with_struct: &Vec<again::Text>,
) -> Vec<Option<Text>> {
    (0..width_of_line)
        .map(|index| {
            vec_with_struct
                .iter()
                .find(|pt| pt.line_number == (index + 1) as u32)
                .map(|private_text| Some(private_text.clone()))
                .unwrap_or(None)
        })
        .collect()
}
fn overlay(lst: &[&'static str]) -> Option<String> {
    lst.iter()
        .try_fold("".to_string(), |acc, ele| -> Option<String> {
            let (bigger_str, smaler_str) = if acc.len() > ele.len() {
                (acc, ele.to_string())
            } else if acc.len() < ele.len() {
                let var_name = (ele.to_string(), acc);
                var_name
            } else {
                (acc, ele.to_string())
            };

            let (bigger_list, smaller_list): (Vec<char>, Vec<char>) = make_lists_equal_length(
                bigger_str.chars().collect::<Vec<char>>(),
                smaler_str.chars().collect::<Vec<char>>(),
            );

            bigger_list
                .iter()
                .zip(smaller_list)
                .map(|(x, y)| {
                    if *x == ' ' && y == ' ' {
                        Some(' ')
                    } else if *x != ' ' && y == ' ' {
                        Some(*x)
                    } else if *x == ' ' && y != ' ' {
                        Some(y)
                    } else if *x != ' ' && y != ' ' {
                        Some(*x)
                    } else {
                        None
                    }
                })
                .collect::<Option<String>>()
        })
}

fn check_errors(texts: &Vec<again::Text>) -> Result<(), TextError> {
    let mut sorted = texts.clone();
    sorted.sort_by_key(|k| k.column);

    for i in 0..sorted.len() - 1 {
        let sorted1 = &sorted[i];
        let sorted2 = &sorted[i + 1];

        // Calculate the end column of the first string
        let end1 = sorted1.column + sorted2.text.len() as u32;

        // Check if the first string overlaps with the next one
        if end1 > sorted2.column {
            return Err(TextError::TextOverlayed(
                "IDK".to_string(),
                "IDK".to_string(),
            ));
        }
    }

    Ok(())
}

pub fn handle(texts: Vec<again::Text>) -> Result<Vec<again::Text>, TextError> {
    let _ = check_errors(&texts)?;
    Ok(texts
        .iter()
        .group_by(|x| x.line_number)
        .into_iter()
        .map(|(_, x)| x.into_iter().map(|y| y.clone()).collect())
        .map(|x: Vec<Text>| (x.clone(), x.len()))
        .map(|x: (Vec<Text>, usize)| {
            let b =
                x.0.iter()
                    .scan(None, |state: &mut Option<Text>, current: &Text| {
                        let result = match state {
                            None => current.column,
                            Some(prev) => {
                                current.column
                                    - (prev.column
                                        + prev.text.chars().collect::<Vec<char>>().len() as u32)
                            }
                        };
                        *state = Some(current.clone());
                        Some((
                            Text::new(&current.text, current.line_number, result, current.style),
                            x.1,
                        ))
                    })
                    .collect_vec();
            dbg!(x, b.clone());
            b
        })
        .map(|x: Vec<(again::Text, usize)>| {
            x.iter()
                .map(|y| {
                    (
                        again::Text::new(
                            &format!(
                                "{}{}{}\x1b[0m",
                                " ".repeat(y.0.column as usize),
                                parse_text_style(y.0.style.into()),
                                y.0.text
                            ),
                            y.0.line_number,
                            y.0.column,
                            y.0.style.into(),
                        ),
                        y.1,
                    )
                })
                .collect::<Vec<(again::Text, usize)>>()
        })
        .map(|x: Vec<(again::Text, usize)>| {
            Text::new(
                &x.iter()
                    .map(|y| y.0.clone())
                    .map(|y| y.text.clone())
                    .join(""),
                x[0].0.line_number,
                0,
                &[],
            )
            .no_of_ansi(x[0].1 as u32)
        })
        .collect())
}
