#![feature(thin_box)]

mod errors;
mod nodes;
use nodes::flex::Flex;
mod pages;
mod renderer;
mod storage;
mod style;
mod world;

fn fill(slice: &[style::TextStyle]) -> [style::TextStyle; 12] {
    let mut arr = [style::TextStyle::Bold(false); 12];
    arr[..slice.len()].copy_from_slice(slice);
    arr
}

fn main() {
    use nodes::flex;

    let texts = vec![renderer::sub_win::TextType::Text(
        renderer::window_renderer::Text::new(
            "@, hello world hehe, lololololololololololololololololol",
            1,
            1,
            &[],
        ),
    )];
    let window = renderer::window::Window {
        texts,
        width: 56,
        height: 12,
        type_of_border: crate::renderer::window_renderer::TypeOfBorder::CurvedBorders,
    };

    let flexed = window.flex(24, 12);
    assert!(flexed.is_some());
}
