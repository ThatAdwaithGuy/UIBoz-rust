#[derive(Debug, Clone, std::marker::Copy, PartialEq, Eq, Hash)]
pub enum Colors {
    Blank,
    Red,
    Green,
    Blue,
    Orange,
    Violet,
    Black,
    White,
    Yellow,
    Cyan,
    Purple,
    Grey,
    Pink,
    Brown,
    Magenta,
    Gold,
    RGB { red: i32, green: i32, blue: i32 },
}
#[derive(Debug, Clone, std::marker::Copy, PartialEq, Eq, Hash)]
pub enum TextOpts {
    ForeColor(Colors),
    BackGroundColor(Colors),
    Bold(bool),
    Dim(bool),
    Underline(bool),
    Blink(bool),
    Reverse(bool),
    Hide(bool),
}

// BOILERPLATE

pub fn parse_text_opts(lst: Vec<TextOpts>) -> String {
    let mut copy_lst = lst.clone();

    let mut filter_lst: Vec<TextOpts> = Vec::new();
    let mut output_string: String = r#""#.to_string();

    let mut is_fore_seen: bool = false;
    let mut is_back_seen: bool = false;
    let mut is_bold_seen: bool = false;
    let mut is_dim_seen: bool = false;
    let mut is_underline_seen: bool = false;
    let mut is_blink_seen: bool = false;
    let mut is_reverse_seen: bool = false;
    let mut is_hide_seen: bool = false;

    for i in &copy_lst {
        match i {
            TextOpts::ForeColor(_) => {
                if is_fore_seen {
                    continue;
                }
                is_fore_seen = true;
                filter_lst.push(*i);
            }
            TextOpts::BackGroundColor(_) => {
                if is_back_seen {
                    continue;
                }
                is_back_seen = true;
                filter_lst.push(*i);
            }
            TextOpts::Bold(_) => {
                if is_bold_seen {
                    continue;
                }
                is_bold_seen = true;
                filter_lst.push(*i);
            }
            TextOpts::Dim(_) => {
                if is_dim_seen {
                    continue;
                }
                is_dim_seen = true;
                filter_lst.push(*i);
            }
            TextOpts::Underline(_) => {
                if is_underline_seen {
                    continue;
                }
                is_underline_seen = true;
                filter_lst.push(*i);
            }
            TextOpts::Blink(_) => {
                if is_blink_seen {
                    continue;
                }
                is_blink_seen = true;
                filter_lst.push(*i);
            }
            TextOpts::Reverse(_) => {
                if is_reverse_seen {
                    continue;
                }
                is_reverse_seen = true;
                filter_lst.push(*i);
            }
            TextOpts::Hide(_) => {
                if is_hide_seen {
                    continue;
                }
                is_hide_seen = true;
                filter_lst.push(*i);
            }
        }
    }

    if !is_fore_seen {
        filter_lst.push(TextOpts::ForeColor(Colors::Blank));
    }

    if !is_back_seen {
        filter_lst.push(TextOpts::BackGroundColor(Colors::Blank));
    }

    if !is_bold_seen {
        filter_lst.push(TextOpts::Bold(false));
    }

    if !is_dim_seen {
        filter_lst.push(TextOpts::Dim(false));
    }

    if !is_underline_seen {
        filter_lst.push(TextOpts::Underline(false));
    }

    if !is_blink_seen {
        filter_lst.push(TextOpts::Blink(false));
    }

    if !is_reverse_seen {
        filter_lst.push(TextOpts::Reverse(false));
    }

    if !is_hide_seen {
        filter_lst.push(TextOpts::Hide(false));
    }

    for i in filter_lst {
        match i {
            TextOpts::ForeColor(color) => {
                match color {
                    Colors::Red => output_string.push_str("\x1b[38;2;255;000;000m"),
                    Colors::Green => output_string.push_str("\x1b[38;2;000;255;000m"), // Green (true color)
                    Colors::Blue => output_string.push_str("\x1b[38;2;000;000;255m"), // Blue (true color)
                    Colors::Orange => output_string.push_str("\x1b[38;2;255;165;000m"), // Orange (true color)
                    Colors::Violet => output_string.push_str("\x1b[38;2;138;043;226m"), // Violet (true color)
                    Colors::Black => output_string.push_str("\x1b[38;2;000;000;000m"), // Black (true color)
                    Colors::White => output_string.push_str("\x1b[38;2;255;255;255m"), // White (true color)
                    Colors::Yellow => output_string.push_str("\x1b[38;2;255;255;000m"), // Yellow (true color)
                    Colors::Cyan => output_string.push_str("\x1b[38;2;000;255;255m"), // Cyan (true color)
                    Colors::Purple => output_string.push_str("\x1b[38;2;128;000;128m"), // Purple (true color)
                    Colors::Grey => output_string.push_str("\x1b[38;2;169;169;169m"), // Grey (true color)
                    Colors::Pink => output_string.push_str("\x1b[38;2;255;182;193m"), // Pink (true color)
                    Colors::Brown => output_string.push_str("\x1b[38;2;165;042;042m"), // Brown (true color)
                    Colors::Magenta => output_string.push_str("\x1b[38;2;255;000;255m"), // Magenta (true color)
                    Colors::Gold => output_string.push_str("\x1b[38;2;255;215;000m"), // Gold (true color)
                    Colors::RGB { red, green, blue } => {
                        output_string
                            .push_str(&format!("\x1b[38;2;{:03};{:03};{:03}m", red, green, blue));
                    }
                    Colors::Blank => output_string.push_str("\x1b[0000000000000022m"),
                }
            }
            TextOpts::BackGroundColor(color) => {
                match color {
                    Colors::Red => output_string.push_str("\x1b[48;2;255;000;000m"),
                    Colors::Green => output_string.push_str("\x1b[48;2;000;255;000m"), // Green (true color background)
                    Colors::Blue => output_string.push_str("\x1b[48;2;000;000;255m"), // Blue (true color background)
                    Colors::Orange => output_string.push_str("\x1b[48;2;255;165;000m"), // Orange (true color background)
                    Colors::Violet => output_string.push_str("\x1b[48;2;138;043;226m"), // Violet (true color background)
                    Colors::Black => output_string.push_str("\x1b[48;2;000;000;000m"), // Black (true color background)
                    Colors::White => output_string.push_str("\x1b[48;2;255;255;255m"), // White (true color background)
                    Colors::Yellow => output_string.push_str("\x1b[48;2;255;255;000m"), // Yellow (true color background)
                    Colors::Cyan => output_string.push_str("\x1b[48;2;000;255;255m"), // Cyan (true color background)
                    Colors::Purple => output_string.push_str("\x1b[48;2;128;000;128m"), // Purple (true color background)
                    Colors::Grey => output_string.push_str("\x1b[48;2;169;169;169m"), // Grey (true color background)
                    Colors::Pink => output_string.push_str("\x1b[48;2;255;182;193m"), // Pink (true color background)
                    Colors::Brown => output_string.push_str("\x1b[48;2;165;042;042m"), // Brown (true color background)
                    Colors::Magenta => output_string.push_str("\x1b[48;2;255;000;255m"), // Magenta (true color background)
                    Colors::Gold => output_string.push_str("\x1b[48;2;255;215;000m"), // Gold (true color background)
                    Colors::RGB { red, green, blue } => {
                        output_string
                            .push_str(&format!("\x1b[48;2;{:03};{:03};{:03}m", red, green, blue));
                    }
                    Colors::Blank => output_string.push_str("\x1b[0000000000000022m"),
                }
            }
            TextOpts::Bold(true) => output_string.push_str("\x1b[001m"),
            TextOpts::Dim(true) => output_string.push_str("\x1b[002m"), // Dim
            TextOpts::Underline(true) => output_string.push_str("\x1b[004m"), // Underline
            TextOpts::Blink(true) => output_string.push_str("\x1b[005m"), // Blink
            TextOpts::Reverse(true) => output_string.push_str("\x1b[007m"), // Reverse
            TextOpts::Hide(true) => output_string.push_str("\x1b[008m"),

            TextOpts::Bold(false) => output_string.push_str("\x1b[022m"), // Bold
            TextOpts::Dim(false) => output_string.push_str("\x1b[022m"),  // Dim
            TextOpts::Underline(false) => output_string.push_str("\x1b[022m"), // Underline
            TextOpts::Blink(false) => output_string.push_str("\x1b[022m"), // Blink
            TextOpts::Reverse(false) => output_string.push_str("\x1b[022m"), // Reverse
            TextOpts::Hide(false) => output_string.push_str("\x1b[022m"),
        }
    }
    output_string
}
use rand::seq::SliceRandom;

fn generate_random_text_opts() -> Vec<TextOpts> {
    let colors = [
        Colors::Blank,
        Colors::Red,
        Colors::Green,
        Colors::Blue,
        Colors::Orange,
        Colors::Violet,
        Colors::Black,
        Colors::White,
        Colors::Yellow,
        Colors::Cyan,
        Colors::Purple,
        Colors::Grey,
        Colors::Pink,
        Colors::Brown,
        Colors::Magenta,
        Colors::Gold,
        Colors::RGB {
            red: 0,
            green: 0,
            blue: 0,
        },
    ];

    let bool_options = [true, false];

    let mut rng = rand::thread_rng();

    let random_combination: Vec<TextOpts> = vec![
        TextOpts::ForeColor(*colors.choose(&mut rng).unwrap()),
        TextOpts::BackGroundColor(*colors.choose(&mut rng).unwrap()),
        TextOpts::Bold(*bool_options.choose(&mut rng).unwrap()),
        TextOpts::Dim(*bool_options.choose(&mut rng).unwrap()),
        TextOpts::Underline(*bool_options.choose(&mut rng).unwrap()),
        TextOpts::Blink(*bool_options.choose(&mut rng).unwrap()),
        TextOpts::Reverse(*bool_options.choose(&mut rng).unwrap()),
        TextOpts::Hide(*bool_options.choose(&mut rng).unwrap()),
    ];

    random_combination
}
