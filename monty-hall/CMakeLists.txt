cmake_minimum_required(VERSION 3.1.3)

set(MONTY_HALL_SOURCES Cargo.toml src/main.rs)
cargo_add_executable(monty-hall "${MONTY_HALL_SOURCES}")
