cmake_minimum_required(VERSION 3.1.3)
project(rust_projects)

include(CTest)
enable_testing()

include(cmake/CargoBuild.cmake)

add_subdirectory(monty-hall)
add_subdirectory(minesweeper)
add_subdirectory(cminesweeper)
