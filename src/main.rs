use draw_boz::opts::{self, Opts};

use crate::draw_boz::opts::{parse_opts, Colors};
mod animations;
mod draw_boz;
mod errors;
mod event_loop;

// The Idea is to have

fn main() {
    let text_data = vec![
        draw_boz::Text {
            text: "hello",
            line_number: 5,
            column: 10,
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
    ];

    println!(
        "{:?}",
        parse_opts(vec![
            Opts::ForeColor(Some(Colors::Red)),
            Opts::BackGroundColor(Some(Colors::Black)),
            Opts::ForeColor(Some(Colors::Red)),
            Opts::BackGroundColor(Some(Colors::Black)),
        ])
    );

    let a = draw_boz::Boz::new(text_data, true, 16, 52);
    // let av = match a.render_string() {
    //     Ok(val) => println!("{:?}", val),
    //     Err(err) => {
    //         panic!("{:?}", err)
    //     }
    // };
}
