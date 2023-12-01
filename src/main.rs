use draw_boz::opts;

mod draw_boz;

// The Idea is to have

/*
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

    for i in 0  ..bigger_list.len() {
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

    Ok(String::from_iter(output_list))
}
*/

fn func(a: i32, b: i32) -> i32 {
    let result = a + b;
    result
}

fn overlay(lst: Vec<&str>) -> Result<String, draw_boz::TextError> {
    let mut last_element = lst[0].to_string();
    for i in 1..lst.len() {
        last_element = draw_boz::overlay_2_str(&last_element, lst[i])?;
    }
    Ok(last_element.to_owned())
}

fn main() -> Result<(), draw_boz::TextError> {
    let lst = vec![opts::Opts::Bold(true)];
    let mut lol = "".to_string();
    match draw_boz::test(lst) {
        Ok(val) => {
            lol = val;
        }
        Err(err) => {
            panic!("{}", err);
        }
    }

    println!(r#"{}"#, lol);

    Ok(())
}
