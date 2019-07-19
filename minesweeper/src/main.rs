mod logic;
use logic::game::{Game, GameLevel};

fn main() {
    let mut g = Game::new(GameLevel::Expert);
    g.open(1, 1).unwrap();
    g.open(9, 9).unwrap();
    g.table.print();
    //d.print();
}
