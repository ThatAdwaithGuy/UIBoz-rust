use crate::renderer::rewrite::Text;

impl Text {
    pub fn left(&self) -> Self {
        Text {
            text: self.text.clone(),
            line_number: self.line_number,
            column: 0,
            style: self.style,
            no_of_ansi: 1,
        }
    }
    pub fn center(&self, width: u32) -> Self {
        let padding = (width - self.text.len() as u32) / 2;
        Text {
            text: self.text.clone(),
            line_number: self.line_number,
            column: padding,
            style: self.style,
            no_of_ansi: 1,
        }
    }
    pub fn right(&self, width: u32) -> Self {
        let padding = width - self.text.len() as u32;
        Text {
            text: self.text.clone(),
            line_number: self.line_number,
            column: padding,
            style: self.style,
            no_of_ansi: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn left_test() {
        let text: Text = Text::new("@", 1, 0, &[]).left();
        assert_eq!(text.column, 0);
    }
    #[test]
    fn center_test() {
        let text: Text = Text::new("@", 1, 0, &[]).center(10);
        dbg!(text);
    }
    #[test]
    fn right_test() {
        let text: Text = Text::new("@", 1, 0, &[]).right(10);
        assert_eq!(text.column, 9);
    }
}
