#![feature(thin_box)]

mod renderer;
mod nodes;
mod errors;
mod pages;
mod style;
mod storage;
mod world;


fn fill(slice: &[style::TextStyle]) -> [style::TextStyle; 12] {
    let mut arr = [style::TextStyle::Bold(false); 12];
    arr[..slice.len()].copy_from_slice(slice);
    arr
}

fn main() {}
