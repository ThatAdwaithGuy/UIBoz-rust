use std::collections::HashMap;
mod components;
use draw_boz::{
    opts::{self, TextOpts},
    PrivateText,
};
use errors::TextError;

use crate::draw_boz::opts::{parse_text_opts, Colors};
mod animations;
mod draw_boz;
mod errors;
mod event_loop;

struct Jello;

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

fn main() -> Result<(), TextError> {
    let text_data = vec![
        draw_boz::TextType::Text(draw_boz::Text {
            text: "hello",
            line_number: 1,
            column: 5,
            opts: vec![],
        }),
        draw_boz::TextType::Boz(draw_boz::NestedBoz {
            boz: draw_boz::Boz {
                text_data: vec![draw_boz::TextType::Text(draw_boz::Text {
                    text: "im in a BOZ",
                    line_number: 1,
                    column: 4,
                    opts: vec![],
                })],
                height: 1,
                width: 52,
                type_of_border: draw_boz::TypeOfBorder::CurvedBorders,
            },
            start_line_number: 5,
            column: 10,
        }),
    ];

    println!("{:#?}", draw_boz::parse_boz_to_text(&text_data));

    Ok(())
}
