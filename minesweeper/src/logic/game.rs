use crate::logic::table::{OpenResult, Table};
use hrsw::Stopwatch;

pub enum GameLevel {
    Beginner,
    Intermediate,
    Expert,
}

#[derive(PartialEq)]
pub enum GameState {
    NotStarted,
    Started,
    Stopped { win: bool },
}

pub struct Game {
    // TODO remove this pub
    pub table: Table,
    stopwatch: Stopwatch,
    state: GameState,
}

impl Game {
    pub fn new(level: GameLevel) -> Game {
        match level {
            GameLevel::Beginner => Game::new_custom(10, 10, 10).unwrap(),
            GameLevel::Intermediate => Game::new_custom(16, 16, 25).unwrap(),
            GameLevel::Expert => Game::new_custom(30, 16, 99).unwrap(),
        }
    }

    pub fn new_custom(
        width: usize,
        height: usize,
        number_of_mines: usize,
    ) -> Result<Game, &'static str> {
        let table = Table::new(width, height, number_of_mines)?;
        Ok(Game {
            table,
            stopwatch: Stopwatch::new(),
            state: GameState::NotStarted,
        })
    }

    fn start_game_if_needed(&mut self) {
        if self.state != GameState::NotStarted {
            return;
        }

        self.stopwatch.start();
        self.state = GameState::Started;
    }

    pub fn open(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
        self.start_game_if_needed();
        match self.table.open_field(row, col)? {
            OpenResult::Ok => Ok(()),
            OpenResult::WINNER => {
                self.state = GameState::Stopped { win: true };
                self.stopwatch.stop();
                Ok(())
            }
            OpenResult::Boom => {
                self.state = GameState::Stopped { win: false };
                self.stopwatch.stop();
                Ok(())
            }
        }
    }

    pub fn toggle_flag(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
        self.start_game_if_needed();
        self.table.toggle_flag(row, col)?;
        Ok(())
    }
}
