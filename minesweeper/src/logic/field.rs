use std::char;

#[derive(PartialEq)]
pub enum FieldState {
    Closed,
    Opened,
    Flagged,
}

impl FieldState {
    fn is_opened(&self) -> bool {
        self == &FieldState::Opened
    }

    fn is_flagged(&self) -> bool {
        self == &FieldState::Flagged
    }
}

#[derive(PartialEq)]
pub enum FieldOpenResult {
    AlreadyOpened,
    SimpleOpen,
    MultiOpen,
    Boom,
    IsFlagged,
}

pub trait Field {
    fn get_field_state(&self) -> &FieldState;

    fn set_field_state(&mut self, state: FieldState);

    fn get_char_repr_inner(&self) -> char;

    fn get_open_result_inner(&self) -> FieldOpenResult;

    fn get_char_repr(&self) -> char {
        if !self.get_field_state().is_opened() {
            'O'
        } else {
            self.get_char_repr_inner()
        }
    }

    fn open(&mut self) -> FieldOpenResult {
        if self.get_field_state().is_flagged() {
            FieldOpenResult::IsFlagged
        } else if self.get_field_state().is_opened() {
            FieldOpenResult::AlreadyOpened
        } else {
            self.set_field_state(FieldState::Opened);
            self.get_open_result_inner()
        }
    }

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

impl Field for EmptyField {
    fn set_field_state(&mut self, state: FieldState) {
        self.state = state;
    }

    fn get_field_state(&self) -> &FieldState {
        &self.state
    }

    fn get_char_repr_inner(&self) -> char {
        ' '
    }

    fn get_open_result_inner(&self) -> FieldOpenResult {
        FieldOpenResult::MultiOpen
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
    fn set_field_state(&mut self, state: FieldState) {
        self.state = state;
    }

    fn get_field_state(&self) -> &FieldState {
        &self.state
    }

    fn get_char_repr_inner(&self) -> char {
        std::char::from_digit(self.value as u32, 10).unwrap()
    }

    fn get_open_result_inner(&self) -> FieldOpenResult {
        FieldOpenResult::SimpleOpen
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
    fn set_field_state(&mut self, state: FieldState) {
        self.state = state;
    }

    fn get_field_state(&self) -> &FieldState {
        &self.state
    }

    fn get_char_repr_inner(&self) -> char {
        'X'
    }

    fn get_open_result_inner(&self) -> FieldOpenResult {
        FieldOpenResult::Boom
    }

    fn is_mine(&self) -> bool {
        true
    }
}
