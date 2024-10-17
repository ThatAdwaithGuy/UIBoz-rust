use crossterm::event::KeyCode;
use frontend::node::Node;
use node_proc_macro::Node;
use nodes::inputs::getch;
use renderer::window_renderer::*;
use std::collections::HashMap;

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

fn render_text(node_storage: &frontend::node::Storage<frontend::node::Immutable>) -> Vec<TextType> {
    let selected = node_storage.get_mut::<Selected>().unwrap();
    let mut texts = vec![];
    let full_line = "-".repeat(23);
    texts.push(renderer::sub_win::TextType::Text(Text::new(
        &full_line,
        5,
        0,
        &[],
    )));

    // Operator veritical dividers
    for i in 1..22 {
        texts.push(renderer::sub_win::TextType::Text(Text::new(
            "|",
            5 + i,
            17,
            &[],
        )));
    }

    // Operator horizontal dividers
    texts.push(renderer::sub_win::TextType::Text(Text::new(
        "-----",
        9,
        18,
        &[],
    )));
    texts.push(renderer::sub_win::TextType::Text(Text::new(
        "-----",
        13,
        18,
        &[],
    )));
    texts.push(renderer::sub_win::TextType::Text(Text::new(
        "-----",
        17,
        18,
        &[],
    )));
    texts.push(renderer::sub_win::TextType::Text(Text::new(
        "-----",
        21,
        18,
        &[],
    )));

    // Number pad horizontal dividers
    texts.push(renderer::sub_win::TextType::Text(Text::new(
        "----- ----- -----",
        12,
        0,
        &[],
    )));

    texts.push(renderer::sub_win::TextType::Text(Text::new(
        "----- ----- -----",
        19,
        0,
        &[],
    )));

    texts.push(renderer::sub_win::TextType::Text(Text::new(
        "-----",
        23,
        6,
        &[],
    )));

    // Number pad veritical dividers
    for i in 1..7 {
        texts.push(renderer::sub_win::TextType::Text(Text::new(
            "|",
            5 + i,
            5,
            &[],
        )));

        texts.push(renderer::sub_win::TextType::Text(Text::new(
            "|",
            5 + i,
            11,
            &[],
        )));
    }

    for i in 8..14 {
        texts.push(renderer::sub_win::TextType::Text(Text::new(
            "|",
            5 + i,
            5,
            &[],
        )));

        texts.push(renderer::sub_win::TextType::Text(Text::new(
            "|",
            5 + i,
            11,
            &[],
        )));
    }

    for i in 15..27 {
        texts.push(renderer::sub_win::TextType::Text(Text::new(
            "|",
            5 + i,
            5,
            &[],
        )));

        texts.push(renderer::sub_win::TextType::Text(Text::new(
            "|",
            5 + i,
            11,
            &[],
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
                    &[style::TextStyle::Reverse(true)],
                )));
            } else {
                texts.push(TextType::Text(Text::new(
                    $string,
                    $line_number,
                    $column,
                    &[style::TextStyle::Reverse(false)],
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
        &[style::TextStyle::Bold(true)],
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

fn move_node(node_storage: &frontend::Storage<frontend::Immutable>) -> Option<()> {
    let mut selected = node_storage.get_mut::<Selected>()?;
    let grid = create_new_node().grid;
    match nodes::inputs::getch().ok()?.code {
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

fn compute_string(string: String) -> Option<f32> {
    let mut lhs = "".to_string();
    let mut rhs = "".to_string();
    let mut operator = "".to_string();
    let mut rhs_reached = false;

    for c in string.chars() {
        if !rhs_reached {
            if c.is_ascii_digit() {
                lhs.push(c);
            } else {
                match c {
                    '+' => {
                        operator = "+".to_string();
                        rhs_reached = true;
                    }
                    '-' => {
                        operator = "-".to_string();
                        rhs_reached = true;
                    }
                    '*' => {
                        operator = "*".to_string();
                        rhs_reached = true;
                    }
                    '/' => {
                        operator = "/".to_string();
                        rhs_reached = true;
                    }
                    _ => {
                        return None;
                    }
                }
            }
        } else {
            rhs.push(c);
        }
    }
    let lhs_num: i32 = lhs.parse::<i32>().ok()?;
    let rhs_num: i32 = rhs.parse::<i32>().ok()?;

    match operator.as_str() {
        "+" => Some((lhs_num + rhs_num) as f32),
        "-" => Some((lhs_num - rhs_num) as f32),
        "*" => Some((lhs_num * rhs_num) as f32),
        "/" => {
            if rhs_num == 0 {
                None
            } else {
                Some((lhs_num / rhs_num) as f32)
            }
        }
        _ => None,
    }
}

fn input_controller(
    node_storage: &frontend::node::Storage<frontend::node::Immutable>,
) -> Option<()> {
    let mut current_numbers = node_storage.get_mut::<Output>()?;
    let selected = node_storage.get_mut::<Selected>()?;

    match getch().ok()?.code {
        KeyCode::Enter => match selected.0 {
            KeyPads::Zero => current_numbers.0.push_str("0"),
            KeyPads::One => current_numbers.0.push_str("1"),
            KeyPads::Two => current_numbers.0.push_str("2"),
            KeyPads::Three => current_numbers.0.push_str("3"),
            KeyPads::Four => current_numbers.0.push_str("4"),
            KeyPads::Five => current_numbers.0.push_str("5"),
            KeyPads::Six => current_numbers.0.push_str("6"),
            KeyPads::Seven => current_numbers.0.push_str("7"),
            KeyPads::Eight => current_numbers.0.push_str("8"),
            KeyPads::Nine => current_numbers.0.push_str("9"),
            KeyPads::Plus => current_numbers.0.push_str("+"),
            KeyPads::Minus => current_numbers.0.push_str("-"),
            KeyPads::Multiply => current_numbers.0.push_str("*"),
            KeyPads::Divide => current_numbers.0.push_str("/"),
            KeyPads::Equal => {
                current_numbers.0 = compute_string(current_numbers.0.clone())?.to_string();
            }
        },
        KeyCode::Char('q') => panic!("bye"),
        _ => {
            return None;
        }
    };
    Some(())
}
#[derive(Node)]
struct View;

impl frontend::ViewNode for View {
    fn view(&self, storage: &frontend::node::Storage<frontend::node::Immutable>) -> Option<String> {
        let texts = render_text(storage);
        let win = renderer::window::Window::new(texts, 23, 26, TypeOfBorder::CurvedBorders);
        print!("\x1b[2J\x1b[H");
        std::thread::sleep(std::time::Duration::from_millis(100));
        Some(win.render().ok()?)
    }
}
#[derive(Node)]
struct Update;

impl frontend::ControllerNode for Update {
    fn update(
        &mut self,
        storage: &frontend::node::Storage<frontend::node::Immutable>,
    ) -> Option<()> {
        move_node(storage)?;
        input_controller(storage)?;
        Some(())
    }
}

fn main() {
    let mut node_storage = frontend::node::Storage::new();
    node_storage.put_mut(Selected(KeyPads::Zero));
    node_storage.put_mut(frontend::Runtime { is_running: true });
    node_storage.put_mut(Output("1+1".to_string()));
    //let node_storage = node_storage.into_immutable();
    //dbg!(move_node(&node_storage));
    //dbg!(compute_string("1+1".to_string()));
    //dbg!(input_controller(&node_storage));
    //dbg!(node_storage.get_mut::<Selected>());
    //

    //let texts = render_text(&node_storage);
    //let win = renderer::window::Window::new(texts, 23, 26, TypeOfBorder::CurvedBorders);
    //println!("{}", win.render().unwrap());
    let mut app =
        frontend::App::<View, Update>::new(View {}, Update {}, node_storage.into_immutable());
    app.run();
}

#[test]
fn parsing() {
    let t = "123 + 321";
    assert_eq!(compute_string(t.to_string()), Some(123.0 + 321.0));
}
