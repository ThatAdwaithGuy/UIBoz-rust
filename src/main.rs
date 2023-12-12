use std::collections::HashMap;

use draw_boz::{
    opts::{self, Opts},
    PrivateText,
};

use crate::draw_boz::opts::{parse_opts, Colors};
mod animations;
mod draw_boz;
mod errors;
mod event_loop;

/*
all_values:
{
    3: PrivateText {
        text: "                              \u{1b}[38;2;255;000;000m\u{1b}[0000000000000022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022mhell\u{1b}[0m",
        line_number: 10,
        column: 30,
    },
    0: PrivateText {
        text: "                    \u{1b}[001m\u{1b}[0000000000000022m\u{1b}[0000000000000022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022mhello\u{1b}[0m",
        line_number: 5,
        column: 20,
    },
    4: PrivateText {
        text: "          \u{1b}[0000000000000022m\u{1b}[0000000000000022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022mthis is hell\u{1b}[0m",
        line_number: 15,
        column: 10,
    },
    1: PrivateText {
        text: "          \u{1b}[001m\u{1b}[0000000000000022m\u{1b}[0000000000000022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022mhello\u{1b}[0m",
        line_number: 5,
        column: 10,
    },
    2: PrivateText {
        text: "          \u{1b}[38;2;255;000;000m\u{1b}[0000000000000022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022mhell\u{1b}[0m",
        line_number: 10,
        column: 10,
    },
}


duplicate_values:
[
    {
        0: PrivateText {
            text: "                    \u{1b}[001m\u{1b}[0000000000000022m\u{1b}[0000000000000022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022mhello\u{1b}[0m",
            line_number: 5,
            column: 20,
        },
        2: PrivateText {
            text: "          \u{1b}[38;2;255;000;000m\u{1b}[0000000000000022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022mhell\u{1b}[0m",
            line_number: 10,
            column: 10,
        },
        1: PrivateText {
            text: "          \u{1b}[001m\u{1b}[0000000000000022m\u{1b}[0000000000000022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022mhello\u{1b}[0m",
            line_number: 5,
            column: 10,
        },
    },
    {
        4: PrivateText {
            text: "          \u{1b}[0000000000000022m\u{1b}[0000000000000022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022mthis is hell\u{1b}[0m",
            line_number: 15,
            column: 10,
        },
    },
    {
        3: PrivateText {
            text: "                              \u{1b}[38;2;255;000;000m\u{1b}[0000000000000022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022m\u{1b}[022mhell\u{1b}[0m",
            line_number: 10,
            column: 30,
        },
    },
]

*/

fn main() {
    let text_data = vec![
        draw_boz::Text {
            text: "hello",
            line_number: 5,
            column: 20,
            opts: vec![Opts::Bold(Some(true))],
        },
        draw_boz::Text {
            text: "hell",
            line_number: 10,
            column: 10,
            opts: vec![
                Opts::ForeColor(Some(Colors::Red)),
                Opts::ForeColor(Some(Colors::Black)),
            ],
        },
        draw_boz::Text {
            text: "hello",
            line_number: 5,
            column: 10,
            opts: vec![Opts::Bold(Some(true))],
        },
        draw_boz::Text {
            text: "hell",
            line_number: 10,
            column: 30,
            opts: vec![
                Opts::ForeColor(Some(Colors::Red)),
                Opts::ForeColor(Some(Colors::Black)),
            ],
        },
        draw_boz::Text {
            text: "this is hell",
            line_number: 15,
            column: 10,
            opts: vec![],
        },
    ];

    let mut all_values_test: HashMap<i32, PrivateText> = HashMap::new();
    all_values_test.insert(
        3,
        PrivateText {
            text: "            hell".to_string(),
            line_number: 10,
            column: 30,
        },
    );

    all_values_test.insert(
        0,
        PrivateText {
            text: "hello".to_string(),
            line_number: 5,
            column: 20,
        },
    );

    all_values_test.insert(
        4,
        PrivateText {
            text: "this is hell".to_string(),
            line_number: 15,
            column: 10,
        },
    );
    all_values_test.insert(
        2,
        PrivateText {
            text: "hell".to_string(),
            line_number: 10,
            column: 10,
        },
    );
    // all_values_test.insert(
    //     1,
    //     PrivateText {
    //         text: "hello".to_string(),
    //         line_number: 5,
    //         column: 10,
    //     },
    // );
    //
    //println!(
    //    "{:#?}",
    //    draw_boz::get_duplicates(&draw_boz::generate_all_values(&text_data))
    //);

    println!(
        "{:#?}",
        draw_boz::handle_duplicates_and_ansi_codes(&draw_boz::generate_all_values(&text_data))
    );

    let a = draw_boz::Boz::new(text_data, true, 16, 52);
    // let av = match a.render_string() {
    //     Ok(val) => println!("{:#?}", draw_boz::get_duplicates(&val)),
    //     Err(err) => {
    //         panic!("{:?}", err)
    //     }
    // };
}
