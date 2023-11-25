// mod draw_boz;

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

pub fn overlay_2_str<'a>(str1: &'a str, str2: &'a str) -> Vec<char> {
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

    output_list
}

fn main() {
    let str1 = "        Hello";
    let str2 = "     hello";

    let list1 = vec![
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'H', 'e', 'l', 'l', 'o',
    ];

    let list2 = vec![' ', ' ', ' ', ' ', ' ', 'h', 'e', 'l', 'l', 'o'];

    let ls1 = overlay_2_str(str1, str2);

    println!("{}", str1);
    println!("{}", str2);

    println!();
    println!("{:?}", ls1);
}
