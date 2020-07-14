use strum_macros::Display;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Display, Debug)]
pub enum FieldType {
    Empty,
    Numbered(u8),
    Mine,
}

impl FieldType {
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self == &FieldType::Empty
    }

    #[allow(dead_code)]
    pub fn is_mine(&self) -> bool {
        self == &FieldType::Mine
    }

    #[allow(dead_code)]
    pub fn is_numbered(&self) -> bool {
        match self {
            FieldType::Numbered(_) => true,
            _ => false,
        }
    }

    pub fn get_char_repr(&self) -> char {
        match self {
            FieldType::Empty => ' ',
            FieldType::Numbered(x) => std::char::from_digit(*x as u32, 10).unwrap(),
            FieldType::Mine => 'X',
        }
    }
}
