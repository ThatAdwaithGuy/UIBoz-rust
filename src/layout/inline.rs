use crate::{
    errors::{self, LayoutErrors},
    layout::{self, ElementType},
    renderer::{self, SubWindow, Text, TextType, TypeOfBorder, Window},
};

fn inline_render_same_height(
    layout: layout::Layout,
) -> Result<Vec<ElementType>, errors::LayoutErrors> {
    let mut elements: Vec<ElementType> = vec![];
    let line_number = layout.elements[0].element.line_number();
    let mut previous_column: u32 = 0;

    for element in layout.elements {
        if previous_column > layout.width {
            return Err(LayoutErrors::InlineTooManyElementsForWidth());
        }

        match element.element {
            ElementType::Text(text) => {
                let text_column = previous_column + element.left_padding;

                elements.push(ElementType::Text(Text::new_unchecked(
                    &text.text,
                    line_number,
                    text_column,
                    &text.style,
                )));

                previous_column += text_column + text.text_len() as u32;
            }
            ElementType::SubWindow(sub_window) => {
                let window_column = previous_column + element.left_padding;
                let window_width = sub_window.window.width;
                let type_of_border = sub_window.window.type_of_border;

                elements.push(ElementType::SubWindow(SubWindow::new(
                    sub_window.window,
                    line_number,
                    window_column,
                )));
                if let TypeOfBorder::No = type_of_border {
                    previous_column += window_column + window_width;
                } else {
                    // The +2 is to account for the borders
                    previous_column += window_column + window_width + 2;
                }
            }
        };
    }

    Ok(elements)
}

pub fn inline_render(layout: layout::Layout) -> Result<Vec<ElementType>, errors::LayoutErrors> {
    let max_height = layout
        .elements
        .iter()
        .map(|element| match &element.element {
            layout::ElementType::SubWindow(sub_window) => sub_window.window.height,
            layout::ElementType::Text(_) => 1,
        })
        .collect::<Vec<u32>>();

    if max_height.iter().any(|height| height == &max_height[0]) {
        return inline_render_same_height(layout);
    }

    let mut elements: Vec<ElementType> = vec![];
    let line_number = layout.elements[0].element.line_number();
    let mut previous_column: u32 = 0;

    for element in layout.elements {
        if previous_column > layout.width {
            return Err(LayoutErrors::InlineTooManyElementsForWidth());
        }

        match element.element {
            ElementType::Text(text) => {
                let text_len: u32 = text.text_len() as u32;
                let text_column = previous_column + element.left_padding;
                let win = Window {
                    texts: vec![TextType::Text(Text::new_unchecked(
                        &text.text,
                        0,
                        0,
                        &text.style,
                    ))],
                    width: text_len,
                    height: *max_height.iter().max().unwrap() as u32,
                    type_of_border: renderer::TypeOfBorder::No,
                };

                elements.push(ElementType::SubWindow(SubWindow::new(
                    win,
                    line_number,
                    previous_column,
                )));

                previous_column += text_column + text_len;
            }
            ElementType::SubWindow(_) => {}
        }
    }

    Ok(elements)
}
