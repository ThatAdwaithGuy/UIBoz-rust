// mod draw_boz;

pub fn make_lists_equal_length(list1: Vec<char>, list2: Vec<char>) -> (Vec<char>, Vec<char>) {
    let mut bigger_list: Vec<char> = vec![];
    let mut smaler_list: Vec<char> = vec![];

    if list1.len() > list2.len() {
        smaler_list = list2.clone();
        bigger_list = list1.clone();
    } else if list1.len() < list2.len() {
        smaler_list = list1.clone();
        bigger_list = list2.clone();
    }

    let padding: usize = bigger_list.len() - smaler_list.len();
    let add_pad: Vec<char> = vec![' '; padding];
    let output = (bigger_list, smaler_list.extend(add_pad));
    output
}

fn main() {
    let (string1, string2) = make_lists_equal_length(
        "             hello".chars().collect(),
        "  hello".chars().collect(),
    );
    println!("str1 = {:?}, str2 = {:?}", string1, string2);
}
/* println!(
    "{:?}",
    draw_boz::get_duplicates(vec![
        draw_boz::PrivateText {
            text: "hello",
            line_number: &10,
            column: &10
        },
        draw_boz::PrivateText {
            text: "jj",
            line_number: &10,
            column: &10
        },
        draw_boz::PrivateText {
            text: "ha",
            line_number: &10,
            column: &1
        },
        draw_boz::PrivateText {
            text: "ha",
            line_number: &1,
            column: &1
        }
    ])
); */
