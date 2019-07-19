use crate::logic::table::{OpenResult, Table};

pub enum GameLevel {
    Beginner,
    Intermediate,
    Expert,
}

pub struct Game {
    // TODO remove this pub
    pub table: Table,
}

impl Game {
    pub fn new(level: GameLevel) -> Game {
        match level {
            GameLevel::Beginner => Game::new_custom(10, 10, 10).unwrap(),
            GameLevel::Intermediate => Game::new_custom(16, 16, 25).unwrap(),
            GameLevel::Expert => Game::new_custom(30, 16, 99).unwrap()
        }
    }

    pub fn new_custom(width: usize, height: usize, number_of_mines: usize) -> Result<Game, &'static str> {
        let table = Table::new(width, height, number_of_mines)?;
        Ok(Game{table})
    }

    pub fn open(&mut self, row: usize, col:usize) -> Result<(), &'static str> {
        // start timer here...
        match self.table.open_field(row, col)?{
            OpenResult::Ok => Ok(()),
            OpenResult::Boom => Ok(()),
            OpenResult::WINNER => Ok(()),
        }
    }
}

