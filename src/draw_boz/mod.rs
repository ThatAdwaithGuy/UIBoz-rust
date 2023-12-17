use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ops::Deref,
};

use crate::draw_boz;

use super::errors::TextError;

pub mod opts;
use opts::TextOpts;
/*def get_duplicates(dict: dict[int, PrivateText]):
frequency_dict = {}
for i in dict.values():
           frequency_dict[i.line_number] += 1
    else:
        frequency_dict[i.line_number] = 1

seen_sets = set()
groups = []
current_group = {}
for i, v in dict.items():
    if v.line_number in seen_sets:
        current_group[i] = v
        groups.append(current_group)
        current_group = {}
    else:
        seen_sets.add(v.line_number)
        current_group[i] = v
return groups*/

pub fn get_duplicates(input_hashmap: &HashMap<i32, PrivateText>) -> Vec<HashMap<i32, PrivateText>> {
    // let mut values: Vec<PrivateText> = input_hashmap.values().cloned().collect();
    // values.sort_by_key(|text| text.line_number);
    // let input_array: HashMap<_, _> = values
    //     .into_iter()
    //     .map(|text| (text.line_number, text))
    //    .collect();
    let deref_input = input_hashmap.clone();
    let input_array: BTreeMap<i32, PrivateText> =
        deref_input.into_iter().collect::<BTreeMap<_, _>>();

    let mut frequency_dict = HashMap::new();
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
        // Use entry method to get or create key-value pairs in seen_values
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

    let output = (bigger_list, smaler_list);
    output
}

pub fn overlay_2_str<'a>(str1: &'a str, str2: &'a str) -> Result<String, TextError> {
    let (bigger_str, smaler_str) = if str1.len() > str2.len() {
        (str1, str2)
    } else if str1.len() < str2.len() {
        (str2, str1)
    } else {
        (str1, str2)
    };

    let mut bigger_list: Vec<char> = bigger_str.chars().collect::<Vec<char>>();
    let mut smaler_list: Vec<char> = smaler_str.chars().collect::<Vec<char>>();

    (bigger_list, smaler_list) = make_lists_equal_length(bigger_list.clone(), smaler_list.clone());

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

// This stupid function made me waste 2 hours of my life
// butttttt my noggin solved it 4 FRICKING LINES
fn overlay<'a>(lst: Vec<&str>) -> Result<String, TextError> {
    let mut last_element = lst[0].to_string();
    for i in 1..lst.len() {
        last_element = overlay_2_str(&last_element, lst[i])?;
    }
    Ok(last_element.to_owned())
}

pub fn generate_all_values<'a>(text_data: &Vec<Text<'a>>) -> HashMap<i32, PrivateText> {
    let mut sorted_text_data = text_data.clone();
    sorted_text_data.sort_by_key(|a| a.line_number);

    let mut all_values: HashMap<i32, PrivateText> = HashMap::new();
    for (i, v) in sorted_text_data.iter().enumerate() {
        let formatted_opts = opts::parse_text_opts(v.opts.clone());

        all_values.insert(
            i.try_into().unwrap(),
            PrivateText {
                text: format!("{}{}\x1b[0m", formatted_opts, v.text),
                line_number: v.line_number,
                column: v.column,
            },
        );
    }
    all_values
}

fn replace_none_with_line_numbers(
    width_of_line: i32,
    vec_with_struct: Vec<PrivateText>,
) -> Vec<LineTypes> {
    let result: Vec<LineTypes> = (0..width_of_line)
        .map(|index| {
            vec_with_struct
                .iter()
                .find(|pt| pt.line_number == (index + 1))
                .map(|private_text| {
                    if private_text.column < 9999 {
                        LineTypes::SingleText(private_text.clone())
                    } else {
                        LineTypes::MultiText(private_text.clone())
                    }
                })
                .unwrap_or(LineTypes::Empty)
        })
        .collect();

    result
}

pub fn render_string_unpacked<'a>(
    text_data: &Vec<Text<'a>>,
    height: i32,
    width: i32,
    type_of_border: TypeOfBorder,
) -> Result<String, TextError> {
    let mut output_string: String = "".to_string();
    match type_of_border {
        TypeOfBorder::NoBorders => output_string.push_str("\n"),
        TypeOfBorder::CurvedBorders => {
            output_string.push_str(format!("{}\n", "".repeat(width as usize)).as_str())
        }
        TypeOfBorder::SquareBorders => {
            output_string.push_str(format!("{}\n", "─".repeat(width as usize)).as_str())
        }
    }

    let all_values = handle_duplicates_and_ansi_codes(&generate_all_values(&text_data))?;

    let complete_vec: Vec<LineTypes> = replace_none_with_line_numbers(height, all_values);

    // <Error handling>
    let mut seen_pairs = HashSet::new();
    for i in text_data {
        let pair = (i.line_number, i.column);
        if seen_pairs.contains(&pair) {
            return Err(TextError::DuplicateText(i.text.to_string()));
        }
        seen_pairs.insert(pair);
    }

    for i in text_data {
        // Length of the text (without ASNI) + column and 88 because of ANSI, compared to width
        // times 88 because of ANSI
        if (i.text.len() as i32 - 123) + i.column >= width {
            return Err(TextError::LeftBounds(i.text.to_string()));
        }
    }
    // <\Error handling>

    for i in complete_vec {
        match i {
            LineTypes::SingleText(val) => match type_of_border {
                TypeOfBorder::CurvedBorders | TypeOfBorder::SquareBorders => output_string
                    .push_str(
                        format!(
                            "│{}{}│\n",
                            val.text,
                            " ".repeat((width - (val.text.len() as i32 - 78)) as usize)
                        )
                        .as_str(),
                    ),

                TypeOfBorder::NoBorders => {
                    output_string.push_str(format!("{}\n", val.text).as_str())
                }
            },
            LineTypes::MultiText(val) => match type_of_border {
                TypeOfBorder::SquareBorders | TypeOfBorder::CurvedBorders => output_string
                    .push_str(
                        format!(
                            "│{}{}│\n",
                            val.text,
                            " ".repeat(
                                (width - val.text.len() as i32 - ((100000 - val.column) * 78))
                                    as usize
                            )
                            .as_str()
                        )
                        .as_str(),
                    ),
                TypeOfBorder::NoBorders => {
                    output_string.push_str(format!("{}\n", val.text).as_str())
                }
            },
            LineTypes::Empty => match type_of_border {
                TypeOfBorder::NoBorders => output_string.push_str("\n"),
                TypeOfBorder::CurvedBorders | TypeOfBorder::SquareBorders => output_string
                    .push_str(format!("│{}│\n", " ".repeat(width as usize).as_str()).as_str()),
            },
        }
    }

    match type_of_border {
        TypeOfBorder::NoBorders => output_string.push_str("\n"),
        TypeOfBorder::CurvedBorders => {
            output_string.push_str(format!("╰{}╯\n", "─".repeat(width as usize)).as_str())
        }
        TypeOfBorder::SquareBorders => {
            output_string.push_str(format!("└{}┘\n", "─".repeat(width as usize)).as_str())
        }
    }

    Ok(output_string)
}

pub fn parse_boz_to_text<'a>(text: &Vec<TextType<'a>>) -> Result<Vec<Text<'a>>, TextError> {
    let output: Vec<Text<'a>> = vec![];

    let nested_boz: Vec<_> = text
        .iter()
        .filter(|boz| matches!(boz, TextType::Boz(_)))
        .map(|x| -> Result<Text, TextError> {
            if let TextType::Boz(data) = x {
                let mut unpacked_text_data: Vec<Text<'_>> = data
                    .boz
                    .text_data
                    .iter()
                    .map(|y| {
                        if let TextType::Text(text) = y {
                            text.to_owned()
                        } else {
                            Text {
                                text: "",
                                line_number: 10,
                                column: 10,
                                opts: vec![],
                            }
                        }
                    })
                    .collect::<Vec<Text<'a>>>();
                println!(
                    "TEST: {:#?}",
                    handle_duplicates_and_ansi_codes(&generate_all_values(&unpacked_text_data))
                );
                for (i, v) in unpacked_text_data.iter_mut().enumerate() {
                    println!("{}", data.start_line_number);
                    v.line_number += data.start_line_number + (i as i32);
                    v.column += data.column;
                }
                /*
                let curved_border_top_border = format!("╭{}╮", border_lines);
                let square_borders_top_border = format!("┌{}┐", border_lines);
                match data.boz.type_of_border {
                    TypeOfBorder::NoBorders => unpacked_text_data.insert(
                        0,
                        Text {
                            text: "",
                            line_number: data.start_line_number,
                            column: data.column,
                            opts: vec![],
                        },
                    ),
                    TypeOfBorder::CurvedBorders => unpacked_text_data.insert(
                        0,
                        Text {
                            text: curved_border_top_border.as_str(),
                            line_number: data.start_line_number,
                            column: data.column,
                            opts: vec![],
                        },
                    ),
                    TypeOfBorder::SquareBorders => unpacked_text_data.insert(
                        0,
                        Text {
                            text: square_borders_top_border.as_str(),
                            line_number: data.start_line_number,
                            column: data.column,
                            opts: vec![],
                        },
                    ),
                }

                let curved_border_bottom_border = format!("╰{}╯", border_lines);
                let square_borders_bottom_border = format!("┌{}┐", border_lines);
                match data.boz.type_of_border {
                    TypeOfBorder::NoBorders => unpacked_text_data.insert(
                        0,
                        Text {
                            text: "",
                            line_number: data.start_line_number
                                - data.boz.text_data.len() as i32
                                - 1,

                            column: data.column,
                            opts: vec![],
                        },
                    ),
                    TypeOfBorder::CurvedBorders => unpacked_text_data.insert(
                        0,
                        Text {
                            text: curved_border_bottom_border.as_str(),
                            line_number: data.start_line_number
                                - data.boz.text_data.len() as i32
                                - 1,
                            column: data.column,
                            opts: vec![],
                        },
                    ),
                    TypeOfBorder::SquareBorders => unpacked_text_data.insert(
                        0,
                        Text {
                            text: square_borders_bottom_border.as_str(),
                            line_number: data.start_line_number
                                - data.boz.text_data.len() as i32
                                - 1,
                            column: data.column,
                            opts: vec![],
                        },
                    ),
                }*/
                println!("A: {:#?}", unpacked_text_data);

                Ok(Text {
                    text: "PLACEHOLDER",
                    line_number: 10,
                    column: 10,
                    opts: vec![],
                })
            } else {
                if let TextType::Text(data) = x {
                    Ok(data.to_owned())
                } else {
                    let text = Text {
                        text: "Looks like I HAVE A STUPID ERROR",
                        line_number: 10,
                        column: 10,
                        opts: vec![],
                    };
                    Ok(text)
                }
            }
        })
        .collect();
    println!("nested {:#?}", nested_boz);
    Ok(output)
}

// Forgive me SOILD, I let you down 😔
pub fn handle_duplicates_and_ansi_codes(
    all_values: &HashMap<i32, PrivateText>,
) -> Result<Vec<PrivateText>, TextError> {
    let mut output: Vec<PrivateText> = vec![];
    let mut sorted_vals: Vec<_> = get_duplicates(all_values)
        .iter()
        .map(|a| a.values().cloned().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for i in &mut sorted_vals {
        i.sort_by_key(|key| key.column);
    }
    let mut sorted_values = sorted_vals;

    for i in &mut sorted_values {
        if i.len() == 1 {
            continue;
        } else if i.len() != 1 {
            let mut sum = 0;
            for j in i.iter_mut() {
                j.column -= sum;
                sum += j.column;
            }
        }
    }

    for i in &sorted_values {
        // If the line has only one Text.
        if i.len() == 1 {
            let val = &i[0];
            output.push(PrivateText {
                text: format_column(val.text.as_str(), val.column),
                line_number: val.line_number,
                column: val.column,
            });
        } else if i.len() != 1 {
            let mut result = "".to_string();
            let mut count = 0;
            for j in i {
                let mut last = "".to_string();
                if last == "" {
                    last.push_str(format_column(j.text.as_str(), j.column).as_str());
                    result.push_str(format_column(j.text.as_str(), j.column).as_str());
                    count += 1;
                } else if last != "" {
                    let format_column =
                        format_column(j.text.as_str(), j.column + last.len() as i32);
                    last.push_str(format_column.as_str());
                    result.push_str(format_column.as_str());
                    count += 1;
                }
            }

            output.push(PrivateText {
                text: result,
                line_number: i[0].clone().line_number,
                column: 100000 + count,
            });
        }
    }

    Ok(output)
}

#[derive(Debug, Clone)]
pub struct Text<'a> {
    pub text: &'a str,
    pub line_number: i32,
    pub column: i32,
    pub opts: Vec<TextOpts>,
}

#[derive(Debug, Clone)]
pub enum TypeOfBorder {
    NoBorders,
    CurvedBorders,
    SquareBorders,
}

#[derive(Debug, Clone)]
pub enum TextType<'a> {
    Text(Text<'a>),
    Boz(NestedBoz<'a>),
}

#[derive(Debug, Clone)]
pub struct NestedBoz<'a> {
    pub boz: Boz<'a>,
    pub start_line_number: i32,
    pub column: i32,
}
#[derive(Debug, Clone)]
pub struct Boz<'a> {
    pub text_data: Vec<TextType<'a>>,
    pub height: i32,
    pub width: i32,
    pub type_of_border: TypeOfBorder,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrivateText {
    pub text: String,
    pub line_number: i32,
    pub column: i32,
}

#[derive(Clone)]
enum LineTypes {
    Empty,
    SingleText(PrivateText),
    // the data is for how many text are there in the line
    MultiText(PrivateText),
}

// Implementations
impl<'a> Boz<'a> {
    pub fn new(
        text_data: Vec<TextType<'a>>,
        height: i32,
        width: i32,
        type_of_border: TypeOfBorder,
    ) -> Boz<'a> {
        Boz {
            text_data,
            height,
            width,
            type_of_border,
        }
    }

    pub fn render_string(&'a self) -> Result<String, TextError> {
        let textized_text_data = parse_boz_to_text(&self.text_data)?;
        let mut output_string: String = "".to_string();
        match self.type_of_border {
            TypeOfBorder::NoBorders => output_string.push_str("\n"),
            TypeOfBorder::CurvedBorders => {
                output_string.push_str(format!("╭{}╮\n", "─".repeat(self.width as usize)).as_str())
            }
            TypeOfBorder::SquareBorders => {
                output_string.push_str(format!("┌{}┐\n", "─".repeat(self.width as usize)).as_str())
            }
        }

        let all_values =
            handle_duplicates_and_ansi_codes(&generate_all_values(&textized_text_data))?;

        let complete_vec: Vec<LineTypes> = replace_none_with_line_numbers(self.height, all_values);

        // <Error handling>
        let mut seen_pairs = HashSet::new();
        for i in &textized_text_data {
            let pair = (i.line_number, i.column);
            if seen_pairs.contains(&pair) {
                return Err(TextError::DuplicateText(i.text.to_string()));
            }
            seen_pairs.insert(pair);
        }

        for i in &textized_text_data {
            // Length of the text (without ASNI) + column and 88 because of ANSI, compared to width
            // times 88 because of ANSI
            if (i.text.len() as i32 - 123) + i.column >= self.width {
                return Err(TextError::LeftBounds(i.text.to_string()));
            }
        }
        // <\Error handling>

        for i in complete_vec {
            match i {
                LineTypes::SingleText(val) => match self.type_of_border {
                    TypeOfBorder::CurvedBorders | TypeOfBorder::SquareBorders => output_string
                        .push_str(
                            format!(
                                "│{}{}│\n",
                                val.text,
                                " ".repeat((self.width - (val.text.len() as i32 - 78)) as usize)
                            )
                            .as_str(),
                        ),

                    TypeOfBorder::NoBorders => {
                        output_string.push_str(format!("{}\n", val.text).as_str())
                    }
                },
                LineTypes::MultiText(val) => match self.type_of_border {
                    TypeOfBorder::SquareBorders | TypeOfBorder::CurvedBorders => output_string
                        .push_str(
                            format!(
                                "│{}{}│\n",
                                val.text,
                                " ".repeat(
                                    (self.width
                                        - val.text.len() as i32
                                        - ((100000 - val.column) * 78))
                                        as usize
                                )
                                .as_str()
                            )
                            .as_str(),
                        ),
                    TypeOfBorder::NoBorders => {
                        output_string.push_str(format!("{}\n", val.text).as_str())
                    }
                },
                LineTypes::Empty => match self.type_of_border {
                    TypeOfBorder::NoBorders => output_string.push_str("\n"),
                    TypeOfBorder::CurvedBorders | TypeOfBorder::SquareBorders => output_string
                        .push_str(
                            format!("│{}│\n", " ".repeat(self.width as usize).as_str()).as_str(),
                        ),
                },
            }
        }

        match self.type_of_border {
            TypeOfBorder::NoBorders => output_string.push_str("\n"),
            TypeOfBorder::CurvedBorders => {
                output_string.push_str(format!("╰{}╯\n", "─".repeat(self.width as usize)).as_str())
            }
            TypeOfBorder::SquareBorders => {
                output_string.push_str(format!("└{}┘\n", "─".repeat(self.width as usize)).as_str())
            }
        }

        Ok(output_string)
    }
}
