use itertools::Itertools;

use crate::style;

use super::Text;

// Chunks the input text by line number.
// each inner vector's element have the same line number
// and also sorts the inner vector by column
fn chunk_texts(texts: &Vec<Text>) -> Vec<Vec<&Text>> {
    texts
        .iter()
        .chunk_by(|x| x.line_number())
        .into_iter()
        // Simplify the output for chunk_by
        .map(|chunk| {
            chunk
                .1
                .into_iter()
                .sorted_by_key(|x| x.column())
                .collect::<Vec<&Text>>()
        })
        .collect::<Vec<Vec<&Text>>>()
}

// Applies style and proper formatting to each text.
fn apply_style(texts: &Vec<Text>) -> Vec<Text> {
    texts
        .iter()
        .map(|x| {
            let style = style::parse_text_style(x.style().into());
            let string = format!("{}{}\x1b[0m", style, x.text());
            // This will be fine as we are always supplying an empty style
            Text::new_unchecked(&string, x.line_number(), x.column(), &[])
        })
        .collect()
}
// IMPORTANT: texts should all be in one line and texts should be sorted by column
fn column_pad(texts: &Vec<Text>) -> () {
    let relative_columns = texts.windows(2).map(|x| dbg!(x));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn clean_code() {
        let texts = vec![
            Text::new_unchecked("Hi", 1, 0, &[]),
            Text::new_unchecked("Hi", 1, 5, &[]),
            Text::new_unchecked("Hi", 1, 8, &[]),
            Text::new_unchecked("Hi", 1, 10, &[]),
        ];
        dbg!(apply_style(&texts));
    }
}
