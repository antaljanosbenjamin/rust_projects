set(MONTY_HALL_SOURCES Cargo.toml src/main.rs)

cargo_add_executable(monty-hall FOLDER Others ${MONTY_HALL_SOURCES})
