#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Colors {
    #[default]
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
    RGB {
        red: i32,
        green: i32,
        blue: i32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextStyle {
    ForeColor(Colors),
    BackGroundColor(Colors),
    Blank,
    Bold,
    Dim,
    Underline,
    Blink,
    Reverse,
    Hide,
    // What is the use of the number. I wrote this and forgot. :)
    // Wait, is it column
    UpSideConnect(i32),
    DownSideConnect(i32),
    RightSideConnect,
    LeftSideConnect,
}

pub struct TextStyleBuilder {
    styles: Vec<TextStyle>,
}

impl TextStyleBuilder {
    pub fn new() -> Self {
        Self { styles: Vec::new() }
    }

    pub fn fore_color(mut self, color: Colors) -> Self {
        self.styles.push(TextStyle::ForeColor(color));
        self
    }

    pub fn background_color(mut self, color: Colors) -> Self {
        self.styles.push(TextStyle::BackGroundColor(color));
        self
    }

    pub fn blank(mut self) -> Self {
        self.styles.push(TextStyle::Blank);
        self
    }

    pub fn bold(mut self) -> Self {
        self.styles.push(TextStyle::Bold);
        self
    }

    pub fn dim(mut self) -> Self {
        self.styles.push(TextStyle::Dim);
        self
    }

    pub fn underline(mut self) -> Self {
        self.styles.push(TextStyle::Underline);
        self
    }

    pub fn blink(mut self) -> Self {
        self.styles.push(TextStyle::Blink);
        self
    }

    pub fn reverse(mut self) -> Self {
        self.styles.push(TextStyle::Reverse);
        self
    }

    pub fn hide(mut self) -> Self {
        self.styles.push(TextStyle::Hide);
        self
    }

    pub fn up_side_connect(mut self, val: i32) -> Self {
        self.styles.push(TextStyle::UpSideConnect(val));
        self
    }

    pub fn down_side_connect(mut self, val: i32) -> Self {
        self.styles.push(TextStyle::DownSideConnect(val));
        self
    }

    pub fn right_side_connect(mut self) -> Self {
        self.styles.push(TextStyle::RightSideConnect);
        self
    }

    pub fn left_side_connect(mut self) -> Self {
        self.styles.push(TextStyle::LeftSideConnect);
        self
    }

    pub fn build(self) -> Vec<TextStyle> {
        self.styles
    }
}

// BOILERPLATE
pub fn parse_text_style(lst: Vec<TextStyle>) -> String {
    let mut output_string = String::new();

    let mut seen = std::collections::HashSet::new();
    let mut filter_lst: Vec<TextStyle> = Vec::new();

    for style in lst {
        let key = std::mem::discriminant(&style);
        if seen.insert(key) {
            filter_lst.push(style);
        }
    }

    let is_fore_seen = seen.contains(&std::mem::discriminant(&TextStyle::ForeColor(
        Colors::Blank,
    )));
    let is_back_seen = seen.contains(&std::mem::discriminant(&TextStyle::BackGroundColor(
        Colors::Blank,
    )));
    let is_up_seen = seen.contains(&std::mem::discriminant(&TextStyle::UpSideConnect(0)));
    let is_down_seen = seen.contains(&std::mem::discriminant(&TextStyle::DownSideConnect(0)));

    if !is_fore_seen {
        filter_lst.push(TextStyle::ForeColor(Colors::Blank));
    }
    if !is_back_seen {
        filter_lst.push(TextStyle::BackGroundColor(Colors::Blank));
    }
    if !is_up_seen {
        filter_lst.push(TextStyle::UpSideConnect(-1));
    }
    if !is_down_seen {
        filter_lst.push(TextStyle::DownSideConnect(-1));
    }

    let fore_code = |color| match color {
        Colors::Red => "\x1b[38;2;255;000;000m".to_string(),
        Colors::Green => "\x1b[38;2;000;255;000m".to_string(),
        Colors::Blue => "\x1b[38;2;000;000;255m".to_string(),
        Colors::Orange => "\x1b[38;2;255;165;000m".to_string(),
        Colors::Violet => "\x1b[38;2;138;043;226m".to_string(),
        Colors::Black => "\x1b[38;2;000;000;000m".to_string(),
        Colors::White => "\x1b[38;2;255;255;255m".to_string(),
        Colors::Yellow => "\x1b[38;2;255;255;000m".to_string(),
        Colors::Cyan => "\x1b[38;2;000;255;255m".to_string(),
        Colors::Purple => "\x1b[38;2;128;000;128m".to_string(),
        Colors::Grey => "\x1b[38;2;169;169;169m".to_string(),
        Colors::Pink => "\x1b[38;2;255;182;193m".to_string(),
        Colors::Brown => "\x1b[38;2;165;042;042m".to_string(),
        Colors::Magenta => "\x1b[38;2;255;000;255m".to_string(),
        Colors::Gold => "\x1b[38;2;255;215;000m".to_string(),
        Colors::RGB { red, green, blue } => {
            format!("\x1b[38;2;{:03};{:03};{:03}m", red, green, blue)
        }
        Colors::Blank => "\x1b[0000000000000010m".to_string(),
    };

    let back_code = |color| match color {
        Colors::Red => "\x1b[48;2;255;000;000m".to_string(),
        Colors::Green => "\x1b[48;2;000;255;000m".to_string(),
        Colors::Blue => "\x1b[48;2;000;000;255m".to_string(),
        Colors::Orange => "\x1b[48;2;255;165;000m".to_string(),
        Colors::Violet => "\x1b[48;2;138;043;226m".to_string(),
        Colors::Black => "\x1b[48;2;000;000;000m".to_string(),
        Colors::White => "\x1b[48;2;255;255;255m".to_string(),
        Colors::Yellow => "\x1b[48;2;255;255;000m".to_string(),
        Colors::Cyan => "\x1b[48;2;000;255;255m".to_string(),
        Colors::Purple => "\x1b[48;2;128;000;128m".to_string(),
        Colors::Grey => "\x1b[48;2;169;169;169m".to_string(),
        Colors::Pink => "\x1b[48;2;255;182;193m".to_string(),
        Colors::Brown => "\x1b[48;2;165;042;042m".to_string(),
        Colors::Magenta => "\x1b[48;2;255;000;255m".to_string(),
        Colors::Gold => "\x1b[48;2;255;215;000m".to_string(),
        Colors::RGB { red, green, blue } => {
            format!("\x1b[48;2;{:03};{:03};{:03}m", red, green, blue)
        }
        Colors::Blank => "\x1b[0000000000000010m".to_string(),
    };

    for style in filter_lst {
        match style {
            TextStyle::ForeColor(color) => output_string.push_str(&fore_code(color)),
            TextStyle::BackGroundColor(color) => output_string.push_str(&back_code(color)),
            _ => {}
        }
    }

    let style_codes = [
        (TextStyle::Dim, "\x1b[0000000000000002m"),
        (TextStyle::Underline, "\x1b[0000000000000004m"),
        (TextStyle::Blink, "\x1b[0000000000000005m"),
        (TextStyle::Reverse, "\x1b[0000000000000007m"),
        (TextStyle::Hide, "\x1b[0000000000000008m"),
        (TextStyle::RightSideConnect, "\x1b[0000000000000010m"),
        (TextStyle::LeftSideConnect, "\x1b[0000000000000010m"),
        (TextStyle::Bold, "\x1b[0000000000000001m"),
    ];

    for (style, code) in &style_codes {
        if seen.contains(&std::mem::discriminant(style)) {
            output_string.push_str(code);
        } else {
            output_string.push_str("\x1b[0000000000000010m");
        }
    }

    output_string
}
