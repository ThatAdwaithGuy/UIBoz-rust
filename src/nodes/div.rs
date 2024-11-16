use super::widgets;
use crate::storage::Node;
use crate::renderer::*;
use node_proc_macro::Node;

#[derive(Debug, Node)]
struct Div {
    texts: Vec<Box<dyn widgets::Widget>>,
    type_of_border: TypeOfBorder,
}

macro_rules! div {
    // Empty case
    () => {
        Vec::new()
    };

    // Single value case
    ($elem:expr, $type_of_border:expr) => {
        {
            let mut v: Vec<Box<dyn widgets::Widget>> = Vec::new();
            v.push(Box::new($elem.clone()));
            Div::new(v, $type_of_border)
        }
    };

    // Multiple values case
    ($($elem:expr),+ $(,)?, $type_of_border:expr) => {
        {
            let mut v: Vec<Box<dyn widgets::Widget>> = Vec::new();
            $(
                v.push(Box::new($elem.clone()));
            )+
            Div::new(v, $type_of_border)
        }
    };
}

impl Div {
    fn new(texts: Vec<Box<dyn widgets::Widget>>, type_of_border: TypeOfBorder) -> Div {
        Self {
            texts,
            type_of_border,
        }
    }
}

impl widgets::Widget for Div {
    fn render(&self) -> Vec<Text> {
        let flat = self.texts.iter().map(|x| x.render()).flatten();
        let window = Window::new(
            flat.clone().map(|x| TextType::Text(x.clone())).collect(),
            flat.clone()
                .map(|x| x.column + x.len() as u32)
                .max()
                .unwrap(),
            flat.map(|x| x.line_number).max().unwrap(),
            self.type_of_border,
        );

        window
            .render()
            .expect("Render failed.")
            .split("\n")
            .enumerate()
            .map(|(i, v)| Text::new(v, i as u32, 0, &[]))
            .collect::<Vec<Text>>()
    }
}

#[cfg(test)]
mod tests {
    use super::super::label;
    use super::*;
    use widgets::Widget;

    #[test]
    fn div_test() {
        let label = label::Label::new("Hi", &[]);
        let div = div![label, TypeOfBorder::CurvedBorders];
        dbg!(div.render());
    }
}
