use super::field_type::FieldType;
use std::collections::HashMap;
use strum_macros::Display;

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Display, Debug)]
pub enum FieldFlagResult {
    Flagged,
    FlagRemoved,
    AlreadyOpened,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Display, Debug)]
pub enum OpenResult {
    Ok,
    IsFlagged,
    Boom,
    WINNER,
}

#[derive(PartialEq)]
pub struct OpenInfo {
    pub result: OpenResult,
    pub field_infos: HashMap<(usize, usize), FieldType>,
}
