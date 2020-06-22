use super::field_type::FieldType;
use std::collections::HashMap;
use strum_macros::Display;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Display, Debug)]
pub enum FieldFlagResult {
    Flagged,
    FlagRemoved,
    AlreadyOpened,
}

#[repr(C)]
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
