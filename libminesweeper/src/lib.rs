use libc::c_char;
use minesweeper::{FieldType, Game, GameLevel, OpenResult};
use std::cmp;
use std::convert::TryFrom;
use std::ptr;
use std::slice;

#[repr(C)]
pub enum CError {
    Ok,
    NullPointerAsInput,
    InvalidInput,
    InsufficientBuffer,
    UnexpectedError,
}

macro_rules! return_error {
    ($error_info_ptr:expr, $error_code:expr,  $error_msg:expr) => {{
        let error_info = unsafe { &mut *$error_info_ptr };
        let ref error_message = $error_msg;
        let error_msg_len = error_message.len();
        let src = error_message.as_bytes().as_ptr();
        let len_without_terminator = cmp::min(
            usize::try_from(error_info.error_message_max_length - 1).unwrap_or(usize::MAX),
            error_msg_len,
        );
        unsafe {
            ptr::copy_nonoverlapping(
                src,
                error_info.error_message as *mut u8,
                len_without_terminator,
            );
            *error_info
                .error_message
                .offset(len_without_terminator as isize) = 0;
            error_info.error_message_length = len_without_terminator as u64;
        }
        error_info.error_code = $error_code;
        return;
    }};
    ($error_info_ptr:expr, $error_code:expr) => {
        return_error!($error_info_ptr, $error_code, "");
    };
    ($error_info_ptr:expr) => {
        return_error!($error_info_ptr, CError::UnexpectedError);
    };
}

macro_rules! return_or_assign {
    ($x:expr, $error_info_ptr:expr, $error_code:expr) => {
        match $x {
            Ok(value) => value,
            Err(error_msg) => {
                return_error!($error_info_ptr, $error_code, error_msg);
            }
        }
    };
    ($x:expr, $error_info_ptr:expr) => {
        return_or_assign!($x, $error_info_ptr, CError::UnexpectedError)
    };
}

// Based on this https://s3.amazonaws.com/temp.michaelfbryan.com/objects/index.html

#[repr(C)]
pub struct CFieldInfo {
    row: u32,
    column: u32,
    field_type: FieldType,
}

#[repr(C)]
pub struct COpenInfo {
    result: OpenResult,
    field_infos_length: u64,
    field_infos_max_length: u64,
    field_infos_ptr: *mut CFieldInfo,
}

#[repr(C)]
pub struct CErrorInfo {
    error_code: CError,
    error_message_length: u64,
    error_message_max_length: u64,
    error_message: *mut c_char,
}

#[no_mangle]
pub extern "C" fn new_game(
    game_ptr_ptr: *mut *mut Game,
    game_level: GameLevel,
    ei_ptr: *mut CErrorInfo,
) {
    if game_ptr_ptr.is_null() {
        return_error!(ei_ptr, CError::NullPointerAsInput);
    }
    let game_ptr = unsafe { &mut *game_ptr_ptr };
    if !game_ptr.is_null() {
        return_error!(ei_ptr, CError::InvalidInput);
    }

    *game_ptr = Box::into_raw(Box::new(Game::new(game_level)));
}

#[no_mangle]
pub extern "C" fn game_open(
    game_ptr: *mut Game,
    row: u32,
    column: u32,
    c_open_info_ptr: *mut COpenInfo,
    ei_ptr: *mut CErrorInfo,
) {
    if game_ptr.is_null() || c_open_info_ptr.is_null() {
        return_error!(ei_ptr, CError::NullPointerAsInput);
    }
    let mut c_open_info = unsafe { &mut *c_open_info_ptr };
    if c_open_info.field_infos_length != 0 {
        return_error!(ei_ptr, CError::InvalidInput);
    }
    if c_open_info.field_infos_max_length == 0 {
        return_error!(ei_ptr, CError::InsufficientBuffer);
    }
    let game = unsafe { &mut *game_ptr };
    let open_info = return_or_assign!(game.open(row as usize, column as usize), ei_ptr);

    if open_info.field_infos.len() as u64 > c_open_info.field_infos_max_length {
        return;
    }
    c_open_info.result = open_info.result;
    let c_field_infos: &mut [CFieldInfo] = unsafe {
        slice::from_raw_parts_mut(
            c_open_info.field_infos_ptr,
            c_open_info.field_infos_max_length as usize,
        )
    };
    let mut index: usize = 0;
    for (coords, field_type) in open_info.field_infos.iter() {
        c_field_infos[index].row = match u32::try_from(coords.0) {
            Ok(value) => value,
            _ => return_error!(ei_ptr),
        };
        c_field_infos[index].column = match u32::try_from(coords.1) {
            Ok(value) => value,
            _ => return_error!(ei_ptr),
        };
        c_field_infos[index].field_type = field_type.clone();
        index = index + 1;
    }
    c_open_info.field_infos_length = index as u64;
}

#[no_mangle]
pub extern "C" fn destroy_game(game_ptr: *mut Game) {
    if game_ptr.is_null() {
        return;
    }
    let _ = unsafe {
        Box::from_raw(game_ptr);
    };
}
