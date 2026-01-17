use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum ThemeType {
    #[default]
    Ascii,
    Unicode,
    Nerd,
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
    pub is_nerd: bool,
}

impl Theme {
    pub fn new(t: ThemeType) -> Self {
        match t {
            ThemeType::Ascii => Self::ascii(),
            ThemeType::Unicode => Self::unicode(),
            ThemeType::Nerd => Self::nerd(),
        }
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
            is_nerd: false,
        }
    }

    pub fn unicode() -> Self {
        Theme {
            tree_vertical: '│',
            tree_branch: '├',
            tree_end: '└',
            tree_dash: '─',
            icon_dir: "📁 ", // Unicode folder? Or just empty? Roadmap says Unicode has smoother lines.
            icon_file: "",
            diff_bar_plus: '█',
            diff_bar_minus: '█',
            is_nerd: false,
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
            diff_bar_plus: '█',
            diff_bar_minus: '█',
            is_nerd: true,
        }
    }
}
