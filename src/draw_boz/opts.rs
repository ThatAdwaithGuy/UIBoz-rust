#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Colors {
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
pub enum Opts {
    ForeColor(Option<Colors>),
    BackGroundColor(Option<Colors>),
    Bold(Option<bool>),
    Dim(Option<bool>),
    Underline(Option<bool>),
    Blink(Option<bool>),
    Reverse(Option<bool>),
    Hide(Option<bool>),
}

// BOILERPLATE

pub fn parse_opts(lst: Vec<Opts>) -> String {
    let mut copy_lst = lst.clone();

    let mut filter_lst: Vec<Opts> = Vec::new();
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
            Opts::ForeColor(_) => {
                if is_fore_seen {
                    continue;
                }
                is_fore_seen = true;
                filter_lst.push(*i);
            }
            Opts::BackGroundColor(_) => {
                if is_back_seen {
                    continue;
                }
                is_back_seen = true;
                filter_lst.push(*i);
            }
            Opts::Bold(_) => {
                if is_bold_seen {
                    continue;
                }
                is_bold_seen = true;
                filter_lst.push(*i);
            }
            Opts::Dim(_) => {
                if is_dim_seen {
                    continue;
                }
                is_dim_seen = true;
                filter_lst.push(*i);
            }
            Opts::Underline(_) => {
                if is_underline_seen {
                    continue;
                }
                is_underline_seen = true;
                filter_lst.push(*i);
            }
            Opts::Blink(_) => {
                if is_blink_seen {
                    continue;
                }
                is_blink_seen = true;
                filter_lst.push(*i);
            }
            Opts::Reverse(_) => {
                if is_reverse_seen {
                    continue;
                }
                is_reverse_seen = true;
                filter_lst.push(*i);
            }
            Opts::Hide(_) => {
                if is_hide_seen {
                    continue;
                }
                is_hide_seen = true;
                filter_lst.push(*i);
            }
        }
    }

    if !is_fore_seen {
        filter_lst.push(Opts::ForeColor(None));
    }

    if !is_back_seen {
        filter_lst.push(Opts::BackGroundColor(None));
    }

    if !is_bold_seen {
        filter_lst.push(Opts::Bold(None));
    }

    if !is_dim_seen {
        filter_lst.push(Opts::Dim(None));
    }

    if !is_underline_seen {
        filter_lst.push(Opts::Underline(None));
    }

    if !is_blink_seen {
        filter_lst.push(Opts::Blink(None));
    }

    if !is_reverse_seen {
        filter_lst.push(Opts::Reverse(None));
    }

    if !is_hide_seen {
        filter_lst.push(Opts::Hide(None));
    }

    for i in filter_lst {
        match i {
            Opts::ForeColor(color) => {
                match color {
                    Some(val) => {
                        match val {
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
                                output_string.push_str(&format!(
                                    "\x1b[38;2;{:03};{:03};{:03}m",
                                    red, green, blue
                                ));
                            }
                        }
                    }
                    None => output_string.push_str("\x1b[0000000000000022m"),
                }
            }
            Opts::BackGroundColor(color) => {
                match color {
                    Some(val) => {
                        match val {
                            Colors::Red => output_string.push_str("\x1b[48;2;255;000;000"),
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
                                output_string.push_str(&format!(
                                    "\x1b[48;2;{:03};{:03};{:03}m",
                                    red, green, blue
                                ));
                            }
                        }
                    }
                    None => output_string.push_str("\x1b[0000000000000022m"),
                }
            }
            Opts::Bold(Some(true)) => output_string.push_str("\x1b[001m"),
            Opts::Dim(Some(true)) => output_string.push_str("\x1b[002m"), // Dim
            Opts::Underline(Some(true)) => output_string.push_str("\x1b[004m"), // Underline
            Opts::Blink(Some(true)) => output_string.push_str("\x1b[005m"), // Blink
            Opts::Reverse(Some(true)) => output_string.push_str("\x1b[007m"), // Reverse
            Opts::Hide(Some(true)) => output_string.push_str("\x1b[008m"),

            Opts::Bold(Some(false)) => output_string.push_str("\x1b[022m"), // Bold
            Opts::Dim(Some(false)) => output_string.push_str("\x1b[022m"),  // Dim
            Opts::Underline(Some(false)) => output_string.push_str("\x1b[022m"), // Underline
            Opts::Blink(Some(false)) => output_string.push_str("\x1b[022m"), // Blink
            Opts::Reverse(Some(false)) => output_string.push_str("\x1b[022m"), // Reverse
            Opts::Hide(Some(false)) => output_string.push_str("\x1b[022m"),

            Opts::Bold(None) => output_string.push_str("\x1b[022m"), // Bold
            Opts::Dim(None) => output_string.push_str("\x1b[022m"),  // Dim
            Opts::Underline(None) => output_string.push_str("\x1b[022m"), // Underline
            Opts::Blink(None) => output_string.push_str("\x1b[022m"), // Blink
            Opts::Reverse(None) => output_string.push_str("\x1b[022m"), // Reverse
            Opts::Hide(None) => output_string.push_str("\x1b[022m"),
        }
    }
    output_string
}
