set(MINESWEEPER_SOURCES
    Cargo.toml
    src/lib.rs
    src/minesweeper_logic/field_type.rs
    src/minesweeper_logic/game.rs
    src/minesweeper_logic/mod.rs
    src/minesweeper_logic/results.rs
    src/minesweeper_logic/table.rs
)

cargo_add_rust_library(
  minesweeper
  MANIFEST_PATH
  Cargo.toml
  WITH_TESTS
  FOLDER
  minesweeper
  ${MINESWEEPER_SOURCES}
)

set(MINESWEEPER_DEMO_SOURCES ${MINESWEEPER_SOURCES} src/main.rs)

cargo_add_executable(
  minesweeper_demo
  MANIFEST_PATH
  Cargo.toml
  WITH_TESTS
  FOLDER
  minesweeper
  ${MINESWEEPER_DEMO_SOURCES}
)

set(CMINESWEEPER_SOURCES ${MINESWEEPER_SOURCES} ../cminesweeper/src/lib.rs
                         ../cminesweeper/Cargo.toml
)

cargo_add_library(
  cminesweeper
  MANIFEST_PATH
  ../cminesweeper/Cargo.toml
  WITH_TESTS
  FOLDER
  minesweeper
  ${CMINESWEEPER_SOURCES}
)
