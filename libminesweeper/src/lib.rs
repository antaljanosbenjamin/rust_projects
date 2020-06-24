use minesweeper::{FieldType, Game, GameLevel, OpenResult};
use std::convert::TryFrom;
use std::slice;

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

#[no_mangle]
pub extern "C" fn new_game(game_ptr_ptr: *mut *mut Game, game_level: GameLevel) {
    if game_ptr_ptr.is_null() {
        return;
    }
    let game_ptr = unsafe { &mut *game_ptr_ptr };
    if !game_ptr.is_null() {
        return;
    }

    let boxed_game = Box::new(Game::new(game_level));
    *game_ptr = Box::into_raw(boxed_game);
}

#[no_mangle]
pub extern "C" fn game_open(
    game_ptr: *mut Game,
    row: u32,
    column: u32,
    c_open_info_ptr: *mut COpenInfo,
) {
    if game_ptr.is_null() || c_open_info_ptr.is_null() {
        return;
    }
    let mut c_open_info = unsafe { &mut *c_open_info_ptr };
    if c_open_info.field_infos_length != 0 || c_open_info.field_infos_max_length == 0 {
        return;
    }
    let game = unsafe { &mut *game_ptr };
    let open_info = match game.open(row as usize, column as usize) {
        Ok(ok) => ok,
        _ => return,
    };
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
            _ => return,
        };
        c_field_infos[index].column = match u32::try_from(coords.1) {
            Ok(value) => value,
            _ => return,
        };
        c_field_infos[index].field_type = field_type.clone();
        c_open_info.field_infos_length = index as u64;
        index = index + 1;
    }
}

#[no_mangle]
pub extern "C" fn destroy_game(game_ptr: *mut Game) {
    if game_ptr.is_null() {
        return;
    }
    let boxed_game = unsafe {
        Box::from_raw(game_ptr);
    };
    drop(boxed_game);
}
