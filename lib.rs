use std::collections::HashMap;

use node_proc_macro::Node;
use renderer::window_renderer::*;
use storage::Node;

enum Operators {
    Add(f32, f32),
    Subtraction(f32, f32),
    Multiplication(f32, f32),
    Division(f32, f32),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum KeyPads {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
}

#[derive(Debug, Node)]
struct Selected(KeyPads);

#[derive(Node)]
struct Output(String);

struct History {
    lst: Vec<Operators>,
}
// Option: Only to use for division by 0
fn eval_operator(op: Operators) -> Option<f32> {
    match op {
        Operators::Add(lhs, rhs) => Some(lhs + rhs),
        Operators::Subtraction(lhs, rhs) => Some(lhs - rhs),
        Operators::Multiplication(lhs, rhs) => Some(lhs * rhs),
        Operators::Division(lhs, rhs) => {
            if rhs == 0.0 {
                None
            } else {
                Some(lhs / rhs)
            }
        }
    }
}

use renderer::sub_win::TextType;

fn render_text(node_storage: storage::Storage<storage::Immutable>) -> Vec<TextType> {
    let selected = node_storage.get_mut::<Selected>().unwrap();
    let mut texts = vec![];
    let full_line = "-".repeat(23);
    texts.push(renderer::sub_win::TextType::Text(Text::new(
        &full_line,
        5,
        0,
        [style::TextStyle::Bold(false); 9],
    )));

    // Operator veritical dividers
    for i in 1..22 {
        texts.push(renderer::sub_win::TextType::Text(Text::new(
            "|",
            5 + i,
            17,
            [style::TextStyle::Bold(false); 9],
        )));
    }

    // Operator horizontal dividers
    texts.push(renderer::sub_win::TextType::Text(Text::new(
        "-----",
        9,
        18,
        [style::TextStyle::Bold(false); 9],
    )));
    texts.push(renderer::sub_win::TextType::Text(Text::new(
        "-----",
        13,
        18,
        [style::TextStyle::Bold(false); 9],
    )));
    texts.push(renderer::sub_win::TextType::Text(Text::new(
        "-----",
        17,
        18,
        [style::TextStyle::Bold(false); 9],
    )));
    texts.push(renderer::sub_win::TextType::Text(Text::new(
        "-----",
        21,
        18,
        [style::TextStyle::Bold(false); 9],
    )));

    // Number pad horizontal dividers
    texts.push(renderer::sub_win::TextType::Text(Text::new(
        "----- ----- -----",
        12,
        0,
        [style::TextStyle::Bold(false); 9],
    )));

    texts.push(renderer::sub_win::TextType::Text(Text::new(
        "----- ----- -----",
        19,
        0,
        [style::TextStyle::Bold(false); 9],
    )));

    texts.push(renderer::sub_win::TextType::Text(Text::new(
        "-----",
        23,
        6,
        [style::TextStyle::Bold(false); 9],
    )));

    // Number pad veritical dividers
    for i in 1..7 {
        texts.push(renderer::sub_win::TextType::Text(Text::new(
            "|",
            5 + i,
            5,
            [style::TextStyle::Bold(false); 9],
        )));

        texts.push(renderer::sub_win::TextType::Text(Text::new(
            "|",
            5 + i,
            11,
            [style::TextStyle::Bold(false); 9],
        )));
    }

    for i in 8..14 {
        texts.push(renderer::sub_win::TextType::Text(Text::new(
            "|",
            5 + i,
            5,
            [style::TextStyle::Bold(false); 9],
        )));

        texts.push(renderer::sub_win::TextType::Text(Text::new(
            "|",
            5 + i,
            11,
            [style::TextStyle::Bold(false); 9],
        )));
    }

    for i in 15..27 {
        texts.push(renderer::sub_win::TextType::Text(Text::new(
            "|",
            5 + i,
            5,
            [style::TextStyle::Bold(false); 9],
        )));

        texts.push(renderer::sub_win::TextType::Text(Text::new(
            "|",
            5 + i,
            11,
            [style::TextStyle::Bold(false); 9],
        )));
    }

    // Icons, *BOILERPLATE*
    let mut is_selected: [bool; 15] = [false; 15];

    match selected.0 {
        KeyPads::Zero => is_selected[0] = true,
        KeyPads::One => is_selected[1] = true,
        KeyPads::Two => is_selected[2] = true,
        KeyPads::Three => is_selected[3] = true,
        KeyPads::Four => is_selected[4] = true,
        KeyPads::Five => is_selected[5] = true,
        KeyPads::Six => is_selected[6] = true,
        KeyPads::Seven => is_selected[7] = true,
        KeyPads::Eight => is_selected[8] = true,
        KeyPads::Nine => is_selected[9] = true,
        KeyPads::Plus => is_selected[10] = true,
        KeyPads::Minus => is_selected[11] = true,
        KeyPads::Multiply => is_selected[12] = true,
        KeyPads::Divide => is_selected[13] = true,
        KeyPads::Equal => is_selected[14] = true,
    }

    macro_rules! texts_boilerplate {
        ($num:expr, $string:expr,$line_number:expr, $column:expr) => {
            if is_selected[$num] {
                texts.push(TextType::Text(Text::new(
                    $string,
                    $line_number,
                    $column,
                    [style::TextStyle::Reverse(true); 9],
                )));
            } else {
                texts.push(TextType::Text(Text::new(
                    $string,
                    $line_number,
                    $column,
                    [style::TextStyle::Reverse(false); 9],
                )));
            }
        };
    }

    texts_boilerplate!(0, "0", 25, 8);

    texts_boilerplate!(1, "1", 8, 2);
    texts_boilerplate!(2, "2", 8, 8);
    texts_boilerplate!(3, "3", 8, 14);

    texts_boilerplate!(4, "4", 15, 2);
    texts_boilerplate!(5, "5", 15, 8);
    texts_boilerplate!(6, "6", 15, 14);

    texts_boilerplate!(7, "7", 22, 2);
    texts_boilerplate!(8, "8", 21, 8);
    texts_boilerplate!(9, "9", 22, 14);

    texts_boilerplate!(10, "+", 7, 20);
    texts_boilerplate!(11, "-", 11, 20);
    texts_boilerplate!(12, "*", 15, 20);
    texts_boilerplate!(13, "/", 19, 20);
    texts_boilerplate!(14, "=", 23, 20);

    let out = node_storage.get_mut::<Output>().unwrap();
    texts.push(TextType::Text(Text::new(
        &out.0,
        3,
        11,
        [style::TextStyle::Bold(true); 9],
    )));

    texts
}

struct Direction {
    up: Option<KeyPads>,
    down: Option<KeyPads>,
    right: Option<KeyPads>,
    left: Option<KeyPads>,
}

fn create_new_node() -> GridNode {
    let mut grid: HashMap<KeyPads, Direction> = HashMap::new();

    grid.insert(
        KeyPads::One,
        Direction {
            up: None,
            left: None,
            down: Some(KeyPads::Four),
            right: Some(KeyPads::Two),
        },
    );

    grid.insert(
        KeyPads::Two,
        Direction {
            up: None,
            left: Some(KeyPads::One),
            down: Some(KeyPads::Five),
            right: Some(KeyPads::Three),
        },
    );

    grid.insert(
        KeyPads::Three,
        Direction {
            up: None,
            left: Some(KeyPads::Two),
            down: Some(KeyPads::Six),
            right: Some(KeyPads::Plus),
        },
    );

    grid.insert(
        KeyPads::Plus,
        Direction {
            up: None,
            left: Some(KeyPads::Three),
            down: Some(KeyPads::Minus),
            right: None,
        },
    );

    grid.insert(
        KeyPads::Four,
        Direction {
            up: Some(KeyPads::One),
            left: None,
            down: Some(KeyPads::Seven),
            right: Some(KeyPads::Five),
        },
    );

    grid.insert(
        KeyPads::Five,
        Direction {
            up: Some(KeyPads::Two),
            left: Some(KeyPads::Four),
            down: Some(KeyPads::Eight),
            right: Some(KeyPads::Six),
        },
    );

    grid.insert(
        KeyPads::Six,
        Direction {
            up: Some(KeyPads::Three),
            left: Some(KeyPads::Five),
            down: Some(KeyPads::Nine),
            right: Some(KeyPads::Multiply),
        },
    );

    grid.insert(
        KeyPads::Minus,
        Direction {
            up: Some(KeyPads::Plus),
            left: Some(KeyPads::Three),
            down: Some(KeyPads::Multiply),
            right: None,
        },
    );

    grid.insert(
        KeyPads::Multiply,
        Direction {
            up: Some(KeyPads::Minus),
            left: Some(KeyPads::Six),
            down: Some(KeyPads::Divide),
            right: None,
        },
    );

    grid.insert(
        KeyPads::Divide,
        Direction {
            up: Some(KeyPads::Multiply),
            left: Some(KeyPads::Six),
            down: Some(KeyPads::Equal),
            right: None,
        },
    );

    grid.insert(
        KeyPads::Equal,
        Direction {
            up: Some(KeyPads::Divide),
            left: Some(KeyPads::Nine),
            down: None,
            right: None,
        },
    );

    grid.insert(
        KeyPads::Seven,
        Direction {
            up: Some(KeyPads::Four),
            left: None,
            down: None,
            right: Some(KeyPads::Eight),
        },
    );

    grid.insert(
        KeyPads::Eight,
        Direction {
            up: Some(KeyPads::Five),
            left: Some(KeyPads::Seven),
            down: Some(KeyPads::Zero),
            right: Some(KeyPads::Nine),
        },
    );

    grid.insert(
        KeyPads::Nine,
        Direction {
            up: Some(KeyPads::Six),
            left: Some(KeyPads::Eight),
            down: None,
            right: Some(KeyPads::Equal),
        },
    );

    grid.insert(
        KeyPads::Zero,
        Direction {
            up: Some(KeyPads::Eight),
            left: Some(KeyPads::Seven),
            down: None,
            right: Some(KeyPads::Nine),
        },
    );

    GridNode { grid }
}

struct GridNode {
    grid: HashMap<KeyPads, Direction>,
}

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn getch() -> std::io::Result<event::KeyEvent> {
    enable_raw_mode()?;
    if event::poll(std::time::Duration::from_millis(1000))? {
        if let Event::Key(key) = event::read()? {
            disable_raw_mode()?;
            return Ok(key);
        } else {
            disable_raw_mode()?;
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid event captured",
            ));
        }
    } else {
        disable_raw_mode()?;
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid event captured",
        ));
    }
}
#[derive(Node)]
struct Movement;

impl Movement {
    fn move_node(node_storage: &storage::Storage<storage::Immutable>) -> Option<()> {
        let mut selected = node_storage.get_mut::<Selected>()?;
        let grid = create_new_node().grid;
        match getch().ok()?.code {
            KeyCode::Up => {
                *selected = Selected((grid.get(&selected.0)?.up).clone()?);
            }
            KeyCode::Down => {
                *selected = Selected((grid.get(&selected.0)?.down).clone()?);
            }
            KeyCode::Left => {
                *selected = Selected((grid.get(&selected.0)?.left).clone()?);
            }
            KeyCode::Right => {
                *selected = Selected((grid.get(&selected.0)?.right).clone()?);
            }
            _ => {}
        };
        Some(())
    }
}

fn compute_string(string: String) -> Option<f32> {
    let splited = string.split_whitespace().collect::<Vec<&str>>();
    let rhs: f32 = splited[2].parse().ok()?;
    let lhs: f32 = splited[0].parse().ok()?;

    match splited[1] {
        "+" => Some(lhs + rhs),
        "-" => Some(lhs - rhs),
        "*" => Some(lhs * rhs),
        "/" => {
            if rhs == 0.0 {
                None
            } else {
                Some(lhs / rhs)
            }
        }
        _ => None,
    }
}

fn input_controller(node_storage: storage::Storage<storage::Immutable>) {
    let mut current_number = node_storage.get_mut::<CurrentNumbers>().unwrap();
    let selected = node_storage.get_mut::<Selected>().unwrap();

    match getch().unwrap().code {
        KeyCode::Enter => match selected.0 {
            KeyPads::Zero => todo!(),
            KeyPads::One => todo!(),
            KeyPads::Two => todo!(),
            KeyPads::Three => todo!(),
            KeyPads::Four => todo!(),
            KeyPads::Five => todo!(),
            KeyPads::Six => todo!(),
            KeyPads::Seven => todo!(),
            KeyPads::Eight => todo!(),
            KeyPads::Nine => todo!(),
            KeyPads::Plus => todo!(),
            KeyPads::Minus => todo!(),
            KeyPads::Multiply => todo!(),
            KeyPads::Divide => todo!(),
            KeyPads::Equal => todo!(),
        },
        _ => {}
    }
}

#[derive(Node)]
struct CurrentNumbers(String);

fn main() {}

#[test]
fn parsing() {
    let t = "123 + 321";
    assert_eq!(compute_string(t.to_string()), Some(123.0 + 321.0));
}
