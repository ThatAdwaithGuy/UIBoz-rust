#![feature(thin_box)]
#![feature(unsize)]
use crate::renderer::style;
use crate::renderer::sub_win::*;
use crate::renderer::window::*;
use crate::renderer::window_renderer::*;
use crate::renderer::*;
use errors;
use frontend;
use std::{ops::Sub, vec};
use style;
fn main() -> Result<(), errors::TextError> {
    let first_window = NestedWindow::new(vec![], 1, 1, TypeOfBorder::CurvedBorders);
    let first_sub_window = SubWindow::new(first_window, 1, 0);
    let second_window = NestedWindow::new(
        vec![TextType::SubWindow(first_sub_window)],
        3,
        3,
        TypeOfBorder::CurvedBorders,
    );
    let second_sub_window = SubWindow::new(second_window, 1, 0);
    println!(
        "{}",
        Window::new(
            collapse_sub_window(second_sub_window.clone(), 1)?
                .iter()
                .map(|x| TextType::Text(x.clone()))
                .collect(),
            56,
            12,
            TypeOfBorder::CurvedBorders
        )
        .render()?
    );
    let third_window = NestedWindow::new(
        vec![TextType::SubWindow(second_sub_window)],
        3,
        3,
        TypeOfBorder::CurvedBorders,
    );
    let third_sub_window = SubWindow::new(third_window, 1, 0);

    println!(
        "{}",
        Window::new(
            collapse_sub_window(third_sub_window.clone(), 1)?
                .iter()
                .map(|x| TextType::Text(x.clone()))
                .collect(),
            56,
            12,
            TypeOfBorder::CurvedBorders
        )
        .render()?
    );

    let fourth_window = NestedWindow::new(
        vec![TextType::SubWindow(third_sub_window)],
        6,
        6,
        TypeOfBorder::CurvedBorders,
    );
    let fourth_sub_window = SubWindow::new(fourth_window, 1, 0);

    println!(
        "{}",
        Window::new(
            collapse_sub_window(fourth_sub_window.clone(), 1)?
                .iter()
                .map(|x| TextType::Text(x.clone()))
                .collect(),
            56,
            12,
            TypeOfBorder::CurvedBorders
        )
        .render()?
    );
    Ok(())
}
