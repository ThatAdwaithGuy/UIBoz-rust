use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TextError {
    #[error("Looks Like {0} and {1} are on top of each other.")]
    TextOverlayed(String, String),
}

#[derive(Clone)]
struct PrivateText<'a> {
    pub text: &'a str,
    pub line_number: &'a usize,
    pub column: &'a usize,
}

fn get_duplicates<'a>(lst: Vec<PrivateText<'a>>) -> Vec<Vec<PrivateText<'a>>> {
    let mut frequency_hashmap: HashMap<&usize, i32> = HashMap::new();

    for i in lst.iter() {
        if frequency_hashmap.contains_key(&i.column) {
            *frequency_hashmap.get_mut(&i.column).unwrap() += 1;
        } else {
            frequency_hashmap.insert(i.column, 1);
        }
    }

    let mut seen_sets: Vec<&usize> = vec![];
    let mut groups: Vec<Vec<PrivateText<'a>>> = vec![];
    let mut current_group: Vec<PrivateText<'a>> = vec![];
    let lst_copy = lst.clone();

    for i in lst_copy {
        if seen_sets.contains(&i.column) {
            current_group.push(i);
            groups.push(current_group);
            current_group = vec![];
        } else {
            seen_sets.push(i.column);
            current_group.push(i);
        }
    }
    groups
}

fn format(istring: &str, column: usize) -> String {
    let binding = " ".repeat(column);
    let fstring = binding.as_str();
    format!("{}{}", fstring, istring)
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

pub fn apply_func(lst: Vec<String>) -> Result<String, TextError> {
    let mut temp = vec!["".to_string()];

    while lst.len() > 1 {
        let mut temp_lst: Vec<String> = Vec::new();
        for i in (0..lst.len()).step_by(2) {
            if i + 1 < lst.len() {
                let buffer = overlay_2_str(lst[1].as_str(), lst[i + 1].as_str())?;
                println!("{}", buffer);
                temp_lst.push(buffer);
            } else {
                let buff = &lst[i];
                temp_lst.push(buff.to_string());
            }
        }
        temp = temp_lst;
    }
    Ok(temp[0].to_owned())
}

pub struct Text<'a> {
    id: i32,
    pub text: &'a str,
    pub line_number: i32,
    pub column: i32,
    pub opts: HashMap<&'a str, &'a str>,
}

pub struct Boz<'a> {
    pub text_data: Vec<Text<'a>>,
    pub borders: bool,
    pub height: usize,
    pub width: usize,
}

// Implementations
impl<'a> Boz<'a> {
    pub fn new(
        &mut self,
        text_data: Vec<Text<'a>>,
        borders: bool,
        height: usize,
        width: usize,
    ) -> Boz<'a> {
        Boz {
            text_data,
            borders,
            height,
            width,
        }
    }

    pub fn render_string(&'a self) {
        let mut output_string: &str = "";
        let mut complete_vec: Vec<Option<PrivateText>> = vec![None; self.height];
        let mut all_values: Vec<PrivateText> = vec![];
        let mut duplicate_values: Vec<Vec<PrivateText<'_>>> = get_duplicates(all_values);
    }
}
