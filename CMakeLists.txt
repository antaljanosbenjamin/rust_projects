cmake_minimum_required(VERSION 3.1.3)
project(rust_projects)

include(CTest)
enable_testing()

include(cmake/CargoBuild.cmake)

cargo_add_executable(monty-hall monty-hall/Cargo.toml)
cargo_add_rust_library(minesweeper minesweeper/Cargo.toml WITH_TESTS)
cargo_add_executable(minesweeper_demo minesweeper/Cargo.toml WITH_TESTS)
cargo_add_library(cminesweeper MANIFEST_PATH cminesweeper/Cargo.toml WITH_TESTS)
