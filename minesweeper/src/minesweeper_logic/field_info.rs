use strum_macros::Display;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Display, Debug)]
pub enum FieldState {
    Closed,
    Opened,
    Flagged,
}

impl FieldState {
    pub fn is_opened(&self) -> bool {
        self == &FieldState::Opened
    }

    pub fn is_flagged(&self) -> bool {
        self == &FieldState::Flagged
    }
}

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
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct FieldInfo {
    pub state: FieldState,
    pub field_type: FieldType,
}
