use std::collections::{BTreeMap, HashMap, HashSet};

use crate::draw_boz;

use super::errors::TextError;

pub mod opts;
use opts::Opts;
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

#[derive(Debug, Clone)]
pub struct Text<'a> {
    pub text: &'a str,
    pub line_number: i32,
    pub column: i32,
    pub opts: Vec<Opts>,
}

pub struct Boz<'a> {
    pub text_data: Vec<Text<'a>>,
    pub borders: bool,
    pub height: i32,
    pub width: i32,
}

#[derive(Debug, Clone)]
pub struct PrivateText {
    pub text: String,
    pub line_number: i32,
    pub column: i32,
}

pub fn generate_all_values<'a>(text_data: &Vec<Text<'a>>) -> HashMap<i32, PrivateText> {
    let mut sorted_text_data = text_data.clone();

    sorted_text_data.sort_by_key(|text| text.line_number);

    let mut all_values: HashMap<i32, PrivateText> = HashMap::new();
    for (i, v) in sorted_text_data.iter().enumerate() {
        let formatted_opts = opts::parse_opts(v.opts.clone());

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
// Forgive me SOILD, I let you down 😔
pub fn handle_duplicates_and_ansi_codes(
    all_values: &HashMap<i32, PrivateText>,
) -> Result<Vec<PrivateText>, TextError> {
    let mut output: Vec<PrivateText> = vec![];
    for i in get_duplicates(all_values) {
        if i.len() == 1 {
            let val = i.keys().next().and_then(|key| i.get(key)).unwrap();

            output.push(PrivateText {
                text: format_column(val.text.as_str(), val.column),
                line_number: val.line_number,
                column: val.column,
            })
        } else if i.len() > 1 {
            let mut last = 0;
            let values: Vec<PrivateText> = i.values().cloned().collect();
            let strings: Vec<String> = values
                .iter()
                .map(|a| {
                    if last == a.text.len() as i32 + a.column {
                        last += a.text.len() as i32 + a.column;
                        format_column(a.text.as_str(), a.column)
                    } else if last != a.text.len() as i32 + a.column {
                        let buff = format_column(a.text.as_str(), a.column + last);
                        last += a.text.len() as i32 + a.column;
                        buff
                    } else {
                        "".to_string()
                    }
                })
                .collect();
            let strs: Vec<&str> = strings.iter().map(|string| string.as_str()).collect();
            let processed_string = overlay(strs)?;
            output.push(PrivateText {
                text: processed_string,
                line_number: i
                    .keys()
                    .next()
                    .and_then(|key| i.get(key))
                    .unwrap()
                    .line_number,
                column: 100000 + i.len() as i32, // i encoded how many times does the line have
                                                 // text. 100000 + how many times.
            })
        }
    }
    // println!("Output: {:#?}", output);
    Ok(output)
}
// Future adwaith, The problem is the text is added at the start and not after the
// end of the last text. so i added a buffer called lat at line 201 and implement
// to format_column.

// Add 92
//                     \u{1b}[001m\u{1b}[0000000000000022m\u{1b}[0000000000000022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022mhello\u{1b}[0m
//           \u{1b}[001m\u{1b}[0000000000000022m\u{1b}[0000000000000022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022mhello\u{1b}[0m

//

// Implementations
impl<'a> Boz<'a> {
    pub fn new(text_data: Vec<Text<'a>>, borders: bool, height: i32, width: i32) -> Boz<'a> {
        Boz {
            text_data,
            borders,
            height,
            width,
        }
    }

    pub fn render_string(&'a self) -> Result<HashMap<i32, PrivateText>, TextError> {
        let real_width = self.width * 88;
        let _output_string: &str = "";
        let _complete_vec: Vec<Option<PrivateText>> = vec![None; self.height.try_into().unwrap()];

        let all_values = generate_all_values(&self.text_data);
        let duplicate_values = get_duplicates(&all_values);

        // <Error handling>
        let mut seen_pairs = HashSet::new();
        for i in &self.text_data {
            let pair = (i.line_number, i.column);
            if seen_pairs.contains(&pair) {
                return Err(TextError::DuplicateText(i.text.to_string()));
            }
            seen_pairs.insert(pair);
        }

        for i in &self.text_data {
            // Length of the text (without ASNI) + column and 88 because of ANSI, compared to width
            // times 88 because of ANSI
            if (i.text.len() as i32 - 123) + i.column >= self.width {
                return Err(TextError::LeftBounds(i.text.to_string()));
            }
        }
        // <\Error handling>

        Ok(all_values)
    }
}
