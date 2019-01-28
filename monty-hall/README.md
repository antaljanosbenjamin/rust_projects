# Monty-Hall simulator
This program is a simple simulation of the [Monty-Hall problem](https://en.wikipedia.org/wiki/Monty_Hall_problem). 

## Usage
`cargo run num_of_doors num_of_games switch_choice`
where the meaning of the parameters are the following:
- `num_of_doors`: the number of doors in the simulated game.
- `num_of_games`: number of the simulated games.
- `switch_choice`: either `t` (true) or `f` (false) to determine whether the user switches his choice or not.

## Motivation
This is my first Rust application which is more complex than the [Guessing game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) in the Rust tutorial. I just heard about the Monty-Hall problem and I really would like to try out, so I created this little program to simulate millions of games in a few seconds.
