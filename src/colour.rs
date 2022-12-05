use strum_macros::EnumIter;

#[derive(PartialEq, Clone, Debug, EnumIter)]
pub enum Colour {
    Any,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Pink,
    Purple,
    Red,
    White,
    Yellow,
}

impl Colour {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Colour::Any => None,
            Colour::Blue => Some("blue"),
            Colour::Brown => Some("brown"),
            Colour::Green => Some("green"),
            Colour::Grey => Some("grey"),
            Colour::Orange => Some("orange"),
            Colour::Pink => Some("pink"),
            Colour::Purple => Some("purple"),
            Colour::Red => Some("red"),
            Colour::White => Some("white"),
            Colour::Yellow => Some("yellow"),
        }
    }
}
