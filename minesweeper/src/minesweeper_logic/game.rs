use super::results::{FieldFlagResult, OpenInfo, OpenResult};
use super::table::Table;
use hrsw::Stopwatch;
use strum_macros::Display;

static GAME_IS_ALREADY_STOPPED_ERROR: &'static str = "Invalid value!";

#[repr(C)]
#[allow(dead_code)]
#[derive(Eq, PartialEq, Display, Debug)]
pub enum GameLevel {
    Beginner,
    Intermediate,
    Expert,
}

#[derive(Eq, PartialEq, Display, Debug)]
enum GameState {
    NotStarted,
    Started,
    Stopped { win: bool },
}

pub struct Game {
    width: usize,
    height: usize,
    table: Table,
    stopwatch: Stopwatch,
    state: GameState,
}

impl Game {
    pub fn new(level: GameLevel) -> Game {
        match level {
            GameLevel::Beginner => Game::new_custom(10, 10, 10).unwrap(),
            GameLevel::Intermediate => Game::new_custom(16, 16, 25).unwrap(),
            GameLevel::Expert => Game::new_custom(16, 30, 99).unwrap(),
        }
    }

    pub fn new_custom(
        height: usize,
        width: usize,
        number_of_mines: usize,
    ) -> Result<Game, &'static str> {
        let table = Table::new(width, height, number_of_mines)?;
        Ok(Game {
            width,
            height,
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

    fn stop_game(&mut self, win: bool) {
        self.stopwatch.stop();
        self.state = GameState::Stopped { win };
    }

    fn is_running(&self) -> bool {
        self.state == GameState::Started
    }

    pub fn open(&mut self, row: usize, col: usize) -> Result<OpenInfo, &'static str> {
        self.start_game_if_needed();

        if self.is_running() {
            let open_info = self.table.open_field(row, col)?;
            match open_info.result {
                OpenResult::WINNER => {
                    self.state = GameState::Stopped { win: true };
                    self.stop_game(true);
                }
                OpenResult::Boom => {
                    self.state = GameState::Stopped { win: false };
                    self.stop_game(false);
                }
                _ => (),
            };

            Ok(open_info)
        } else {
            Err(GAME_IS_ALREADY_STOPPED_ERROR)
        }
    }

    pub fn toggle_flag(&mut self, row: usize, col: usize) -> Result<FieldFlagResult, &'static str> {
        if self.is_running() {
            self.table.toggle_flag(row, col)
        } else {
            Err(GAME_IS_ALREADY_STOPPED_ERROR)
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_game_is_already_stopped_error<T>(result: Result<T, &'static str>) {
        assert!(result.is_err());
        assert_eq!(GAME_IS_ALREADY_STOPPED_ERROR, result.err().unwrap());
    }
    #[test]
    fn game_sizes() {
        let test_cases = vec![
            (Game::new(GameLevel::Beginner), 10, 10),
            (Game::new(GameLevel::Intermediate), 16, 16),
            (Game::new(GameLevel::Expert), 16, 30),
            (Game::new_custom(5, 10, 15).unwrap(), 5, 10),
        ];

        for (game, height, width) in test_cases.iter() {
            assert_eq!(game.get_height(), *height);
            assert_eq!(game.get_width(), *width);
        }
    }

    #[test]
    fn winning_stops_game() {
        let mut game = Game::new_custom(10, 10, 99).unwrap();
        let open_info = game.open(0, 0).unwrap();
        assert_eq!(OpenResult::WINNER, open_info.result);
        check_game_is_already_stopped_error(game.open(9, 9));
        check_game_is_already_stopped_error(game.toggle_flag(9, 9));
    }

    #[test]
    fn boom_stops_game() {
        let width = 60;
        let height = 10;
        let number_of_not_mine_fields = height;
        let mut game =
            Game::new_custom(height, width, height * width - number_of_not_mine_fields).unwrap();
        let mut is_boomed = false;
        while !is_boomed {
            for index in 0..height {
                match game.open(index, index) {
                    Ok(open_info) => match open_info.result {
                        OpenResult::Boom => {
                            is_boomed = true;
                            break;
                        }
                        OpenResult::Ok => (),
                        _ => break,
                    },
                    _ => break,
                }
            }
            if !is_boomed {
                game = Game::new_custom(height, width, height * width - number_of_not_mine_fields)
                    .unwrap();
            }
        }
        println!("is_boomed {}", is_boomed);
        check_game_is_already_stopped_error(game.open(1, 0));
        check_game_is_already_stopped_error(game.toggle_flag(1, 0));
    }

    #[test]
    fn toggle() {
        let width = 10;
        let height = 10;
        let number_of_mines = 10 * 10 - 2;
        let mut game = Game::new_custom(height, width, number_of_mines).unwrap();
        game.open(0, 0).unwrap();
        assert_eq!(
            FieldFlagResult::AlreadyOpened,
            game.toggle_flag(0, 0).unwrap()
        );
        assert_eq!(
            FieldFlagResult::Flagged,
            game.toggle_flag(height - 1, width - 1).unwrap()
        );
        assert_eq!(
            FieldFlagResult::FlagRemoved,
            game.toggle_flag(height - 1, width - 1).unwrap()
        );
    }
}
