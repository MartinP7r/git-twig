use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum, Default, PartialEq)]
pub enum ThemeType {
    #[default]
    Ascii,
    Unicode,
    Rounded,
    Nerd,
}

impl ThemeType {
    pub fn next(&self) -> Self {
        match self {
            ThemeType::Ascii => ThemeType::Unicode,
            ThemeType::Unicode => ThemeType::Rounded,
            ThemeType::Rounded => ThemeType::Nerd,
            ThemeType::Nerd => ThemeType::Ascii,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub tree_vertical: char,
    pub tree_branch: char,
    pub tree_end: char,
    pub tree_dash: char,
    pub icon_dir: &'static str,
    pub icon_file: &'static str,
    pub diff_bar_plus: char,
    pub diff_bar_minus: char,
    pub path_divider: &'static str,
    pub is_nerd: bool,
    pub simple_icons: bool,
    pub color_dir: ratatui::style::Color,
    pub color_file: ratatui::style::Color,
}

impl Theme {
    pub fn new(t: ThemeType) -> Self {
        match t {
            ThemeType::Ascii => Self::ascii(),
            ThemeType::Unicode => Self::unicode(),
            ThemeType::Rounded => Self::rounded(),
            ThemeType::Nerd => Self::nerd(),
        }
    }

    pub fn with_simple_icons(mut self, simple: bool) -> Self {
        self.simple_icons = simple;
        self
    }

    pub fn ascii() -> Self {
        Theme {
            tree_vertical: '|',
            tree_branch: '|',
            tree_end: '`',
            tree_dash: '-',
            icon_dir: "",
            icon_file: "",
            diff_bar_plus: '+',
            diff_bar_minus: '-',
            path_divider: "/",
            is_nerd: false,
            simple_icons: false,
            color_dir: ratatui::style::Color::Rgb(255, 170, 0), // Orange
            color_file: ratatui::style::Color::Reset,
        }
    }

    pub fn unicode() -> Self {
        Theme {
            tree_vertical: '│',
            tree_branch: '├',
            tree_end: '└',
            tree_dash: '─',
            icon_dir: "",
            icon_file: "",
            diff_bar_plus: '◼',
            diff_bar_minus: '◼',
            path_divider: "・",
            is_nerd: false,
            simple_icons: false,
            color_dir: ratatui::style::Color::Rgb(255, 170, 0), // Orange
            color_file: ratatui::style::Color::Reset,
        }
    }

    pub fn rounded() -> Self {
        Theme {
            tree_vertical: '│',
            tree_branch: '├',
            tree_end: '╰',
            tree_dash: '─',
            icon_dir: "",
            icon_file: "",
            diff_bar_plus: '◼',
            diff_bar_minus: '◼',
            path_divider: "・",
            is_nerd: false,
            simple_icons: false,
            color_dir: ratatui::style::Color::Rgb(255, 170, 0), // Orange
            color_file: ratatui::style::Color::Reset,
        }
    }

    pub fn nerd() -> Self {
        Theme {
            tree_vertical: '│',
            tree_branch: '├',
            tree_end: '└',
            tree_dash: '─',
            icon_dir: " ",
            icon_file: " ",
            diff_bar_plus: '◼',
            diff_bar_minus: '◼',
            path_divider: "・",
            is_nerd: true,
            simple_icons: false,
            color_dir: ratatui::style::Color::Rgb(255, 170, 0), // Orange
            color_file: ratatui::style::Color::Reset,
        }
    }

    pub fn load_overrides(&mut self) {
        let colors = crate::git::get_config_regexp("twig.color.");
        for (key, val) in colors {
            let color_name = key.replace("twig.color.", "");
            if let Some(color) = parse_color(&val) {
                match color_name.as_str() {
                    "dir" | "folder" => self.color_dir = color,
                    "file" => self.color_file = color,
                    _ => {}
                }
            }
        }

        let icons = crate::git::get_config_regexp("twig.icon.");
        for (key, val) in icons {
            let icon_name = key.replace("twig.icon.", "");
            // Note: we need to handle the fact that icons are strings but Theme uses &'static str
            // For now, we'll use Box::leak if it's dynamic, or just support it differently.
            // A simpler way for icons: just Leak them as we don't expect many.
            let leaked_icon: &'static str = Box::leak(val.into_boxed_str());
            match icon_name.as_str() {
                "dir" | "folder" => self.icon_dir = leaked_icon,
                "file" => self.icon_file = leaked_icon,
                _ => {}
            }
        }
    }
}

fn parse_color(s: &str) -> Option<ratatui::style::Color> {
    use ratatui::style::Color;
    match s.to_lowercase().as_str() {
        "reset" => Some(Color::Reset),
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "gray" => Some(Color::Gray),
        "dark_gray" => Some(Color::DarkGray),
        "light_red" => Some(Color::LightRed),
        "light_green" => Some(Color::LightGreen),
        "light_yellow" => Some(Color::LightYellow),
        "light_blue" => Some(Color::LightBlue),
        "light_magenta" => Some(Color::LightMagenta),
        "light_cyan" => Some(Color::LightCyan),
        "white" => Some(Color::White),
        "orange" => Some(Color::Rgb(255, 170, 0)),
        _ if s.starts_with('#') && s.len() == 7 => {
            let r = u8::from_str_radix(&s[1..3], 16).ok()?;
            let g = u8::from_str_radix(&s[3..5], 16).ok()?;
            let b = u8::from_str_radix(&s[5..7], 16).ok()?;
            Some(Color::Rgb(r, g, b))
        }
        _ => None,
    }
}
