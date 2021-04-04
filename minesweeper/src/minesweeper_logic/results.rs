use super::basic_types::SizeType;
use super::field_info::FieldType;
use std::collections::HashMap;
use strum_macros::Display;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Display, Debug)]
pub enum FlagResult {
    Flagged,
    FlagRemoved,
    AlreadyOpened,
}

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Display, Debug)]
pub enum OpenResult {
    Ok,
    IsFlagged,
    Boom,
    WINNER,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct OpenInfo {
    pub result: OpenResult,
    pub newly_opened_fields: HashMap<(SizeType, SizeType), FieldType>,
}
