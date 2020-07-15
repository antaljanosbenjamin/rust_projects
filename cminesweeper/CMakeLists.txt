cmake_minimum_required(VERSION 3.1.3)

set(LIB_SOURCES Cargo.toml src/lib.rs)
cargo_add_library(cminesweeper "${LIB_SOURCES}" WITH_TESTS)
