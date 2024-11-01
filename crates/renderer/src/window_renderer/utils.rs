use std::cmp::Ordering;

use super::*;
use errors::TextError;
use style::parse_text_style;

use super::Text;
/*
fn group_lines(texts: Vec<Text>) -> Vec<Vec<Text>> {
    texts
        .iter()
        .chunk_by(|x| x.line_number)
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
}*/

pub fn replace_none_with_line_numbers(
    width_of_line: u32,
    vec_with_struct: &Vec<Text>,
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
/*
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
}*/

fn check_errors(texts: &Vec<Text>) -> Result<(), TextError> {
    //dbg!(texts);
    let mut sorted = texts.clone();
    sorted.sort_by_key(|k| k.column);
    let chunked = sorted
        .clone()
        .into_iter()
        .chunk_by(|x| x.line_number)
        .into_iter()
        .map(|(_, x)| x.collect_vec())
        .collect_vec();

    for lst in chunked {
        for i in 0..lst.len() - 1 {
            let sorted1 = &lst[i];
            let sorted2 = &lst[i + 1];

            // Calculate the end column of the first string
            let end1 = sorted1.column + sorted2.text.len() as u32;

            // Check if the first string overlaps with the next one
            if end1 > sorted2.column {
                return Err(TextError::TextOverlaid(
                    sorted1.text.clone(),
                    sorted2.text.clone(),
                ));
            }
        }
    }

    Ok(())
}

pub fn handle(unsorted_texts: Vec<Text>) -> Result<Vec<Text>, TextError> {
    let _ = check_errors(&unsorted_texts)?;
    let mut texts = unsorted_texts;
    texts.sort_by_key(|x| x.line_number);
    texts.sort_by(|a, b| {
        // First, compare by the 'first' field
        let first_cmp = a.line_number.cmp(&b.line_number);

        if first_cmp == Ordering::Equal {
            // If 'first' fields are equal, compare by 'second' field
            a.column.cmp(&b.column)
        } else {
            // Otherwise, use the result of comparing 'first' fields
            first_cmp
        }
    });

    //dbg!(&texts);
    Ok(texts
        .iter()
        .chunk_by(|x| x.line_number)
        .into_iter()
        .map(|(_, x)| x.into_iter().map(|y| y.clone()).collect())
        .collect::<Vec<Vec<Text>>>()
        .into_iter()
        .map(|x: Vec<Text>| (x.clone(), x.len()))
        .map(|x: (Vec<Text>, usize)| {
            let b = x
                .0
                .iter()
                .scan(None, |state: &mut Option<Text>, current: &Text| {
                    let result = match state {
                        None => current.column,
                        Some(prev) => {
                            //dbg!(
                            //    &prev,
                            //    current.column,
                            //    prev.column
                            //        + (prev.text.chars().collect::<Vec<char>>().len()
                            //            - ((prev.text.matches("\x1b").count() / 8) as u32 * 78)
                            //                as usize) as u32
                            //);

                            current.column
                                - (prev.column
                                    + (prev.text.chars().collect::<Vec<char>>().len()
                                        - ((prev.text.matches("\x1b").count() / 8) as u32 * 78)
                                            as usize) as u32)
                        }
                    };
                    *state = Some(current.clone());
                    Some((
                        Text::new(&current.text, current.line_number, result, current.style)
                            .no_of_ansi(current.no_of_ansi),
                        x.1,
                    ))
                })
                .collect_vec();
            b
        })
        .map(|x: Vec<(Text, usize)>| {
            let a = x
                .iter()
                .map(|y| {
                    (
                        Text::new(
                            &format!(
                                "{}{}{}\x1b[0m",
                                " ".repeat(y.0.column as usize),
                                parse_text_style(y.0.style.into()),
                                y.0.text
                            ),
                            y.0.line_number,
                            y.0.column,
                            y.0.style.into(),
                        )
                        .no_of_ansi(y.0.no_of_ansi),
                        y.1,
                    )
                })
                .collect::<Vec<(Text, usize)>>();
            a
        })
        .map(|x: Vec<(Text, usize)>| {
            let no_of_ansi = if x.len() == 1 {
                x[0].0.no_of_ansi
            } else {
                x.len() as u32
            };
            let mut styles: [style::TextStyle; 4] = [
                style::TextStyle::RightSideConnect(false),
                style::TextStyle::LeftSideConnect(false),
                style::TextStyle::UpSideConnect(-1),
                style::TextStyle::DownSideConnect(-1),
            ];

            for text in &x {
                let column = text.0.column as i32;
                for s in text.0.style {
                    match s {
                        style::TextStyle::RightSideConnect(_) => {
                            styles[0] = style::TextStyle::RightSideConnect(true);
                        }
                        style::TextStyle::LeftSideConnect(_) => {
                            styles[1] = style::TextStyle::LeftSideConnect(false);
                        }
                        style::TextStyle::UpSideConnect(_) => {
                            styles[2] = style::TextStyle::UpSideConnect(column);
                        }
                        style::TextStyle::DownSideConnect(_) => {
                            styles[3] = style::TextStyle::DownSideConnect(column);
                        }
                        _ => {}
                    }
                    if styles
                        == [
                            style::TextStyle::RightSideConnect(false),
                            style::TextStyle::LeftSideConnect(false),
                            style::TextStyle::UpSideConnect(-1),
                            style::TextStyle::DownSideConnect(-1),
                        ]
                    {
                        break;
                    }
                }
            }
            let b = Text::new(
                &x.iter().map(|y| y.0.text.clone()).join(""),
                x[0].0.line_number,
                0,
                {
                    let mut arr = [style::TextStyle::Bold(false); 12];
                    arr[..4].copy_from_slice(&styles);
                    arr
                },
            )
            .no_of_ansi(no_of_ansi as u32);

            b
        })
        .collect())
}
