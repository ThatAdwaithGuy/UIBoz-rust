// use crate::draw_boz::opts::{parse_text_opts, Colors};
pub mod errors;
pub mod node;
pub mod style;
pub mod sub_win;
pub mod window;
fn main() -> Result<(), errors::TextError> {
    //let text_data = vec![
    //    draw_boz::boz::Text {
    //        text: "hello",
    //        line_number: 5,
    //        column: 30,
    //        opts: vec![],
    //    },
    //    draw_boz::boz::Text {
    //        text: "hellow",
    //        line_number: 5,
    //        column: 10,
    //        opts: vec![],
    //    },
    //    draw_boz::boz::Text {
    //        text: "hellow",
    //        line_number: 5,
    //        column: 1,
    //        opts: vec![],
    //    },
    //    draw_boz::boz::Text {
    //        text: "hello",
    //        line_number: 4,
    //        column: 30,
    //        opts: vec![],
    //    },
    //    draw_boz::boz::Text {
    //        text: "hellow",
    //        line_number: 4,
    //        column: 10,
    //        opts: vec![],
    //    },
    //    draw_boz::boz::Text {
    //        text: "hellow",
    //        line_number: 4,
    //        column: 1,
    //        opts: vec![],
    //    },
    //];

    //let boz = draw_boz::boz::Boz::new(
    //    text_data,
    //    false,
    //    10,
    //    52,
    //    draw_boz::boz::TypeOfBorder::CurvedBorders,
    //);

    //println!("{}", boz.render_string()?);

    Ok(())
}
