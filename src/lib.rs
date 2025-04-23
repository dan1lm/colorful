use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}


impl Color {

    //  ANSI color code
    // foreground color
    fn fg_code(&self) -> u8 {
        match self {
            Color::Black => 30,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Magenta => 35,
            Color::Cyan => 36,
            Color::White => 37,
            Color::BrightBlack => 90,
            Color::BrightRed => 91,
            Color::BrightGreen => 92,
            Color::BrightYellow => 93,
            Color::BrightBlue => 94,
            Color::BrightMagenta => 95,
            Color::BrightCyan => 96,
            Color::BrightWhite => 97,
        }
    }

    // background color
    fn bg_code(&self) -> {

        match self {
            Color::Black => 40,
            Color::Red => 41,
            Color::Green => 42,
            Color::Yellow => 43,
            Color::Blue => 44,
            Color::Magenta => 45,
            Color::Cyan => 46,
            Color::White => 47,
            Color::BrightBlack => 100,
            Color::BrightRed => 101,
            Color::BrightGreen => 102,
            Color::BrightYellow => 103,
            Color::BrightBlue => 104,
            Color::BrightMagenta => 105,
            Color::BrightCyan => 106,
            Color::BrightWhite => 107,
        }
    }
}



#[derive(Debug, Clone, Copy)]
pub struct Style {
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    bold: bool,
    italic: bool,
    underline: bool,
}


impl Default for Style {
    fn default() -> Self {
        Style {
            fg_color: None,
            bg_color: None,
            bold: false,
            italic: false,
            underline: false,
        }
    }
}

impl Style {
    pub fn new() -> Self {               
        Style::default()
    }
    // note: look into a better implementation
    pub fn fg(mut self, color: Color) -> Self {
        self.fg_color = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }




    fn format_prefix(&self) -> String {
        let mut codes = Vec::new();

        if let Some(fg) = self.fg_color {
            codes.push(fg.fg_code().to_string());
        }

        if let Some(bg) = self.bg_color {
            codes.push(bg.bg_code().to_string());
        }

        if self.bold {
            codes.push("1".to_string());
        }

        if self.italic {
            codes.push("3".to_string());
        }

        if self.underline {
            codes.push("4".to_string());
        }

        if codes.is_empty() {
            return String::new();
        }

        format!("\x1b[{}m", codes.join(";"))
    }

    fn format_suffix() -> &'static str {
        "\x1b[0m"
    }


    pub fn paint<T: AsRef<str>>(&self, text: T) -> ColoredString {
        ColoredString {
            text: text.as_ref().to_string(),
            style: *self,
        }
    }

}

#[derive(Debug, Clone)]
pub struct ColoredString {
    text: String,
    style: Style,
}


impl fmt::Display for ColoredString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = self.style.format_prefix();
        if prefix.is_empty() {
            write!(f, "{}", self.text)
        } else {
            write!(f, "{}{}{}", prefix, self.text, Style::format_suffix())
        }
    }
}