// https://users.rust-lang.org/t/noob-enum-string-with-symbols-resolved/7668/5
pub enum MainCommands {
    FormatPrint,
    Primitives,
    CustomTypes,
    Testing,
    BorrowChecker,
    ConversionTraits,
    ControlFlow,
    Functions,
    GenCompletions,
}

impl MainCommands {
    pub fn from_str(s: &str) -> Option<MainCommands> {
        match s {
            "format-print" => Some(MainCommands::FormatPrint),
            "primitives" => Some(MainCommands::Primitives),
            "custom-types" => Some(MainCommands::CustomTypes),
            "testing" => Some(MainCommands::Testing),
            "borrow-checker" => Some(MainCommands::BorrowChecker),
            "conversion-traits" => Some(MainCommands::ConversionTraits),
            "control-flow" => Some(MainCommands::ControlFlow),
            "functions" => Some(MainCommands::Functions),
            "gen-completions" => Some(MainCommands::GenCompletions),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            MainCommands::FormatPrint => "format-print",
            MainCommands::Primitives => "primitives",
            MainCommands::CustomTypes => "custom-types",
            MainCommands::Testing => "testing",
            MainCommands::BorrowChecker => "borrow-checker",
            MainCommands::ConversionTraits => "conversion-traits",
            MainCommands::ControlFlow => "control-flow",
            MainCommands::Functions => "functions",
            MainCommands::GenCompletions => "gen-completions",
        }
    }
}
