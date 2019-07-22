use crate::logic::table::{OpenResult, Table};
use hrsw::Stopwatch;

pub enum GameLevel {
    Beginner,
    Intermediate,
    Expert,
}

pub struct Game {
    // TODO remove this pub
    pub table: Table,
    stopwatch: Stopwatch,
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
        Ok(Game {
            table,
            stopwatch: Stopwatch::new(),
        })
    }

    pub fn open(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
        self.stopwatch.start();
        match self.table.open_field(row, col)? {
            OpenResult::Ok => Ok(()),
            OpenResult::Boom | OpenResult::WINNER => {
                self.stopwatch.stop();
                println!("{}", self.stopwatch.elapsed().as_millis() as f32 / 1000.0);
                Ok(())
            }
        }
    }
}
