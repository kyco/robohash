use strum_macros::EnumIter;

#[derive(PartialEq, Clone, Debug, EnumIter)]
pub enum Set {
    Default,
    Set1,
    Set2,
    Set3,
    Set4,
    Set5,
}

impl Set {
    pub fn as_str(&self) -> &str {
        match self {
            Set::Default => "set1",
            Set::Set1 => "set1",
            Set::Set2 => "set2",
            Set::Set3 => "set3",
            Set::Set4 => "set4",
            Set::Set5 => "set5",
        }
    }
}
