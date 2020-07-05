// https://users.rust-lang.org/t/noob-enum-string-with-symbols-resolved/7668/5
pub enum MainCommands {
    FormatPrint,
    Testing,
    Structs,
    GenCompletions,
}

impl MainCommands {
    pub fn from_str(s: &str) -> Option<MainCommands> {
        match s {
            "format-print" => Some(MainCommands::FormatPrint),
            "testing" => Some(MainCommands::Testing),
            "structs" => Some(MainCommands::Structs),
            "gen-completions" => Some(MainCommands::GenCompletions),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            MainCommands::FormatPrint => "format-print",
            MainCommands::Testing => "testing",
            MainCommands::Structs => "structs",
            MainCommands::GenCompletions => "gen-completions",
        }
    }
}

