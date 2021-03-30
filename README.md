# My personal rust playground

This repo contains my personal rust adventures. As I am learning Rust via developing small applications/libraries, the code might not be the most idiomatic rust nor the best organized. Apart from these rust snippets, you will also find a working (but probably not perfect) example of CMake integration. If you find anything useful, feel free to use it.

## Rust projects

 - [Monty Hall](monty_hall): A very simple simulation of the [Monty Hall problem](https://en.wikipedia.org/wiki/Monty_Hall_problem). It was my first Rust application apart from the tutorials.
 - [Minesweeper](minesweeper): Another implementation of the good old Minesweeper. The crate contains the `minesweeper` lib which encapsulates the business logic and the `minesweeper_demo` app which presents the business logic within a CLI application.
  - [CMinesweeper](cminesweeper): A C wrapper API without the  corresponding header file. The goal is to use the `minesweeper` lib in a C++ based GUI application. For further information, please see my other [repo](https://github.com/antaljanosbenjamin/miscellaneous/blob/feature/minesweeper-gui/projects/minesweeper/cminesweeper_wrapper/include/CMinesweeper.hpp).

## CMake integration

I wrote a general Rust integration for CMake for study purpose only, therefore it might have bugs. Feel free to use, any comments are very welcome! In the end it might become production ready wrapper.
