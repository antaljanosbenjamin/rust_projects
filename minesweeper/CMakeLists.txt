cmake_minimum_required(VERSION 3.1.3)

set(MINESWEEPER_SOURCES
    Cargo.toml
    src/minesweeper_logic/field_type.rs
    src/minesweeper_logic/game.rs
    src/minesweeper_logic/mod.rs
    src/minesweeper_logic/results.rs
    src/minesweeper_logic/table.rs
    src/lib.rs
)
set(MINESWEEPER_DEMO_SOURCES ${MINESWEEPER_SOURCES} src/main.rs)

cargo_add_rust_library(minesweeper "${MINESWEEPER_SOURCES}" WITH_TESTS)
cargo_add_executable(minesweeper_demo "${MINESWEEPER_DEMO_SOURCES}" WITH_TESTS)
