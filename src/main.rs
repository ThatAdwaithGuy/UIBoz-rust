mod draw_boz;

fn main() {
    println!(
        "{}",
        draw_boz::overlay_2_str("             hello", "  hello")
    )
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
