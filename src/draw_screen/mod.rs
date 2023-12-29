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

// Just for user input, not be used in the processing
#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub text: String,
    pub line_number: u32,
    pub column: u32,
    pub opts: Vec<TextOpts>,
    no_of_ansi_codes: u32,
    text_type: TextType,
}

#[derive(Debug, Clone, PartialEq)]
enum TextType {
    // Start
    Input,
    // Middle
    Formatted,
    // End
    UseInRenderString,
}

#[derive(Debug)]
struct SubScreen {
    subscreen: Screen,
    start_line_number: u32,
    column: u32,
}

#[derive(Debug)]
enum LineType {
    Text(Text),
    SubScreen(SubScreen),
}

#[derive(Debug)]
struct Screen {
    pub texts: Vec<LineType>,
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
            text_type: TextType::Input,
        }
    }
}

mod utils {

    use std::{fmt::Result, process::Output, usize};

    use super::{TextType, *};

    /*
     * Used when you have a Vector of Texts which is in the same line
     * */
    pub fn combine_texts(texts: Vec<Text>) -> std::result::Result<Text, TextError> {
        // Some column shifting. Don't worry about this.
        let mut formatted_texts = texts;
        formatted_texts.sort_by(|a, b| a.column.cmp(&b.column));
        let mut prev: Text = Text::new("", 111, 111, vec![]);
        let mut buffer = vec![];
        // Column shifting.
        for curr in &formatted_texts {
            // The start of the loop.
            if prev == Text::new("", 111, 111, vec![]) {
                buffer.push(curr.to_owned());
            } else {
                let diff = (curr.column - prev.column) - ((prev.text.len() - 78) as u32);
                let mut text = Text::new(&curr.text, curr.line_number, diff, curr.opts.clone());
                text.text_type = TextType::Formatted;
                buffer.push(text);
            }
            prev = curr.to_owned();
        }
        formatted_texts = buffer;

        // Now the Important Stuff
        formatted_texts = formatted_texts
            .iter()
            .map(|text| {
                let output_text = &format_column(&text.text, text.column);
                let mut output = Text::new(
                    output_text,
                    text.line_number,
                    text.column,
                    text.opts.clone(),
                );
                output.text_type = TextType::Formatted;
                output
            })
            .collect();

        let extracted_text: Vec<String> = formatted_texts
            .iter()
            .map(|text| text.text.clone())
            .collect();
        let output_string = push_strings(&extracted_text);
        let mut output = Text::new(&output_string, formatted_texts[0].line_number, 0, vec![]);
        output.no_of_ansi_codes = extracted_text.len() as u32;
        output.text_type = TextType::UseInRenderString;

        Ok(output)
    }

    fn push_strings(strings: &Vec<String>) -> String {
        strings.iter().flat_map(|s| s.chars()).collect()
    }

    pub fn handle_same_line_text_and_ansi(
        texts: Vec<Text>,
    ) -> std::result::Result<Vec<Text>, TextError> {
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
                    text.text_type = TextType::Formatted;
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

    pub fn format_column(string: &str, column: u32) -> String {
        format!("{}{}", " ".repeat(column as usize).as_str(), string)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_test() {
        let text_data = vec![
            Text::new("stupid", 10, 30, vec![]),
            Text::new("world", 10, 10, vec![]),
            Text::new("hello", 10, 20, vec![]),
            //Text::new("stupid", 11, 30, vec![]),
            //Text::new("world", 11, 12, vec![]),
            //Text::new("hello", 11, 15, vec![]),
        ];
        println!(
            "{:#?}",
            utils::handle_same_line_text_and_ansi(text_data).unwrap()
        );
    }
}
