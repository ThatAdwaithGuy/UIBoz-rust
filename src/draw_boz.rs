use std::collections::HashMap;

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
/*
fn make_list_equal_length<'a>(list1: &'a str, list2: &'a str) -> [&'a str] {
    let output_list1 = format!(
        "{}{}",
        list1,
        vec![list1; list2.len() - list1.len()].as_str()
    );
    let output_list2 = format!(
        "{}{}",
        list2,
        vec![list2; list1.len() - list2.len()].as_str()
    );
}
*/

pub fn overlay_2_str(string1: &str, string2: &str) -> String {
    let (smaller_string, bigger_string, lead_spaces) = if string1.len() > string2.len() {
        (string1, string2, string1.len())
    } else if string1.len() < string2.len() {
        (string2, string1, string2.len())
    } else {
        (string1, string2, 0)
    };

    let make_lists_equal_length = |list1: Vec<char>, list2: Vec<char>| -> (Vec<char>, Vec<char>) {
        (
            list1
                .iter()
                .cloned()
                .chain(std::iter::repeat(' ').take(list2.len() - list1.len()))
                .collect(),
            list2
                .iter()
                .cloned()
                .chain(std::iter::repeat(' ').take(list1.len() - list2.len()))
                .collect(),
        )
    };

    let mut smaller_list: Vec<char> = smaller_string.chars().collect();
    let mut bigger_list: Vec<char> = bigger_string.chars().collect();
    let mut output_list: Vec<char> = Vec::new();

    for i in 0..bigger_list.len() {
        if bigger_list[i] == ' ' && smaller_list[i] == ' ' {
            continue;
        } else if bigger_list[i] != ' ' && smaller_list[i] == ' ' {
            continue;
        } else if bigger_list[i] == ' ' && smaller_list[i] != ' ' {
            continue;
        } else if bigger_list[i] != ' ' && smaller_list[i] != ' ' {
            smaller_list.insert(0, ' ');
        }
    }

    let (smaler_list, bigger_list) = make_lists_equal_length(smaller_list, bigger_list);

    for i in 0..bigger_list.len() {
        if bigger_list[i] == ' ' && smaler_list[i] == ' ' {
            output_list.push(' ');
        } else if bigger_list[i] != ' ' && smaler_list[i] == ' ' {
            output_list.push(bigger_list[i]);
        } else if bigger_list[i] == ' ' && smaler_list[i] != ' ' {
            output_list.push(smaler_list[i]);
        }
    }

    output_list.iter().collect()
}

fn overlay_2_string(str1: &str, str2: &str) {
    let mut smaler_string: &str = "";
    let mut bigger_string: &str = "";

    if str1.len() >= str2.len() {
        smaler_string == str2;
        bigger_string == str1;
    }
}
fn overlay() {}

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

        // Error Handling

        // Ok(output_string)
    }
}
