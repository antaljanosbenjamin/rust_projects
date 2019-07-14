use crate::logic::table::{Table};

pub trait Field {

    fn get_char_repr(&self) -> char;

    fn open(&mut self) {}
}

pub struct DummyField;

impl Field for DummyField {

    fn get_char_repr(&self) -> char {
        'D'
    }
}

pub struct EmptyField;

impl Field for EmptyField {

    fn get_char_repr(&self) -> char {
        ' '
    }
}
