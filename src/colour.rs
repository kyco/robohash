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
    pub fn as_str(&self) -> &str {
        match self {
            Colour::Any => "",
            Colour::Blue => "blue",
            Colour::Brown => "brown",
            Colour::Green => "green",
            Colour::Grey => "grey",
            Colour::Orange => "orange",
            Colour::Pink => "pink",
            Colour::Purple => "purple",
            Colour::Red => "red",
            Colour::White => "white",
            Colour::Yellow => "yellow",
        }
    }
}
