cmake_minimum_required(VERSION 3.1.3)
project(rust_projects)

include(CTest)
enable_testing()

include(cmake/CargoBuild.cmake)

set(TESTS_PREFIX "${TESTS_PREFIX}rust_projects.")
add_subdirectory(monty-hall)
add_subdirectory(minesweeper)
