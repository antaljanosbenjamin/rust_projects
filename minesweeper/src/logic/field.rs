use std::char;

#[derive(PartialEq)]
pub enum FieldState {
    Closed,
    Opened,
}

impl FieldState {
    fn is_opened(&self) -> bool {
        self == &FieldState::Opened
    }
}

#[derive(PartialEq)]
pub enum FieldOpenResult {
    AlreadyOpened,
    SimpleOpen,
    MultiOpen,
    Boom,
}

pub trait Field {
    fn get_char_repr(&self) -> char;

    fn open(&mut self) -> FieldOpenResult;

    fn is_mine(&self) -> bool {
        false
    }
}

impl Field {
    pub fn new(mine: bool, value: u8) -> Box<dyn Field> {
        if mine {
            Box::new(MineField::new())
        } else if value == 0 {
            Box::new(EmptyField::new())
        } else {
            Box::new(NumberedField::new(value))
        }
    }
}

#[allow(dead_code)]
pub struct DummyField;

impl Field for DummyField {
    fn get_char_repr(&self) -> char {
        'D'
    }

    fn open(&mut self) -> FieldOpenResult {
        FieldOpenResult::SimpleOpen
    }
}

pub struct EmptyField {
    state: FieldState,
}

impl EmptyField {
    fn new() -> EmptyField {
        EmptyField {
            state: FieldState::Closed,
        }
    }
}
impl Field for EmptyField {
    fn get_char_repr(&self) -> char {
        ' '
    }

    fn open(&mut self) -> FieldOpenResult {
        if self.state.is_opened() {
            FieldOpenResult::AlreadyOpened
        } else {
            self.state = FieldState::Opened;
            FieldOpenResult::MultiOpen
        }
    }
}

pub struct NumberedField {
    state: FieldState,
    value: u8,
}

impl NumberedField {
    fn new(value: u8) -> NumberedField {
        NumberedField {
            state: FieldState::Closed,
            value,
        }
    }
}

impl Field for NumberedField {
    fn get_char_repr(&self) -> char {
        std::char::from_digit(self.value as u32, 10).unwrap()
    }

    fn open(&mut self) -> FieldOpenResult {
        if self.state.is_opened() {
            FieldOpenResult::AlreadyOpened
        } else {
            self.state = FieldState::Opened;
            FieldOpenResult::SimpleOpen
        }
    }
}

pub struct MineField {
    state: FieldState,
}

impl MineField {
    fn new() -> MineField {
        MineField {
            state: FieldState::Closed,
        }
    }
}

impl Field for MineField {
    fn get_char_repr(&self) -> char {
        'X'
    }

    fn open(&mut self) -> FieldOpenResult {
        if self.state.is_opened() {
            FieldOpenResult::AlreadyOpened
        } else {
            self.state = FieldState::Opened;
            FieldOpenResult::Boom
        }
    }

    fn is_mine(&self) -> bool {
        true
    }
}
