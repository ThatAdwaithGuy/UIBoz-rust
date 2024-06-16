use crate::{again, draw_boz::boz::overlay_2_str, errors::TextError, style::parse_text_style};
use itertools::{self, Itertools};

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

pub fn overlay(lst: &[&'static str]) -> Option<String> {
    lst.iter()
        .try_fold("".to_string(), |acc, ele| -> Option<String> {
            let str2 = ele.to_string();
            let (bigger_str, smaler_str) = if acc.len() > str2.len() {
                (acc, str2)
            } else if acc.len() < str2.len() {
                (str2, acc)
            } else {
                (acc, str2)
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
trait ApplyVec<T, R, A> {
    fn apply(self, f: fn(A) -> R) -> R;
}

impl<I, T, R, A> ApplyVec<T, R, A> for I
where
    I: Iterator<Item = T>,
    A: std::iter::FromIterator<T>,
{
    fn apply(self, f: fn(A) -> R) -> R {
        f(self.collect())
    }
}

pub fn handle(texts: Vec<again::Text>) -> Result<Vec<Vec<again::Text>>, TextError> {
    Ok(texts
        .iter()
        .apply(group_lines)
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| {
                    again::Text::new(
                        &format!(
                            "{}{}{}\x1b[0m",
                            " ".repeat(y.column as usize),
                            parse_text_style(y.style.into()),
                            y.text
                        ),
                        y.line_number,
                        y.column,
                        y.style.into(),
                    )
                })
                .collect::<Vec<again::Text>>()
        })
        .collect())
}
