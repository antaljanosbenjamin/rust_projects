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

pub trait FieldInner {
    fn get_char_repr_inner(&self) -> char;

    fn get_field_state(&self) -> &FieldState;
}

pub trait Field: FieldInner {
    fn get_char_repr(&self) -> char {
        if !self.get_field_state().is_opened() {
            'O'
        } else {
            self.get_char_repr_inner()
        }
    }

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

impl FieldInner for EmptyField {
    fn get_char_repr_inner(&self) -> char {
        ' '
    }

    fn get_field_state(&self) -> &FieldState {
        &self.state
    }
}

impl Field for EmptyField {
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

impl FieldInner for NumberedField {
    fn get_char_repr_inner(&self) -> char {
        std::char::from_digit(self.value as u32, 10).unwrap()
    }

    fn get_field_state(&self) -> &FieldState {
        &self.state
    }
}

impl Field for NumberedField {
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

impl FieldInner for MineField {
    fn get_char_repr_inner(&self) -> char {
        'X'
    }

    fn get_field_state(&self) -> &FieldState {
        &self.state
    }
}

impl Field for MineField {
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
