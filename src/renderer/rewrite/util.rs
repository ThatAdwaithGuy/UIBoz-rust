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
            // dbg!(i, prev, curr, curr.column,prev.text_len(), prev_end);
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
mod tests {
    use crate::style::TextStyle;

    use super::*;

    // ── helpers ──────────────────────────────────────────────────────────────

    fn t(text: &str, line: u32, col: u32) -> Text {
        Text::new_unchecked(text, line, col, &[])
    }

    fn t_styled(text: &str, line: u32, col: u32, style: &[TextStyle]) -> Text {
        Text::new_unchecked(text, line, col, style)
    }

    // ── chunk_texts ──────────────────────────────────────────────────────────

    #[test]
    fn chunk_texts_empty_input() {
        let texts: Vec<Text> = vec![];
        let result = chunk_texts(&texts);
        assert!(result.is_empty());
    }

    #[test]
    fn chunk_texts_single_item() {
        let texts = vec![t("hello", 0, 0)];
        let result = chunk_texts(&texts);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0].text(), "hello");
    }

    #[test]
    fn chunk_texts_groups_by_line_number() {
        let texts = vec![t("a", 0, 0), t("b", 1, 0), t("c", 0, 5)];
        let result = chunk_texts(&texts);
        assert_eq!(result.len(), 2);
        // line 0 gets both "a" and "c"
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn chunk_texts_inner_sorted_by_column() {
        let texts = vec![t("right", 0, 10), t("left", 0, 0)];
        let result = chunk_texts(&texts);
        assert_eq!(result[0][0].column(), 0);
        assert_eq!(result[0][1].column(), 10);
    }

    #[test]
    fn chunk_texts_lines_sorted_ascending() {
        let texts = vec![t("c", 5, 0), t("a", 0, 0), t("b", 2, 0)];
        let result = chunk_texts(&texts);
        assert_eq!(result[0][0].line_number(), 0);
        assert_eq!(result[1][0].line_number(), 2);
        assert_eq!(result[2][0].line_number(), 5);
    }

    #[test]
    fn chunk_texts_multiple_items_same_line_sorted() {
        let texts = vec![t("│", 3, 53), t("│", 3, 0), t("Hi", 3, 2)];
        let result = chunk_texts(&texts);
        assert_eq!(result.len(), 1);
        let cols: Vec<u32> = result[0].iter().map(|x| x.column()).collect();
        assert_eq!(cols, vec![0, 2, 53]);
    }

    // ── find_padding_size ────────────────────────────────────────────────────

    #[test]
    fn find_padding_size_single_item_unchanged() {
        let texts = vec![t("hello", 0, 5)];
        let refs: Vec<&Text> = texts.iter().collect();
        let result = find_padding_size(&refs);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].column(), 5);
        assert_eq!(result[0].text(), "hello");
    }

    #[test]
    fn find_padding_size_adjacent_items_zero_gap() {
        // "ab" at col 0 (len 2), "cd" at col 2 — gap is 0
        let texts = vec![t("ab", 0, 0), t("cd", 0, 2)];
        let refs: Vec<&Text> = texts.iter().collect();
        let result = find_padding_size(&refs);
        assert_eq!(result[0].column(), 0);
        assert_eq!(result[1].column(), 0); // 2 - (0 + 2) = 0
    }

    #[test]
    fn find_padding_size_gap_between_items() {
        // "ab" at col 0 (len 2), "cd" at col 5 — gap is 3
        let texts = vec![t("ab", 0, 0), t("cd", 0, 5)];
        let refs: Vec<&Text> = texts.iter().collect();
        let result = find_padding_size(&refs);
        assert_eq!(result[1].column(), 3); // 5 - (0 + 2) = 3
    }

    #[test]
    fn find_padding_size_preserves_first_column() {
        let texts = vec![t("x", 0, 7), t("y", 0, 10)];
        let refs: Vec<&Text> = texts.iter().collect();
        let result = find_padding_size(&refs);
        assert_eq!(result[0].column(), 7);
    }

    #[test]
    fn find_padding_size_three_items() {
        // "a" col 0 len 1, "b" col 3 len 1, "c" col 8 len 1
        // gaps: first stays 0, second = 3-(0+1)=2, third = 8-(3+1)=4
        let texts = vec![t("a", 0, 0), t("b", 0, 3), t("c", 0, 8)];
        let refs: Vec<&Text> = texts.iter().collect();
        let result = find_padding_size(&refs);
        assert_eq!(result[0].column(), 0);
        assert_eq!(result[1].column(), 2);
        assert_eq!(result[2].column(), 4);
    }

    #[test]
    #[should_panic]
    fn find_padding_size_unsorted_panics_or_underflows() {
        // col order is wrong: second item is behind the first's end
        let texts = vec![t("hello", 0, 10), t("x", 0, 0)];
        let refs: Vec<&Text> = texts.iter().collect();
        let _ = find_padding_size(&refs); // should panic (subtract-with-overflow in debug)
    }

    // ── apply_padding_size_and_combine ───────────────────────────────────────

    #[test]
    #[should_panic]
    fn apply_padding_size_and_combine_panics_on_empty() {
        let line: Vec<Text> = vec![];
        apply_padding_size_and_combine(&line);
    }

    #[test]
    fn apply_padding_size_and_combine_single_item_no_padding() {
        let line = vec![t("hello", 2, 0)];
        let (s, line_num) = apply_padding_size_and_combine(&line);
        assert_eq!(s, "hello");
        assert_eq!(line_num, 2);
    }

    #[test]
    fn apply_padding_size_and_combine_applies_column_as_spaces() {
        // column 3 → 3 spaces prepended
        let line = vec![t("hi", 0, 3)];
        let (s, _) = apply_padding_size_and_combine(&line);
        assert_eq!(s, "   hi");
    }

    #[test]
    fn apply_padding_size_and_combine_two_items_concatenated() {
        // after find_padding_size: "│" col 0, "│" col 53
        // apply_padding gives "│" + " "*53 + "│"
        let line = vec![t("│", 1, 0), t("│", 1, 53)];
        let (s, line_num) = apply_padding_size_and_combine(&line);
        assert_eq!(line_num, 1);
        assert!(s.starts_with("│"));
        assert!(s.contains(&" ".repeat(53)));
    }

    #[test]
    fn apply_padding_size_and_combine_returns_correct_line_number() {
        let line = vec![t("x", 7, 0)];
        let (_, line_num) = apply_padding_size_and_combine(&line);
        assert_eq!(line_num, 7);
    }

    // ── apply_style ──────────────────────────────────────────────────────────

    #[test]
    fn apply_style_empty_input() {
        let texts: Vec<Text> = vec![];
        let result = apply_style(&texts);
        assert!(result.is_empty());
    }

    #[test]
    fn apply_style_output_ends_with_reset() {
        let texts = vec![t("hello", 0, 0)];
        let result = apply_style(&texts);
        assert!(result[0].text().ends_with("\x1b[0m"));
    }

    #[test]
    fn apply_style_preserves_line_and_column() {
        let texts = vec![t("hi", 3, 7)];
        let result = apply_style(&texts);
        assert_eq!(result[0].line_number(), 3);
        assert_eq!(result[0].column(), 7);
    }

    // ── integration: chunk → find_padding → combine ──────────────────────────

    #[test]
    fn full_pipeline_renders_line_correctly() {
        // Simulates: │ at col 0, Hi at col 2, │ at col 7 — all on line 1
        let texts = vec![t("│", 1, 0), t("Hi", 1, 2), t("│", 1, 7)];
        let chunked = chunk_texts(&texts);
        assert_eq!(chunked.len(), 1);

        let padded = find_padding_size(&chunked[0]);
        // first item unchanged: col 0
        assert_eq!(padded[0].column(), 0);
        // "│" len 1, so "Hi" gap = 2 - (0+1) = 1
        assert_eq!(padded[1].column(), 1);
        // "Hi" len 2, so "│" gap = 7 - (2+2) = 3
        assert_eq!(padded[2].column(), 3);

        let (s, line_num) = apply_padding_size_and_combine(&padded);
        assert_eq!(line_num, 1);
        assert_eq!(s, "│ Hi   │");
    }
}
