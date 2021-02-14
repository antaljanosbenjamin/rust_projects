use libc::c_char;
use minesweeper::{FieldType, FlagResult, Game, GameLevel, OpenResult};
use std::cmp;
use std::convert::TryFrom;
use std::ptr;
use std::slice;
use strum_macros::Display;

#[repr(C)]
#[derive(Eq, PartialEq, Display, Debug)]
pub enum CError {
    Ok,
    InvalidInput,
    NullPointerAsInput,
    IndexIsOutOfRange,
    InsufficientBuffer,
    UnexpectedError,
}

type GameSizeType = minesweeper::SizeType;
type ArraySizeType = u64;

macro_rules! return_error {
    ($error_info_ptr:ident, $error_code:expr, $error_msg:expr) => {{
        let error_info = unsafe { &mut *$error_info_ptr };
        if $error_msg.len() > 0 && error_info.error_message_max_length > 0 {
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
                error_info.error_message_length = len_without_terminator as ArraySizeType;
            }
        }
        error_info.error_code = $error_code;
        return;
    }};
    ($error_info_ptr:ident, $error_code:expr) => {
        return_error!($error_info_ptr, $error_code, "");
    };
    ($error_info_ptr:ident) => {
        return_error!($error_info_ptr, CError::UnexpectedError);
    };
}

macro_rules! return_or_assign {
    ($x:expr, $error_info_ptr:ident, $error_code:expr) => {
        match $x {
            Ok(value) => value,
            Err(error_msg) => {
                return_error!($error_info_ptr, $error_code, error_msg);
            }
        }
    };
    ($x:expr, $error_info_ptr:ident) => {
        return_or_assign!($x, $error_info_ptr, CError::UnexpectedError)
    };
}

macro_rules! initialize_to_ok {
    ($error_info_ptr:ident) => {
        let mut error_info = unsafe { &mut *$error_info_ptr };
        error_info.error_code = CError::Ok;
        error_info.error_message_length = 0;
    };
}

// Based on this https://s3.amazonaws.com/temp.michaelfbryan.com/objects/index.html
#[repr(C)]
pub struct CFieldInfo {
    row: GameSizeType,
    column: GameSizeType,
    field_type: FieldType,
}

#[repr(C)]
pub struct COpenInfo {
    result: OpenResult,
    field_infos_length: ArraySizeType,
    field_infos_max_length: ArraySizeType,
    field_infos_ptr: *mut CFieldInfo,
}

#[repr(C)]
pub struct CErrorInfo {
    error_code: CError,
    error_message_length: ArraySizeType,
    error_message_max_length: ArraySizeType,
    error_message: *mut c_char,
}

#[no_mangle]
pub extern "C" fn minesweeper_new_game(
    game_ptr_ptr: *mut *mut Game,
    game_level: GameLevel,
    c_ei_ptr: *mut CErrorInfo,
) {
    initialize_to_ok!(c_ei_ptr);
    if game_ptr_ptr.is_null() {
        return_error!(c_ei_ptr, CError::NullPointerAsInput);
    }
    let game_ptr = unsafe { &mut *game_ptr_ptr };
    if !game_ptr.is_null() {
        return_error!(c_ei_ptr, CError::InvalidInput);
    }

    *game_ptr = Box::into_raw(Box::new(Game::new(game_level)));
}

#[no_mangle]
pub extern "C" fn minesweeper_game_open(
    game_ptr: *mut Game,
    row: GameSizeType,
    column: GameSizeType,
    c_open_info_ptr: *mut COpenInfo,
    c_ei_ptr: *mut CErrorInfo,
) {
    initialize_to_ok!(c_ei_ptr);
    if game_ptr.is_null() || c_open_info_ptr.is_null() {
        return_error!(c_ei_ptr, CError::NullPointerAsInput);
    }

    let mut c_open_info = unsafe { &mut *c_open_info_ptr };
    if c_open_info.field_infos_length != 0 {
        return_error!(c_ei_ptr, CError::InvalidInput);
    }
    if c_open_info.field_infos_max_length == 0 {
        return_error!(c_ei_ptr, CError::InsufficientBuffer);
    }
    if c_open_info.field_infos_ptr.is_null() {
        return_error!(c_ei_ptr, CError::NullPointerAsInput);
    }

    let game = unsafe { &mut *game_ptr };
    let open_info = return_or_assign!(game.open(row, column), c_ei_ptr);

    if open_info.field_infos.len() as ArraySizeType > c_open_info.field_infos_max_length {
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
    for (coords, field_type) in open_info.field_infos {
        c_field_infos[index].row = coords.0;
        c_field_infos[index].column = coords.1;
        c_field_infos[index].field_type = field_type.clone();
        index = index + 1;
    }
    c_open_info.field_infos_length = index as ArraySizeType;
}

#[no_mangle]
pub extern "C" fn minesweeper_game_toggle_flag(
    game_ptr: *mut Game,
    row: GameSizeType,
    column: GameSizeType,
    field_flag_result_ptr: *mut FlagResult,
    c_ei_ptr: *mut CErrorInfo,
) {
    initialize_to_ok!(c_ei_ptr);
    if game_ptr.is_null() || field_flag_result_ptr.is_null() {
        return_error!(c_ei_ptr, CError::NullPointerAsInput);
    }
    let game = unsafe { &mut *game_ptr };
    let flag_result = unsafe { &mut *field_flag_result_ptr };
    *flag_result = return_or_assign!(game.toggle_flag(row, column), c_ei_ptr);
}

#[no_mangle]
pub extern "C" fn minesweeper_destroy_game(
    game_ptr_ptr: *mut *mut Game,
    c_ei_ptr: *mut CErrorInfo,
) {
    initialize_to_ok!(c_ei_ptr);
    if game_ptr_ptr.is_null() {
        return_error!(c_ei_ptr, CError::NullPointerAsInput);
    }
    let game_ptr = unsafe { &mut *game_ptr_ptr };
    if game_ptr.is_null() {
        return;
    }
    let _ = unsafe {
        Box::<Game>::from_raw(*game_ptr);
    };
    *game_ptr = std::ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn minesweeper_game_get_width(
    game_ptr: *const Game,
    width_ptr: *mut GameSizeType,
    c_ei_ptr: *mut CErrorInfo,
) {
    initialize_to_ok!(c_ei_ptr);
    if game_ptr.is_null() || width_ptr.is_null() {
        return_error!(c_ei_ptr, CError::NullPointerAsInput);
    }
    let game = unsafe { &*game_ptr };
    let width = unsafe { &mut *width_ptr };
    *width = game.width();
}

#[no_mangle]
pub extern "C" fn minesweeper_game_get_height(
    game_ptr: *const Game,
    height_ptr: *mut GameSizeType,
    c_ei_ptr: *mut CErrorInfo,
) {
    initialize_to_ok!(c_ei_ptr);
    if game_ptr.is_null() || height_ptr.is_null() {
        return_error!(c_ei_ptr, CError::NullPointerAsInput);
    }
    let game = unsafe { &*game_ptr };
    let height = unsafe { &mut *height_ptr };
    *height = game.height();
}

#[no_mangle]
pub extern "C" fn minesweeper_game_get_elapsed_seconds(
    game_ptr: *const Game,
    elapsed_seconds_ptr: *mut u64,
    c_ei_ptr: *mut CErrorInfo,
) {
    initialize_to_ok!(c_ei_ptr);
    if game_ptr.is_null() || elapsed_seconds_ptr.is_null() {
        return_error!(c_ei_ptr, CError::NullPointerAsInput);
    }
    let game = unsafe { &*game_ptr };
    let elapsed_seconds = unsafe { &mut *elapsed_seconds_ptr };
    let elapsed_duration = game.get_elapsed();
    *elapsed_seconds = elapsed_duration.as_secs();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::CStr;

    macro_rules! assert_ok {
        ($error_info: expr) => {{
            let error_info = &$error_info;
            assert_ne!(
                0, error_info.error_message_max_length,
                "Use error message buffer!"
            );
            if error_info.error_message_length > 0 {
                let error_msg = unsafe { CStr::from_ptr(error_info.error_message) };
                assert_eq!(
                    CError::Ok,
                    error_info.error_code,
                    "{}",
                    error_msg.to_str().unwrap()
                );
            } else {
                assert_eq!(CError::Ok, error_info.error_code);
            }
        }};
    }

    struct BufferedData<S, B> {
        #[allow(dead_code)]
        buffer: Vec<B>,
        data: S,
    }

    fn create_open_info_with_size(size: usize) -> BufferedData<COpenInfo, CFieldInfo> {
        let mut buffer = Vec::with_capacity(size);
        let field_infos_ptr = buffer.as_mut_ptr();
        let field_infos_max_length = ArraySizeType::try_from(size).expect("Size conversion failed");
        let data = COpenInfo {
            result: OpenResult::Boom,
            field_infos_length: 0,
            field_infos_max_length,
            field_infos_ptr,
        };

        BufferedData { buffer, data }
    }

    fn create_open_info_for(game_ptr: *mut Game) -> BufferedData<COpenInfo, CFieldInfo> {
        let width = get_width(game_ptr);
        let height = get_height(game_ptr);
        create_open_info_with_size(usize::try_from(width * height).unwrap())
    }

    fn reset_error_info(error_info: &mut CErrorInfo) {
        error_info.error_code = CError::UnexpectedError;
        error_info.error_message_length = 0;
    }

    #[allow(dead_code)]
    fn create_error_info(max_error_length: usize) -> BufferedData<CErrorInfo, c_char> {
        let mut buffer = Vec::with_capacity(max_error_length);
        let error_message = buffer.as_mut_ptr();
        let mut data = CErrorInfo {
            error_code: CError::UnexpectedError,
            error_message_length: 0,
            error_message_max_length: ArraySizeType::try_from(max_error_length)
                .expect("Size conversion failed"),
            error_message,
        };
        reset_error_info(&mut data);
        BufferedData { buffer, data }
    }

    fn check_no_error(error_info: &CErrorInfo) {
        assert_eq!(CError::Ok, error_info.error_code);
    }

    fn create_empty_error_info() -> CErrorInfo {
        let mut result = CErrorInfo {
            error_code: CError::UnexpectedError,
            error_message_length: 0,
            error_message_max_length: 0,
            error_message: std::ptr::null_mut(),
        };

        reset_error_info(&mut result);
        result
    }

    fn get_width(game_ptr: *mut Game) -> GameSizeType {
        let mut error_info = create_empty_error_info();
        let mut width = 0;

        minesweeper_game_get_width(game_ptr, &mut width, &mut error_info);
        check_no_error(&error_info);
        assert!(width > 0);

        width
    }

    fn get_height(game_ptr: *mut Game) -> GameSizeType {
        let mut error_info = create_empty_error_info();
        let mut height = 0;

        minesweeper_game_get_height(game_ptr, &mut height, &mut error_info);
        check_no_error(&error_info);
        assert!(height > 0);

        height
    }

    fn create_game(level: GameLevel) -> *mut Game {
        let mut error_info = create_empty_error_info();

        let mut game_ptr: *mut Game = std::ptr::null_mut();
        minesweeper_new_game(&mut game_ptr, level, &mut error_info);
        check_no_error(&error_info);
        assert!(!game_ptr.is_null());

        let mut elapsed_seconds = 0;
        minesweeper_game_get_elapsed_seconds(game_ptr, &mut elapsed_seconds, &mut error_info);
        check_no_error(&error_info);
        assert_eq!(elapsed_seconds, 0);

        return game_ptr;
    }

    fn destroy_game(game_ptr: &mut *mut Game) {
        let mut error_info = create_empty_error_info();
        minesweeper_destroy_game(game_ptr, &mut error_info);
        check_no_error(&error_info);
    }

    #[test]
    fn new_game_with_nullptr() {
        let mut error_info = create_empty_error_info();
        let game_ptr_ptr = std::ptr::null_mut();
        minesweeper_new_game(game_ptr_ptr, GameLevel::Beginner, &mut error_info);
        assert_eq!(CError::NullPointerAsInput, error_info.error_code);
    }

    #[test]
    fn new_game_with_already_created_game() {
        let mut error_info = create_empty_error_info();
        let mut game_ptr = create_game(GameLevel::Beginner);
        minesweeper_new_game(&mut game_ptr, GameLevel::Beginner, &mut error_info);
        assert_eq!(CError::InvalidInput, error_info.error_code);
        destroy_game(&mut game_ptr);
    }

    #[test]
    fn destroy_game_with_nullptr() {
        let mut error_info = create_empty_error_info();
        minesweeper_destroy_game(std::ptr::null_mut(), &mut error_info);
        assert_eq!(CError::NullPointerAsInput, error_info.error_code);
    }

    #[test]
    fn destroy_game_double_destroy() {
        let mut game_ptr = create_game(GameLevel::Beginner);
        destroy_game(&mut game_ptr);
        let mut error_info = create_empty_error_info();
        minesweeper_destroy_game(&mut game_ptr, &mut error_info);
        check_no_error(&error_info);
    }

    #[test]
    fn create_and_destroy() {
        let mut error_info = create_empty_error_info();
        {
            let mut game_ptr = create_game(GameLevel::Beginner);
            assert!(!game_ptr.is_null());
            minesweeper_destroy_game(&mut game_ptr, &mut error_info);
            check_no_error(&error_info);
            assert!(game_ptr.is_null());
        }
        {
            let mut game_ptr = create_game(GameLevel::Intermediate);
            assert!(!game_ptr.is_null());
            minesweeper_destroy_game(&mut game_ptr, &mut error_info);
            check_no_error(&error_info);
            assert!(game_ptr.is_null());
        }
        {
            let mut game_ptr = create_game(GameLevel::Expert);
            assert!(!game_ptr.is_null());
            minesweeper_destroy_game(&mut game_ptr, &mut error_info);
            check_no_error(&error_info);
            assert!(game_ptr.is_null());
        }
    }

    #[test]
    fn open() {
        let mut game_ptr = create_game(GameLevel::Beginner);
        let width = get_width(game_ptr);
        let height = get_height(game_ptr);
        let mut buffered_open_info = create_open_info_for(game_ptr);
        let mut buffered_error_info = create_error_info(100);

        minesweeper_game_open(
            game_ptr,
            1,
            1,
            &mut buffered_open_info.data,
            &mut buffered_error_info.data,
        );
        assert_ok!(buffered_error_info.data);
        assert!(buffered_open_info.data.field_infos_length > 0);
        assert_eq!(CError::Ok, buffered_error_info.data.error_code);

        let field_infos_size = usize::try_from(buffered_open_info.data.field_infos_length).unwrap();
        let slice = unsafe {
            std::slice::from_raw_parts(buffered_open_info.data.field_infos_ptr, field_infos_size)
        };
        for field_info in slice.iter() {
            assert_ne!(FieldType::Mine, field_info.field_type);
            assert!(height >= field_info.row);
            assert!(width >= field_info.column);
        }
        destroy_game(&mut game_ptr);
    }

    #[test]
    fn open_with_nullptr_as_game() {
        let mut buffered_open_info = create_open_info_with_size(5);
        let mut error_info = create_empty_error_info();
        minesweeper_game_open(
            std::ptr::null_mut(),
            0,
            0,
            &mut buffered_open_info.data,
            &mut error_info,
        );
        assert_eq!(CError::NullPointerAsInput, error_info.error_code);
    }

    #[test]
    fn open_with_nullptr_as_open_info_ptr() {
        let mut game_ptr = create_game(GameLevel::Beginner);
        let mut error_info = create_empty_error_info();
        minesweeper_game_open(game_ptr, 0, 0, std::ptr::null_mut(), &mut error_info);
        assert_eq!(CError::NullPointerAsInput, error_info.error_code);
        destroy_game(&mut game_ptr);
    }

    #[test]
    fn open_with_not_empty_field_infos() {
        let mut game_ptr = create_game(GameLevel::Beginner);
        let mut buffered_open_info = create_open_info_with_size(5);
        buffered_open_info.data.field_infos_length = 1;
        let mut error_info = create_empty_error_info();
        minesweeper_game_open(
            game_ptr,
            0,
            0,
            &mut buffered_open_info.data,
            &mut error_info,
        );
        assert_eq!(CError::InvalidInput, error_info.error_code);
        destroy_game(&mut game_ptr);
    }

    #[test]
    fn open_with_nullptr_as_field_infos() {
        let mut game_ptr = create_game(GameLevel::Beginner);
        let mut buffered_open_info = create_open_info_with_size(5);
        buffered_open_info.data.field_infos_ptr = std::ptr::null_mut();
        let mut error_info = create_empty_error_info();
        minesweeper_game_open(
            game_ptr,
            0,
            0,
            &mut buffered_open_info.data,
            &mut error_info,
        );
        assert_eq!(CError::NullPointerAsInput, error_info.error_code);
        destroy_game(&mut game_ptr);
    }

    #[test]
    fn open_with_insufficient_buffer() {
        let mut game_ptr = create_game(GameLevel::Beginner);
        let mut buffered_open_info = create_open_info_with_size(5);
        buffered_open_info.data.field_infos_max_length = 0;
        let mut error_info = create_empty_error_info();
        minesweeper_game_open(
            game_ptr,
            0,
            0,
            &mut buffered_open_info.data,
            &mut error_info,
        );
        assert_eq!(CError::InsufficientBuffer, error_info.error_code);
        destroy_game(&mut game_ptr);
    }

    #[test]
    fn open_with_too_big_indices() {
        let mut game_ptr = create_game(GameLevel::Beginner);
        let mut buffered_open_info = create_open_info_with_size(5);
        let mut error_info = create_empty_error_info();

        let width = get_width(game_ptr);
        minesweeper_game_open(
            game_ptr,
            0,
            width,
            &mut buffered_open_info.data,
            &mut error_info,
        );
        assert_eq!(CError::UnexpectedError, error_info.error_code);

        error_info = create_empty_error_info();
        let height = get_width(game_ptr);
        minesweeper_game_open(
            game_ptr,
            height,
            0,
            &mut buffered_open_info.data,
            &mut error_info,
        );
        assert_eq!(CError::UnexpectedError, error_info.error_code);
        destroy_game(&mut game_ptr);
    }

    #[test]
    fn toggle_and_untoggle_flag() {
        let mut game_ptr = create_game(GameLevel::Beginner);
        let mut flag_result = FlagResult::AlreadyOpened;
        let mut buffered_error_info = create_error_info(100);
        minesweeper_game_toggle_flag(
            game_ptr,
            0,
            0,
            &mut flag_result,
            &mut buffered_error_info.data,
        );
        assert_ok!(buffered_error_info.data);
        assert_eq!(FlagResult::Flagged, flag_result);

        reset_error_info(&mut buffered_error_info.data);
        minesweeper_game_toggle_flag(
            game_ptr,
            0,
            0,
            &mut flag_result,
            &mut buffered_error_info.data,
        );
        assert_ok!(buffered_error_info.data);
        assert_eq!(FlagResult::FlagRemoved, flag_result);
        destroy_game(&mut game_ptr);
    }

    #[test]
    fn flag_opened() {
        let mut game_ptr = create_game(GameLevel::Beginner);
        let mut buffered_open_info = create_open_info_for(game_ptr);
        let mut buffered_error_info = create_error_info(100);
        minesweeper_game_open(
            game_ptr,
            0,
            0,
            &mut buffered_open_info.data,
            &mut buffered_error_info.data,
        );
        assert_ok!(buffered_error_info.data);

        reset_error_info(&mut buffered_error_info.data);
        let mut flag_result = FlagResult::AlreadyOpened;
        minesweeper_game_toggle_flag(
            game_ptr,
            0,
            0,
            &mut flag_result,
            &mut buffered_error_info.data,
        );
        assert_ok!(buffered_error_info.data);
        assert_eq!(FlagResult::AlreadyOpened, flag_result);
        destroy_game(&mut game_ptr);
    }

    #[test]
    fn get_width_with_nullptr_as_game() {
        let mut error_info = create_empty_error_info();
        let mut width = 0;
        minesweeper_game_get_width(std::ptr::null_mut(), &mut width, &mut error_info);
        assert_eq!(CError::NullPointerAsInput, error_info.error_code);
    }

    #[test]
    fn get_width_with_nullptr_as_width() {
        let mut error_info = create_empty_error_info();
        let mut game_ptr = create_game(GameLevel::Beginner);
        minesweeper_game_get_width(game_ptr, std::ptr::null_mut(), &mut error_info);
        assert_eq!(CError::NullPointerAsInput, error_info.error_code);
        destroy_game(&mut game_ptr);
    }

    #[test]
    fn get_height_with_nullptr_as_game() {
        let mut error_info = create_empty_error_info();
        let mut height = 0;
        minesweeper_game_get_height(std::ptr::null_mut(), &mut height, &mut error_info);
        assert_eq!(CError::NullPointerAsInput, error_info.error_code);
    }

    #[test]
    fn get_height_with_nullptr_as_height() {
        let mut error_info = create_empty_error_info();
        let mut game_ptr = create_game(GameLevel::Beginner);
        minesweeper_game_get_height(game_ptr, std::ptr::null_mut(), &mut error_info);
        assert_eq!(CError::NullPointerAsInput, error_info.error_code);
        destroy_game(&mut game_ptr);
    }

    #[test]
    fn get_elapsed_seconds_with_nullptr_as_game() {
        let mut error_info = create_empty_error_info();
        let mut elapsed = 0;
        minesweeper_game_get_elapsed_seconds(std::ptr::null_mut(), &mut elapsed, &mut error_info);
        assert_eq!(CError::NullPointerAsInput, error_info.error_code);
    }

    #[test]
    fn get_elapsed_seconds_with_nullptr_as_elapsed() {
        let mut error_info = create_empty_error_info();
        let mut game_ptr = create_game(GameLevel::Beginner);
        minesweeper_game_get_elapsed_seconds(game_ptr, std::ptr::null_mut(), &mut error_info);
        assert_eq!(CError::NullPointerAsInput, error_info.error_code);
        destroy_game(&mut game_ptr);
    }
}
