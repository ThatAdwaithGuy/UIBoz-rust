use super::boz::{Boz, TypeOfBorder};
use crate::{draw_boz::boz, errors::TextError};
use core::panic;
use std::rc::Rc;
// use crate::draw_boz::opts;

type Texts = Vec<TextType>;

#[derive(Clone)]
struct NestedBoz {
    texts: Texts,
    height: i32,
    width: i32,
    type_of_border: boz::TypeOfBorder,
}
impl NestedBoz {
    fn new(texts: Texts, height: i32, width: i32, type_of_border: TypeOfBorder) -> Self {
        Self {
            texts,
            height,
            width,
            type_of_border,
        }
    }
}

#[derive(Clone)]
struct SubBoz {
    boz: NestedBoz,
    start_line_number: i32,
    column: i32,
}
impl SubBoz {
    fn new(boz: NestedBoz, start_line_number: i32, column: i32) -> Self {
        Self {
            boz,
            start_line_number,
            column,
        }
    }
}

#[derive(Clone)]
enum TextType {
    SubBoz(SubBoz),
    Text(boz::Text),
}

fn has_nested_boz(boz: NestedBoz) -> bool {
    boz.texts.iter().any(|tt| match tt {
        TextType::SubBoz(_) => true,
        TextType::Text(_) => false,
    })
}

fn render_lines(
    boz: boz::Boz,
    start_line_number: i32,
    column: i32,
) -> Result<Vec<boz::Text>, TextError> {
    let rendered_boz = boz.render_string()?;
    let splited: Vec<&str> = rendered_boz.split("\n").into_iter().collect();
    let mut res: Vec<boz::Text> = splited
        .into_iter()
        .enumerate()
        .map(|(i, v)| boz::Text::new(v, i as i32 + start_line_number, column, Rc::new([])))
        .collect();
    Ok(res)
}

// NestedBoz to Boz
fn nb_to_b(boz: NestedBoz) -> Option<boz::Boz> {
    let mut texts = vec![];
    for tt in boz.texts {
        match tt {
            TextType::Text(text) => texts.push(text),
            TextType::SubBoz(_) => {
                return None;
            }
        }
    }
    Some(Boz::new(texts, boz.height, boz.width, boz.type_of_border))
}

fn collapse_nested_boz(nested_boz: SubBoz) -> Result<Vec<boz::Text>, TextError> {
    let mut res: Vec<boz::Text> = vec![];
    for text_type in nested_boz.boz.texts {
        match text_type {
            TextType::Text(text) => res.push(text),
            TextType::SubBoz(sub_boz) => {
                let mut res1: Vec<boz::Text> = vec![];
                for text_type1 in sub_boz.boz.texts {
                    match text_type1 {
                        TextType::Text(text1) => res1.push(text1),
                        TextType::SubBoz(sub_boz) => {
                            panic!("SHIT")
                        }
                    }
                }
                res.extend(render_lines(
                    boz::Boz::new(
                        res1,
                        sub_boz.boz.height,
                        sub_boz.boz.width,
                        sub_boz.boz.type_of_border,
                    ),
                    sub_boz.start_line_number,
                    sub_boz.column,
                )?)
            }
        }
    }
    //Ok(render_lines(
    //    boz::Boz::new(
    //        res,
    //        nested_boz.boz.height,
    //        nested_boz.boz.width,
    //        nested_boz.boz.type_of_border,
    //    ),
    //    nested_boz.start_line_number,
    //    nested_boz.column,
    //)?)
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() -> Result<(), TextError> {
        ////////////////////////////////////////
        // Adwaith, no_of_ansi_codes. Got it? //
        ////////////////////////////////////////
        let mut texts: Vec<TextType> = vec![];
        texts.push(TextType::Text(boz::Text::new("Hello", 1, 4, Rc::new([]))));
        texts.push(TextType::Text(boz::Text::new("world", 1, 11, Rc::new([]))));
        texts.push(TextType::Text(boz::Text::new("Love to", 2, 4, Rc::new([]))));
        texts.push(TextType::Text(boz::Text::new(
            "Catpuccin",
            2,
            11,
            Rc::new([]),
        )));
        let boz = NestedBoz::new(
            vec![TextType::Text(boz::Text::new("hello", 5, 5, Rc::new([])))],
            5,
            10,
            TypeOfBorder::CurvedBorders,
        );
        texts.push(TextType::SubBoz(SubBoz::new(boz, 3, 1)));
        let nested_boz = SubBoz::new(
            NestedBoz::new(texts, 5, 40, TypeOfBorder::CurvedBorders),
            1,
            1,
        );
        let collapsed = collapse_nested_boz(nested_boz.clone())?;
        dbg!(collapsed.clone());
        let a = render_lines(
            boz::Boz::new(
                collapsed.clone(),
                nested_boz.clone().boz.height,
                nested_boz.clone().boz.width,
                nested_boz.clone().boz.type_of_border,
            ),
            nested_boz.clone().start_line_number,
            nested_boz.clone().column,
        )?;
        let rboz = boz::Boz::new(collapsed, 20, 100, TypeOfBorder::CurvedBorders);
        let b = rboz.render_string()?;
        println!("{}", b);
        dbg!(a);
        Ok(())
    }
}
