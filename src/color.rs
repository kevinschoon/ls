pub enum Color {
    Normal,
    BlackFg,      // 30
    RedFg,        // 31
    GreenFg,      // 32
    YellowFgBold, // 33
    BlueFgBold,   // 34
    PurpleFg,     // 35
    CyanFgBold,   // 36
}

pub fn paint(text: String, color: Color) -> String {
    match color {
        Color::Normal => text,
        Color::BlackFg => format!("\x1B[0;30m{}\x1B[0m", text),
        Color::RedFg => format!("\x1B[0;31m{}\x1B[0m", text),
        Color::GreenFg => format!("\x1B[0;32m{}\x1B[0m", text),
        Color::YellowFgBold => format!("\x1B[1;33m{}\x1B[0m", text),
        Color::BlueFgBold => format!("\x1B[1;34m{}\x1B[0m", text),
        Color::PurpleFg => format!("\x1B[0;35m{}\x1B[0m", text),
        Color::CyanFgBold => format!("\x1B[1;36m{}\x1B[0m", text),
    }
}
