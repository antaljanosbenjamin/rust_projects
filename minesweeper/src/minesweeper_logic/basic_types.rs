#[cfg(not(target_pointer_width = "64"))]
compile_error!("This crate can only be used on 64-bit systems.");

pub type SizeType = i64;
