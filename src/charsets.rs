pub const BLOCK: &[&str] = &[" ", "░", "▒", "▒", "▓", "▓", "▓", "█"];
pub const CHINESE: &[&str] = &[
    "\u{3000}", "一", "二", "十", "人", "丁", "口", "王", "日", "木", "金", "華", "爱", "黑", "墨",
    "龍", "龘",
];
pub const DEFAULT: &[&str] = &[
    " ", ".", "`", "^", "\"", "\\", ",", ":", ";", "I", "l", "!", "i", ">", "<", "~", "+", "_",
    "-", "?", "]", "[", "}", "{", "1", ")", "(", "|", "\\", "/", "t", "f", "j", "r", "x", "n", "u",
    "v", "c", "z", "X", "Y", "U", "J", "C", "L", "Q", "0", "O", "Z", "m", "w", "q", "p", "d", "b",
    "k", "h", "a", "o", "*", "#", "M", "W", "&", "8", "%", "B", "$", "@",
];
pub const EMOJI: &[&str] = &[
    "\u{3000}", "\u{3000}", "。", "，", "🧔", "👶", "🗣", "👥", "👤", "👀", "👁", "🦴", "🦷", "🫁",
    "🫀", "🧠", "👃", "🦻", "👂", "👅", "🦀", "👿", "🦀", "👄", "🤳", "💅", "🖖", "👆", "🙏", "🤝",
    "🦿", "🦾", "💪", "🤏", "👌", "🤘", "🤞", "👊", "🤚", "🤛", "🙌", "😾", "😿", "🙀", "😺", "👾",
    "👽", "👻", "💀", "👺", "🦀", "👹", "🤡", "💤", "😴", "🥸", "🥳", "🥶", "🥵", "🤮", "🤢", "🤕",
    "😭", "😓", "😯", "😰", "😨", "😱", "😮", "😩", "😫", "🙁", "😔", "😡", "🤬", "😠", "🙄", "😐",
    "😶", "🧐", "😛", "🤗", "🤐", "🤑", "😝", "🤩", "😋", "😊", "😉", "🤣", "😅", "😆",
];
pub const RUSSIAN: &[&str] = &[
    " ", " ", "Я", "Ю", "Э", "Ь", "Ы", "Ъ", "Щ", "Ш", "Ч", "Ц", "Х", "Ф", "У", "Т", "С", "P", "П",
    "О", "Н", "М", "Л", "К", "Й", "И", "З", "Ж", "Ё", "Е", "Д", "Г", "В", "Б", "А",
];
pub const SLIGHT: &[&str] = &[
    " ", ".", "`", "\"", "\\", ":", "I", "!", ">", "~", "_", "?", "[", "{", "|", ")", "(", "\\",
    "\\\\", "/", "Y", "L", "p", "d", "a", "*", "W", "8", "%", "@", "$",
];
pub const MINIMAL: &[&str] = &[
    " ", " ", ".", ":", "'", ";", "o", "l", "d", "c", "x", "0", "k", "K", "N", "M", "W", "X",
];

/// Represents all Charsets that exist in this program.
pub enum Charset {
    Block,
    Chinese,
    Default,
    Emoji,
    Russian,
    Slight,
    Minimal,
}

/// Get a `Charset` from an enum.
pub fn from_enum(charset: Charset) -> &'static [&'static str] {
    match charset {
        Charset::Block => BLOCK,
        Charset::Chinese => CHINESE,
        Charset::Default => DEFAULT,
        Charset::Emoji => EMOJI,
        Charset::Russian => RUSSIAN,
        Charset::Slight => SLIGHT,
        Charset::Minimal => MINIMAL,
    }
}

/// Converts a string into a `Charset`.
pub fn to_charset_enum(s: &str) -> Option<Charset> {
    match s {
        "block" => Some(Charset::Block),
        "chinese" => Some(Charset::Chinese),
        "default" => Some(Charset::Default),
        "emoji" => Some(Charset::Emoji),
        "russian" => Some(Charset::Russian),
        "slight" => Some(Charset::Slight),
        "minimal" => Some(Charset::Minimal),
        _ => None,
    }
}

/// Get a charset from a string slice.
pub fn from_str(s: &str) -> Option<&[&str]> {
    match s {
        "block" => Some(BLOCK),
        "chinese" => Some(CHINESE),
        "default" => Some(DEFAULT),
        "emoji" => Some(EMOJI),
        "russian" => Some(RUSSIAN),
        "slight" => Some(SLIGHT),
        "minimal" => Some(MINIMAL),
        _ => None,
    }
}
