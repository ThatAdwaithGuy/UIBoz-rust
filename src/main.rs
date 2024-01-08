mod animations;
mod draw_screen;
mod errors;
mod event_loop;
use draw_screen::*;
use std::io::{self, Read};

fn main() -> Result<(), errors::TextError> {
    let text_data = SubScreen {
        subscreen: Screen {
            texts: vec![
                TextType::Text(Text::new("@", 1, 1, vec![])),
                TextType::SubScreen(SubScreen {
                    subscreen: Screen {
                        texts: vec![TextType::Text(Text::new("@", 1, 1, vec![]))],
                        width: 10,
                        height: 1,
                        type_of_border: TypeOfBorder::CurvedBorders,
                    },
                    start_line_number: 5,
                    column: 10,
                }),
            ],
            width: 10,
            height: 1,
            type_of_border: TypeOfBorder::CurvedBorders,
        },
        start_line_number: 5,
        column: 10,
    };
    println!("{:#?}", utils::convert_subscreen_to_texts(&text_data));
    Ok(())
}
