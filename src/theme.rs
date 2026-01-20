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
            path_divider: " › ",
            is_nerd: false,
            simple_icons: false,
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
            path_divider: " › ",
            is_nerd: false,
            simple_icons: false,
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
            path_divider: "  ",
            is_nerd: true,
            simple_icons: false,
        }
    }
}
