use itertools::Itertools;

use crate::again;
use crate::errors::TextError;
use std::collections::HashMap;
use std::hash::Hash;
type Texts = Vec<TextType>;
/*
macro_rules! constructor_step {
    ($a:ident,$b:ty) => {
        pub fn $a(&mut self, pl: $b) -> &mut Self {
            self.$a = pl;
            self
        }
    };
}
#[derive(Clone)]
struct PrivateText {
    pub text: String,
    pub line_number: u32,
    pub column: u32,
    pub style: &'static [style::TextStyle],
    pub no_of_ansi: u32,
    // (Is nested, level of recursion)
    pub is_nested: (bool, u32),
}

impl Into<PrivateText> for again::Text {
    fn into(self) -> PrivateText {
        PrivateText::new()
            .text(&self.text)
            .line_number(self.line_number)
            .column(self.column)
            .style(self.style)
            .no_of_ansi(self.no_of_ansi)
            .to_owned()
    }
}

impl PrivateText {
    pub fn new() -> Self {
        Self {
            text: "".to_string(),
            line_number: 0,
            column: 0,
            style: &[],
            no_of_ansi: 1,
            is_nested: (false, 0),
        }
    }

    pub fn text(&mut self, pl: &str) -> &mut Self {
        self.text = pl.to_string();
        self
    }

    //  I saved 15 lines of code by spending 15 minutes...
    constructor_step!(line_number, u32);
    constructor_step!(column, u32);
    constructor_step!(style, &'static [style::TextStyle]);
    constructor_step!(no_of_ansi, u32);
    constructor_step!(is_nested, (bool, u32));
}
*/

#[derive(Clone, Debug)]
struct NestedWindow {
    texts: Texts,
    height: u32,
    width: u32,
    type_of_border: again::TypeOfBorder,
}

impl NestedWindow {
    fn new(texts: Texts, height: u32, width: u32, type_of_border: again::TypeOfBorder) -> Self {
        Self {
            texts,
            height,
            width,
            type_of_border,
        }
    }
}

#[derive(Clone, Debug)]
struct SubWindow {
    window: NestedWindow,
    start_line_number: u32,
    column: u32,
    // {Line_number: No_Of_Ansi_Codes}
    ansi_codes_map: HashMap<u32, u32>,
}
impl SubWindow {
    fn new(window: NestedWindow, start_line_number: u32, column: u32) -> Self {
        Self {
            window,
            start_line_number,
            column,
            ansi_codes_map: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
enum TextType {
    SubWindow(SubWindow),
    Text(again::Text),
}

fn sort_hashmap_by_key<K, V>(map: &HashMap<K, V>) -> Vec<(K, V)>
where
    K: Ord + Clone + Hash,
    V: Clone,
{
    let mut vec: Vec<(K, V)> = map.iter().map(|(i, v)| (i.clone(), v.clone())).collect();
    vec.sort_by(|a, b| a.0.cmp(&b.0));
    vec
}
// Absolute Hell
fn word_indices(input: &str) -> Vec<(usize, String)> {
    let mut result: Vec<(usize, String)> = Vec::new();
    let string = String::from(input)
        .chars()
        .enumerate()
        .collect::<HashMap<usize, char>>();

    let splited: HashMap<usize, char> = sort_hashmap_by_key(&string)
        .iter()
        .filter(|(_, x)| *x != ' ')
        .map(|(i, v)| (*i, *v))
        .collect();
    //dbg!(input, sort_hashmap_by_key(&splited));
    let mut word = String::new();
    let mut word_start_index = 0usize;
    let mut in_word = false;
    for (i, c) in sort_hashmap_by_key(&splited) {
        match c {
            '│' => {
                if in_word {
                    result.push((word_start_index, word.clone()));
                    word.clear();
                    in_word = false;
                    word_start_index = 0;
                }
                result.push((i, c.to_string()));
            }
            _ => {
                in_word = true;
                if word_start_index == 0 {
                    word_start_index = i;
                }
                word.push(c);
            }
        }
    }
    //dbg!(result.clone());
    if !word.is_empty() && result.is_empty() {
        result.push((word_start_index, word.clone()))
    }

    result
}

fn partition_line(text: again::Text) -> Vec<again::Text> {
    let words = word_indices(&text.text);

    words
        .iter()
        .map(|(i, v)| {
            again::Text::new(v, text.line_number, *i as u32 + text.column, &[])
                .no_of_ansi(words.len() as u32)
        })
        .collect()
}
/*
fn find_no_of_ansi(nested: SubWindow) -> u32 {
    let mut total = 0;
    let mut queue: VecDeque<TextType> = VecDeque::new();
    queue.push_back(TextType::SubWindow(nested));
    while let Some(val) = queue.pop_back() {
        match val {
            TextType::Text(_) => total += 1,
            TextType::SubWindow(window) => {
                total += 2;
                for child in window.window.texts.iter().rev() {
                    queue.push_back(child.clone());
                }
            }
        }
    }
    total - 2
}
*/

fn update_map(win: SubWindow) -> Option<SubWindow> {
    let mut texts: Vec<&again::Text> = vec![];
    for text_type in win.window.texts.iter() {
        match text_type {
            TextType::Text(text) => texts.push(&text),
            TextType::SubWindow(_) => return None,
        }
    }

    let chunked: HashMap<u32, u32> = texts
        .iter()
        .chunk_by(|x| x.line_number)
        .into_iter()
        .map(|x| (x.0, x.1.collect_vec().len() as u32))
        .collect();

    let mut window = win;
    window.ansi_codes_map = chunked;

    Some(window)
}

fn update_window_map(win: again::Window) -> HashMap<u32, u32> {
    win.texts
        .iter()
        .chunk_by(|x| x.line_number)
        .into_iter()
        .map(|x| (x.0, x.1.collect_vec().len() as u32))
        .collect()
}
fn add_maps(map1: HashMap<u32, u32>, map2: HashMap<u32, u32>) -> HashMap<u32, u32> {
    let mut result = map1;

    for (&key, &value) in map2.iter() {
        result
            .entry(key)
            .and_modify(|e| *e += value)
            .or_insert(value);
    }

    result
}

fn add_texts_maps(text: Vec<again::Text>, map: &HashMap<u32, u32>) -> Vec<again::Text> {
    let mut ret = vec![];
    let mut unseen: Vec<u32> = ((text)[0].line_number..=text[text.len() - 1].line_number).collect();

    for (k, v) in map.iter() {
        if let Some(val) = text.iter().find(|x| x.line_number == *k) {
            let mut vaal = val.clone();
            vaal.no_of_ansi = *v;
            ret.push(vaal.to_owned());
            unseen.retain(|x| *x != val.line_number);
        }
    }
    for idx in unseen {
        // TODO: This will not work if the text is not sorted by line number. fix it.
        ret.push(text[idx as usize - 1].clone());
    }
    ret.sort_by_key(|x| x.line_number);
    ret
}

fn is_nested(text: &Vec<TextType>) -> bool {
    text.iter().any(|x| match x {
        TextType::Text(_) => false,
        TextType::SubWindow(_) => true,
    })
}

fn collapse_one_deep_subwindow(
    win: SubWindow,
) -> Result<(Vec<again::Text>, HashMap<u32, u32>), TextError> {
    let mut texts: Vec<again::Text> = Vec::new();

    if win.window.texts.iter().any(|x| match x {
        TextType::Text(text) => {
            texts.push(text.clone());
            false
        }
        TextType::SubWindow(_) => true,
    }) {
        return Err(TextError::DuplicateText(
            "if you see this. I made a mistake here".to_string(),
        ));
    }
    let new_texts = add_texts_maps(texts.clone(), &win.ansi_codes_map);

    //dbg!(new_texts);
    let window = again::Window::new(
        texts.clone(),
        win.window.height,
        win.window.width,
        win.window.type_of_border,
    );

    let binding = window.render(false)?;
    let winding = binding.clone();
    let lines: Vec<&str> = winding.split("\n").collect_vec();
    let mut ret = vec![];
    for i in 0..lines.len() - 1 {
        ret.push(again::Text::new(lines[i], i as u32 + 1u32, 0, &[]));
    }

    Ok((ret.clone(), update_window_map(window)))
}

fn collapse_subwindow(win: SubWindow) -> Result<Vec<again::Text>, TextError> {
    let mut res1: Vec<again::Text> = vec![];
    let mut hashmap: HashMap<u32, u32> = HashMap::new();

    for text_type in win.window.texts {
        match text_type {
            TextType::Text(t) => res1.push(t),
            TextType::SubWindow(window1) => {
                if !is_nested(&window1.window.texts) {
                    let iter = collapse_one_deep_subwindow(window1)?;
                    dbg!(&iter.1);
                    hashmap = add_maps(
                        hashmap,
                        iter.1
                            .iter()
                            .map(|(k, v)| (k + 1, *v + 1))
                            .collect::<HashMap<u32, u32>>(),
                    );
                    res1.extend(iter.0);
                } else {
                    return Err(TextError::LeftBounds("asdfasdf".to_string()));
                }
            }
        }
    }
    res1 = add_texts_maps(res1, &hashmap);

    dbg!(&res1);
    Ok(res1)
}

#[cfg(test)]
mod tests {
    use crate::errors::TextError;

    use super::*;

    #[test]
    fn some() -> Result<(), TextError> {
        let bob = vec![
            again::Text::new("@", 1, 0, &[]),
            again::Text::new("@", 2, 0, &[]),
            again::Text::new("@", 2, 0, &[]),
        ];
        let children = TextType::Text(again::Text::new("@", 1, 0, &[]));
        let children1 = TextType::Text(again::Text::new("@", 2, 0, &[]));
        let children2 = TextType::Text(again::Text::new("@", 2, 2, &[]));

        let texts = vec![children.clone(), children1.clone(), children2.clone()];
        let child1 = SubWindow::new(
            NestedWindow::new(texts, 10, 10, again::TypeOfBorder::CurvedBorders),
            1,
            1,
        );

        let child2 = SubWindow::new(
            NestedWindow::new(
                vec![children.clone(), children.clone()],
                10,
                10,
                again::TypeOfBorder::NoBorders,
            ),
            1,
            1,
        );

        let root = SubWindow::new(
            NestedWindow::new(
                vec![
                    TextType::SubWindow(child1.clone()),
                    //TextType::SubWindow(child2.clone()),
                    //TextType::SubWindow(child2.clone()),
                ],
                10,
                10,
                again::TypeOfBorder::CurvedBorders,
            ),
            1,
            1,
        );

        let t = vec![
            again::Text::new("!", 1, 0, &[]),
            again::Text::new("@", 2, 0, &[]),
            again::Text::new("#", 2, 1, &[]),
        ];

        let windows = again::Window::new(t, 10, 80, again::TypeOfBorder::CurvedBorders);
        //println!("{}", windows.render()?);

        let a = collapse_subwindow(root)?;
        dbg!(&a);
        let b = again::Window::new(a.clone(), 20, 100, again::TypeOfBorder::CurvedBorders);
        println!("{}", b.render(false)?);

        Ok(())
    }
}
