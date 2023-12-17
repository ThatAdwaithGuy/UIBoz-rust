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

fn main() -> Result<(), TextError> {
    let text_data = vec![
        draw_boz::TextType::Text(draw_boz::Text {
            text: "hello",
            line_number: 6,
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
