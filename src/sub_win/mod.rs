use itertools::Itertools;
pub mod new_mod;
use crate::errors::TextError;
use crate::raw_window;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;
use std::vec;
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

impl Into<PrivateText> for window::Text {
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
pub struct NestedWindow {
    texts: Texts,
    height: u32,
    width: u32,
    type_of_border: raw_window::TypeOfBorder,
}

impl NestedWindow {
    pub fn new(
        texts: Texts,
        height: u32,
        width: u32,
        type_of_border: raw_window::TypeOfBorder,
    ) -> Self {
        Self {
            texts,
            height,
            width,
            type_of_border,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubWindow {
    window: NestedWindow,
    start_line_number: u32,
    column: u32,
    // {Line_number: No_Of_Ansi_Codes}
    ansi_codes_map: HashMap<u32, u32>,
}
impl SubWindow {
    pub fn new(window: NestedWindow, start_line_number: u32, column: u32) -> Self {
        Self {
            window,
            start_line_number,
            column,
            ansi_codes_map: HashMap::new(),
        }
    }
    fn render(&self) -> Result<String, TextError> {
        let texts = collapse_subwindow(self.clone())?;
        let window = raw_window::NonNestableWindow::new(
            texts,
            self.window.height,
            self.window.width,
            self.window.type_of_border,
        );
        Ok(window.render(false)?)
    }
}

#[derive(Clone, Debug)]
pub enum TextType {
    SubWindow(SubWindow),
    Text(raw_window::Text),
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

fn partition_line(text: raw_window::Text) -> Vec<raw_window::Text> {
    let words = word_indices(&text.text);

    words
        .iter()
        .map(|(i, v)| {
            raw_window::Text::new(v, text.line_number, *i as u32 + text.column, &[])
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
    let mut texts: Vec<&raw_window::Text> = vec![];
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

fn update_window_map(win: raw_window::NonNestableWindow) -> HashMap<u32, u32> {
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

fn add_texts_maps(text: Vec<raw_window::Text>, map: &HashMap<u32, u32>) -> Vec<raw_window::Text> {
    if text == vec![] {
        return vec![];
    }
    let mut text_clone = text.clone();
    let mut ret = vec![];
    for t in text.iter() {
        for (line_number, ansi_codes) in map.iter() {
            if t.line_number == *line_number {
                ret.push(t.clone().no_of_ansi(*ansi_codes));
                text_clone = text_clone
                    .iter()
                    .filter(|x| *x != t)
                    .map(|x| x.clone())
                    .collect();
            }
        }
    }
    ret.extend(text_clone);
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
) -> Result<(Vec<raw_window::Text>, HashMap<u32, u32>), TextError> {
    let mut texts: Vec<raw_window::Text> = Vec::new();
    // depth check
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
    let window = raw_window::NonNestableWindow::new(
        new_texts.clone(),
        win.window.height,
        win.window.width,
        win.window.type_of_border,
    );

    let binding = window.render(false)?;
    let winding = binding.clone();
    let lines: Vec<&str> = winding.split("\n").collect_vec();
    let mut ret = vec![];
    for i in 0..lines.len() - 1 {
        if i == 0 || i == lines.len() - 1 {
            ret.push(raw_window::Text::new(lines[i], (i + 1) as u32, 0, &[]));
        } else if lines[i].contains("\x1b") {
            dbg!("BROOOO", lines[i]);
            ret.push(
                raw_window::Text::new(lines[i], (i + 1) as u32, 0, &[])
                    .no_of_ansi((lines[i].matches("\x1b").count() / 8) as u32),
            );
        }
    }
    dbg!(&ret, &win);

    Ok((ret.clone(), update_window_map(window)))
}

pub fn collapse_subwindow(win: SubWindow) -> Result<Vec<raw_window::Text>, TextError> {
    let mut res1: Vec<raw_window::Text> = vec![];
    let mut hashmap: HashMap<u32, u32> = HashMap::new();

    for text_type in win.window.texts {
        match text_type {
            TextType::Text(t) => {
                dbg!(&t);
                res1.push(t)
            }
            TextType::SubWindow(window1) => {
                if !is_nested(&window1.window.texts) {
                    dbg!(&window1.window.texts);
                    let iter = collapse_one_deep_subwindow(window1)?;
                    dbg!(&iter.0, &hashmap);
                    hashmap = add_maps(
                        hashmap,
                        iter.1
                            .iter()
                            .map(|(k, v)| (*k, *v + 1))
                            .collect::<HashMap<u32, u32>>(),
                    );
                    dbg!(&iter);
                    dbg!(&hashmap);
                    res1.extend(iter.0);
                } else {
                    return Err(TextError::LeftBounds("asdfasdf".to_string()));
                }
            }
        }
    }
    dbg!(&res1);
    res1 = add_texts_maps(res1, &hashmap);

    dbg!(&res1);
    Ok(res1)
}

#[cfg(test)]
mod tests {
    use core::hash;

    use crate::errors::TextError;

    use super::*;
    #[test]
    fn some() -> Result<(), TextError> {
        let bob = vec![
            raw_window::Text::new("@", 1, 0, &[]),
            raw_window::Text::new("@", 2, 0, &[]),
            raw_window::Text::new("@", 2, 0, &[]),
        ];
        let children = TextType::Text(raw_window::Text::new("", 1, 0, &[]));
        let children1 = TextType::Text(raw_window::Text::new("@", 2, 0, &[]));
        let children2 = TextType::Text(raw_window::Text::new("@", 2, 2, &[]));

        let texts = vec![children.clone()];
        let child1 = SubWindow::new(
            NestedWindow::new(texts, 10, 10, raw_window::TypeOfBorder::CurvedBorders),
            1,
            1,
        );

        let child2 = SubWindow::new(
            NestedWindow::new(
                vec![children.clone(), children.clone()],
                5,
                10,
                raw_window::TypeOfBorder::CurvedBorders,
            ),
            4,
            1,
        );

        let root = SubWindow::new(
            NestedWindow::new(
                vec![
                    //TextType::SubWindow(child1.clone()),
                    //TextType::SubWindow(child2.clone()),
                    //TextType::SubWindow(child2.clone()),
                ],
                20,
                10,
                raw_window::TypeOfBorder::CurvedBorders,
            ),
            1,
            1,
        );

        let t = vec![
            raw_window::Text::new("!", 1, 0, &[]),
            raw_window::Text::new("@", 2, 0, &[]),
            raw_window::Text::new("#", 2, 1, &[]),
        ];
        let mut hashmap: HashMap<u32, u32> = HashMap::new();
        hashmap.insert(2, 2);
        dbg!(&hashmap);
        dbg!(&t);
        dbg!(add_texts_maps(t, &hashmap));

        //let windows = window::Window::new(t, 10, 80, again::TypeOfBorder::CurvedBorders);

        let a = collapse_subwindow(root)?;
        let b = raw_window::NonNestableWindow::new(
            a.clone(),
            20,
            100,
            raw_window::TypeOfBorder::CurvedBorders,
        );
        println!("{}", b.render(false)?);

        Ok(())
    }
}
