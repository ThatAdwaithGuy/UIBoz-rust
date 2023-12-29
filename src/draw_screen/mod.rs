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

#[derive(Debug, Clone)]
pub struct Text {
    pub text: String,
    pub line_number: u32,
    pub column: u32,
    pub opts: Vec<TextOpts>,
    no_of_ansi_codes: u32,
    text_type: TextType,
}

#[derive(Debug, Clone)]
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

    use std::{fmt::Result, process::Output};

    use super::{TextType, *};
    pub fn make_lists_equal_length(list1: Vec<char>, list2: Vec<char>) -> (Vec<char>, Vec<char>) {
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

        let output = (bigger_list, smaler_list);
        output
    }
    // I Thought I was making this overly complicated
    // And it turns out, Its TRUE.
    // I have VERY good replacement and im deprecating it.
    /*
    pub fn overlay_2_str<'a>(
        str1: &'a str,
        str2: &'a str,
    ) -> std::result::Result<String, TextError> {
        let (bigger_str, smaler_str) = if str1.len() > str2.len() {
            (str1, str2)
        } else if str1.len() < str2.len() {
            (str2, str1)
        } else {
            (str1, str2)
        };

        let mut bigger_list: Vec<char> = bigger_str.chars().collect::<Vec<char>>();
        let mut smaler_list: Vec<char> = smaler_str.chars().collect::<Vec<char>>();

        (bigger_list, smaler_list) =
            make_lists_equal_length(bigger_list.clone(), smaler_list.clone());

        let mut output_list: Vec<char> = vec![];

        for i in 0..bigger_list.len() {
            if bigger_list[i] != ' ' && smaler_list[i] != ' ' {
                return Err(TextError::TextOverlayed(
                    bigger_str.to_owned(),
                    smaler_str.to_owned(),
                ));
            }
        }

        for i in 0..bigger_list.len() {
            if bigger_list[i] == ' ' && smaler_list[i] == ' ' {
                output_list.push(' ');
            } else if bigger_list[i] != ' ' && smaler_list[i] == ' ' {
                output_list.push(bigger_list[i]);
            } else if bigger_list[i] == ' ' && smaler_list[i] != ' ' {
                output_list.push(smaler_list[i]);
            } else if bigger_list[i] != ' ' && smaler_list[i] != ' ' {
                output_list.push(bigger_list[i]);
            }
        }

        let from_iter = String::from_iter(output_list);
        Ok(from_iter)
    }
    fn overlay<'a>(lst: Vec<String>) -> std::result::Result<String, TextError> {
        let mut last_element = lst[0].to_string();
        for i in 1..lst.len() {
            last_element = overlay_2_str(&last_element, &lst[i])?;
        }
        Ok(last_element.to_owned())
    }
    */
    /*
     * Used when you have a Vector of Texts which is in the same line
     * */
    pub fn combine_texts(texts: Vec<Text>) -> std::result::Result<Text, TextError> {
        // Some column shifting. Don't worry about this.
        let mut formatted_texts = texts;
        formatted_texts.sort_by(|a, b| a.column.cmp(&b.column));

        formatted_texts = formatted_texts
            .iter()
            .zip(formatted_texts.iter().skip(1))
            .scan(formatted_texts[0].clone(), |prev, (_, curr)| {
                let diff_column = curr.column - prev.column;
                let formatted_text =
                    Text::new(&curr.text, curr.line_number, diff_column, curr.opts.clone());
                *prev = curr.clone();
                Some(formatted_text)
            })
            .collect();
        formatted_texts.insert(0, formatted_texts.get(0).unwrap().to_owned());
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
        println!("{:#?}", formatted_texts);

        let extracted_text: Vec<String> = formatted_texts
            .iter()
            .map(|text| text.text.clone())
            .collect();
        let output_string = push_strings(extracted_text);
        let mut output = Text::new(&output_string, formatted_texts[0].line_number, 0, vec![]);
        output.text_type = TextType::Formatted;

        Ok(output)
    }

    pub fn push_strings(strings: Vec<String>) -> String {
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
        let mut output = vec![];
        for i in same_line_text {
            output.push(combine_texts(i.to_vec())?);
        }
        Ok(output)
    }

    /*
    pub fn convert_one_subscreen_to_texts(sub_screen: SubScreen) -> Result<Vec<Text>, TextError> {
        let mut text_data = vec![];
        for i in &sub_screen.subscreen.texts {
            match i {
                LineType::SubScreen(_) => return Err(TextError::InternalConvertOneBozToTextError()),
                LineType::Text(val) => {
                    text_data.push(val.to_owned());
                }
            }
        }

        let lines: Vec<String> = splitlines(&mut io::Cursor::new(unpacked_render_string(
            &text_data,
            sub_screen.subscreen.width,
            sub_screen.subscreen.height,
            sub_screen.subscreen.type_of_border,
        )?));
        let mut result: Vec<Text> = vec![];
        for (i, v) in lines.into_iter().enumerate() {
            let mut value = Text::new(
                v.as_str(),
                sub_screen.start_line_number + i as u32,
                sub_screen.column,
                vec![],
            );
            result.push(value);
        }

        Ok(result)
    }
    */
    /*
    pub fn get_duplicates(input_hashmap: &HashMap<u32, Text>) -> Vec<HashMap<u32, Text>> {
        let deref_input = input_hashmap.clone();
        let input_array: BTreeMap<u32, Text> = deref_input.into_iter().collect::<BTreeMap<_, _>>();

        let mjjut frequency_dict = HashMap::new();
        for data in input_array.values() {
            if frequency_dict.contains_key(&data.line_number) {
                frequency_dict.insert(&data.line_number, frequency_dict[&data.line_number] + 1);
            } else {
                frequency_dict.insert(&data.line_number, 1);
            }
        }

        let mut seen_values = HashMap::new();
        let mut groups = Vec::new();
        let mut current_group = HashMap::new();

        for (id, data) in input_array.iter() {
            let entry = seen_values.entry(&data.line_number).or_insert(0);
            *entry += 1;
            if *entry != frequency_dict[&data.line_number] {
                current_group.insert(*id, data.clone());
            } else {
                current_group.insert(*id, data.clone());
                groups.push(current_group);
                current_group = HashMap::new();
            }
        }

        groups
    }
    */
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
        println!("ahdhwiudhuh");
        println!("{:#?}", utils::handle_same_line_text_and_ansi(text_data));
    }
}
