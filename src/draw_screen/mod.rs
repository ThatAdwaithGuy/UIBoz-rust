use super::errors::TextError;
use std::collections::HashMap;
mod opts;
use opts::*;
use std::collections::BTreeMap;
use thiserror::Error;
// <Structs & Enums>

#[derive(Debug, Clone)]
pub enum TypeOfBorder {
    NoBorders,
    CurvedBorders,
    SquareBorders,
}

enum LineType {
    Text(Text),
    EmptyLine,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub text: String,
    pub line_number: u32,
    pub column: u32,
    pub opts: Vec<TextOpts>,
    no_of_ansi_codes: u32,
}

#[derive(Debug, Clone)]
pub enum TextType {
    Text(Text),
    SubScreen(SubScreen),
}

#[derive(Debug, Clone)]
pub struct SubScreen {
    pub subscreen: Screen,
    pub start_line_number: u32,
    pub column: u32,
}

#[derive(Debug, Clone)]
pub struct Screen {
    pub texts: Vec<TextType>,
    pub width: u32,
    pub height: u32,
    pub type_of_border: TypeOfBorder,
}

// <\Structs & Enums>

//implementations

impl Text {
    pub fn new(text: &str, line_number: u32, column: u32, opts: Vec<TextOpts>) -> Text {
        Text {
            text: text.to_string(),
            line_number,
            column,
            opts,
            no_of_ansi_codes: 1,
        }
    }
    // Note, Text::new("", 0, 0, vec![]) is null
}

pub mod utils {

    use super::{TextType, *};
    use std::collections::VecDeque;

    /*
     * Used when you have a Vector of Texts which is in the same line
     * */
    fn has_nested_subscreen(subscreen: &SubScreen) -> bool {
        let mut queue = VecDeque::new();

        // Initialize the queue with the initial SubScreen
        queue.push_back(&subscreen.subscreen);

        // Iterative check for nested SubScreen
        while let Some(current_subscreen) = queue.pop_front() {
            for text_type in &current_subscreen.texts {
                match text_type {
                    TextType::Text(_) => {}
                    TextType::SubScreen(nested_subscreen) => {
                        // Check if the current TextType::SubScreen contains nested SubScreen
                        if !nested_subscreen.subscreen.texts.is_empty() {
                            return true;
                        }

                        // If not, add the nested SubScreen to the queue for further checking
                        queue.push_back(&nested_subscreen.subscreen);
                    }
                }
            }
        }

        false
    }

    pub fn combine_texts(texts: Vec<Text>) -> Result<Text, TextError> {
        let mut formatted_texts = texts;
        formatted_texts.sort_by(|a, b| a.column.cmp(&b.column));
        // Column shifting.
        let mut buffer = vec![];
        let mut prev: Text = Text::new("", 0, 0, vec![]);
        for curr in &formatted_texts {
            // The start of the loop.
            if prev == Text::new("", 0, 0, vec![]) {
                buffer.push(curr.to_owned());
            } else {
                println!("PREV: {:#?}\n", prev);
                println!("CURR: {:#?}\n", curr);
                let diff =
                    match (curr.column - prev.column).checked_sub((prev.text.len() - 78) as u32) {
                        Some(result) => result,
                        None => return Err(TextError::LeftBounds(curr.text.clone())),
                    };
                buffer.push(Text::new(
                    &curr.text,
                    curr.line_number,
                    diff,
                    curr.opts.clone(),
                ));
            }
            prev = curr.to_owned();
        }
        formatted_texts = buffer;

        // Now the Important Stuff
        formatted_texts = formatted_texts
            .iter()
            .map(|text| {
                Text::new(
                    &format_column(&text.text, text.column),
                    text.line_number,
                    text.column,
                    text.opts.clone(),
                )
            })
            .collect();

        let extracted_text: Vec<String> = formatted_texts
            .iter()
            .map(|text| text.text.clone())
            .collect();
        let output_vec = push_strings(&extracted_text);
        let mut output = Text::new(&output_vec, formatted_texts[0].line_number, 0, vec![]);
        output.no_of_ansi_codes = extracted_text.len() as u32;

        Ok(output)
    }

    fn push_strings(strings: &Vec<String>) -> String {
        strings.iter().flat_map(|s| s.chars()).collect()
    }

    pub fn handle_same_line_text_and_ansi(texts: Vec<Text>) -> Result<Vec<Text>, TextError> {
        let same_line_text = &get_duplicates(
            &texts
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let mut text = Text::new(
                        &format!("{}{}\x1b[0m", opts::parse_text_opts(v.opts.clone()), v.text),
                        v.line_number,
                        v.column,
                        vec![],
                    );
                    (i as u32, text)
                })
                .collect(),
        );
        // Really want to use a iter. but because combine_texts returns a Result Type. I can't
        // figure it out.
        let mut output = vec![];
        for i in same_line_text {
            output.push(combine_texts(i.to_vec())?);
        }
        Ok(output)
    }

    pub fn get_duplicates(input_hashmap: &HashMap<u32, Text>) -> Vec<Vec<Text>> {
        let deref_input = input_hashmap.clone();
        let input_array: BTreeMap<u32, Text> = deref_input.into_iter().collect::<BTreeMap<_, _>>();

        let mut frequency_dict = HashMap::new();
        for data in input_array.values() {
            if frequency_dict.contains_key(&data.line_number) {
                frequency_dict.insert(data.line_number, frequency_dict[&data.line_number] + 1);
            } else {
                frequency_dict.insert(data.line_number, 1);
            }
        }

        let mut seen_values = HashMap::new();
        let mut groups = Vec::new();
        let mut current_group = Vec::new();

        for (_, data) in input_array.iter() {
            let entry = seen_values.entry(&data.line_number).or_insert(0);
            *entry += 1;
            if *entry != frequency_dict[&data.line_number] {
                current_group.push(data.clone());
            } else {
                current_group.push(data.clone());
                groups.push(current_group.clone());
                current_group.clear();
            }
        }

        groups
    }

    fn pad_text(texts: Vec<Text>, height: u32) -> Vec<LineType> {
        (0..height)
            .map(|index| {
                texts
                    .iter()
                    .find(|pt| pt.line_number == (index + 1))
                    .map(|text| LineType::Text(text.clone()))
                    .unwrap_or(LineType::EmptyLine)
            })
            .collect()
    }

    pub fn unpacked_render_string(
        texts: Vec<Text>,
        width: u32,
        height: u32,
        type_of_border: TypeOfBorder,
    ) -> Result<Vec<String>, TextError> {
        let mut output_vec: Vec<String> = vec![];
        match type_of_border {
            TypeOfBorder::NoBorders => output_vec.push("".to_string()),
            TypeOfBorder::CurvedBorders => {
                output_vec.push(format!("╭{}╮", "─".repeat(width as usize)))
            }
            TypeOfBorder::SquareBorders => {
                output_vec.push(format!("┌{}┐", "─".repeat(width as usize)))
            }
        }
        let all_values = handle_same_line_text_and_ansi(texts)?;
        pad_text(all_values, height)
            .iter()
            .for_each(|text| match text {
                LineType::EmptyLine => match type_of_border {
                    TypeOfBorder::NoBorders => output_vec.push("\n".to_string()),
                    TypeOfBorder::CurvedBorders | TypeOfBorder::SquareBorders => {
                        output_vec.push(format!("│{}│", " ".repeat(width as usize)))
                    }
                },
                LineType::Text(text) => match type_of_border {
                    TypeOfBorder::SquareBorders | TypeOfBorder::CurvedBorders => {
                        output_vec.push(format!(
                            "│{}{}│",
                            text.text,
                            " ".repeat(
                                (width - ((text.text.len() as u32) - (text.no_of_ansi_codes * 78)))
                                    as usize
                            )
                        ))
                    }
                    TypeOfBorder::NoBorders => output_vec.push(format!("{}", text.text)),
                },
            });
        match type_of_border {
            TypeOfBorder::NoBorders => output_vec.push("".to_string()),
            TypeOfBorder::CurvedBorders => {
                output_vec.push(format!("╰{}╯", "─".repeat(width as usize)))
            }
            TypeOfBorder::SquareBorders => {
                output_vec.push(format!("└{}┘", "─".repeat(width as usize)))
            }
        }
        Ok(output_vec)
    }

    pub fn convert_one_subscreen_to_texts(subscreen: &SubScreen) -> Result<Vec<Text>, TextError> {
        let mut text_data = vec![];
        for text in &subscreen.subscreen.texts {
            match text {
                TextType::Text(text) => text_data.push(text.to_owned()),
                TextType::SubScreen(_) => {
                    return Err(TextError::InternalConvertOneBozToTextError());
                }
            }
        }

        let unpacked_render_string = &unpacked_render_string(
            text_data,
            subscreen.subscreen.width,
            subscreen.subscreen.height,
            subscreen.subscreen.type_of_border.clone(),
        )?;

        let mut result: Vec<Text> = vec![];
        for (index, text) in unpacked_render_string.iter().enumerate() {
            result.push(Text::new(
                text.as_str(),
                subscreen.start_line_number + index as u32,
                subscreen.column,
                vec![],
            ));
        }

        Ok(result)
    }

    pub fn format_column(string: &str, column: u32) -> String {
        format!("{}{}", " ".repeat(column as usize).as_str(), string)
    }

    pub fn contains_boz(subscreen: SubScreen) -> bool {
        for text in subscreen.subscreen.texts {
            match text {
                TextType::Text(_) => continue,
                TextType::SubScreen(_) => return true,
            }
        }
        false
    }
    //////////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////// PRIVATE FUNCTION ///////////////////////////////////
    // * Input: It takes a nested subscreen in the main Screen.
    // * Return: It will return a Vector of texts which should be extended into the
    // * text_data
    //////////////////////////////////////////////////////////////////////////////////////
    pub fn convert_subscreen_to_texts(subscreen: &SubScreen) -> Result<Vec<Text>, TextError> {
        let mut mutable_subscreen = subscreen;

        for (index, child) in mutable_subscreen.subscreen.texts.iter().enumerate() {
            match child {
                TextType::Text(text) => {
                    continue;
                }
                TextType::SubScreen(sub_screen) => if !has_nested_subscreen(sub_screen) {},
            }
        }

        Ok(convert_one_subscreen_to_texts(mutable_subscreen)?)
    }
}
#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn eq_test() -> Result<(), TextError> {
        let text_data = SubScreen {
            subscreen: Screen {
                texts: vec![TextType::SubScreen(SubScreen {
                    subscreen: Screen {
                        texts: vec![TextType::SubScreen(SubScreen {
                            subscreen: Screen {
                                texts: vec![TextType::Text(Text::new("@", 1, 1, vec![]))],
                                width: 10,
                                height: 1,
                                type_of_border: TypeOfBorder::CurvedBorders,
                            },
                            column: 1,
                            start_line_number: 2,
                        })],
                        width: 10,
                        height: 1,
                        type_of_border: TypeOfBorder::CurvedBorders,
                    },
                    column: 1,
                    start_line_number: 2,
                })],
                width: 52,
                height: 12,
                type_of_border: TypeOfBorder::CurvedBorders,
            },
            start_line_number: 5,
            column: 1,
        };
        let rizz = utils::convert_subscreen_to_texts(&text_data)?
            .iter()
            .map(|text| TextType::Text(text.clone()))
            .collect();
        let subbby = SubScreen {
            subscreen: Screen {
                texts: rizz,
                width: 20,
                height: 10,
                type_of_border: TypeOfBorder::CurvedBorders,
            },
            column: 10,
            start_line_number: 20,
        };
        println!("input:\n{:#?}", &subbby);
        println!("{:#?}", utils::convert_subscreen_to_texts(&subbby));
        Ok(())
    }
}
