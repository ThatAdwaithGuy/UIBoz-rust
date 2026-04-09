use itertools::Itertools;

use crate::style;

use super::Text;

// Chunks the input text by line number.
// each inner vector's element have the same line number
// and also sorts the inner vector by column
pub fn chunk_texts(texts: &Vec<Text>) -> Vec<Vec<&Text>> {
    texts
        .iter()
        .sorted_by_key(|x| x.line_number())
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
pub fn apply_style(texts: &Vec<Text>) -> Vec<Text> {
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
// The given texts should all be in one line, this should be used after find_padding_size
pub fn apply_padding_size_and_combine(line: &Vec<Text>) -> (String, usize) {
    if line.len() == 0 {
        panic!("Line is empty! {:#?}", line);
    }
    let applied_padding = line.iter().map(|text| {
        Text::new_unchecked(
            &format!("{}{}", " ".repeat(text.column as usize), text.text),
            text.line_number,
            0,
            &[],
        )
    });

    let join = applied_padding.map(|text| text.text).join("");
    (join, line[0].line_number as usize)
}

// IMPORTANT: texts should all be in one line and texts should be sorted by column
// or else this will panic
// pub fn find_padding_size(texts: &Vec<&Text>) -> Vec<Text> {
//     if texts.len() == 1 {
//         return texts.iter().map(|x| (*x).clone()).collect();
//     }
//     texts
//         .windows(2)
//         .map(|x| {
//             let first = x[0].clone();
//             let second = &x[1];
//             // dbg!(&first, first.len(), first.text_len());
//             let first_length = first.text_len() as u32 + first.column();
//             // dbg!(&first, second, first_length,);
//             let second = Text::new_unchecked(
//                 second.text(),
//                 second.line_number(),
//                 second.column() - first_length,
//                 &second.style(),
//             );
//             [first, second]
//         })
//         .flatten()
//         .collect_vec()
// }

pub fn find_padding_size(texts: &Vec<&Text>) -> Vec<Text> {
    if texts.len() == 1 {
        return texts.iter().map(|x| (*x).clone()).collect();
    }

    let mut result: Vec<Text> = Vec::with_capacity(texts.len());

    for i in 0..texts.len() {
        if i == 0 {
            result.push((*texts[i]).clone());
        } else {
            let prev = &texts[i - 1];
            let curr = texts[i];
            let prev_end = prev.text_len() as u32 + prev.column();
            dbg!(i, prev, curr, curr.column,prev.text_len(), prev_end);
            let adjusted = Text::new_unchecked(
                curr.text(),
                curr.line_number(),
                curr.column() - prev_end,
                &curr.style(),
            );
            result.push(adjusted);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn clean_code() {
        // Hi   Hi  Hi  Hi
        let texts = vec![
            Text::new_unchecked("Hi", 1, 0, &[]),
            Text::new_unchecked("Hi", 1, 5, &[]),
            Text::new_unchecked("Hi", 1, 8, &[]),
            Text::new_unchecked("Hi", 1, 10, &[]),
        ];
        // dbg!(find_padding_size(&texts));
    }
}
