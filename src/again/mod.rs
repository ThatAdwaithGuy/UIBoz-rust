use crate::style;
mod utils;

#[derive(Clone, Debug)]
struct Text {
    pub text: String,
    pub line_number: u32,
    pub column: u32,
    pub style: &'static [style::TextStyle],
}

#[derive(Clone, Debug)]
struct Window {
    pub texts: [Text; 1024],
    pub height: u32,
    pub width: u32,
    pub type_of_border: TypeOfBorder,
}

#[derive(Clone, Debug)]
pub enum TypeOfBorder {
    NoBorders,
    CurvedBorders,
    SquareBorders,
}

impl Text {
    pub fn new(
        text: &str,
        line_number: u32,
        column: u32,
        style: &'static [style::TextStyle],
    ) -> Text {
        Text {
            text: text.to_string(),
            line_number,
            column,
            style,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let texts = vec![Text::new("Hello", 0, 0, &[]), Text::new("World", 0, 6, &[])];
        dbg!(utils::handle(texts));
    }
}
