use super::results::{FlagResult, OpenInfo, OpenResult};
use super::table::{BasicTable, Table};
use hrsw::Stopwatch;
use std::time::Duration;
use strum_macros::Display;

static GAME_IS_ALREADY_STOPPED_ERROR: &'static str = "Game is already stopped!";

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
    table: Box<dyn Table>,
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
        let table = Box::new(BasicTable::new(height, width, number_of_mines)?);
        Ok(Game {
            table,
            stopwatch: Stopwatch::new(),
            state: GameState::NotStarted,
        })
    }

    fn new_from_table(table: Box<dyn Table>) -> Game {
        Game {
            table,
            stopwatch: Stopwatch::new(),
            state: GameState::NotStarted,
        }
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

    fn execute_open(
        &mut self,
        open_func: impl Fn(&mut dyn Table) -> Result<OpenInfo, &'static str>,
    ) -> Result<OpenInfo, &'static str> {
        self.start_game_if_needed();

        if self.is_running() {
            let open_info = open_func(&mut *self.table)?;
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

    pub fn open(&mut self, row: usize, col: usize) -> Result<OpenInfo, &'static str> {
        self.execute_open(|table| table.open_field(row, col))
    }

    pub fn open_neighbors(&mut self, row: usize, col: usize) -> Result<OpenInfo, &'static str> {
        self.execute_open(|table| table.open_neighbors(row, col))
    }

    pub fn toggle_flag(&mut self, row: usize, col: usize) -> Result<FlagResult, &'static str> {
        self.start_game_if_needed();

        if self.is_running() {
            self.table.toggle_flag(row, col)
        } else {
            Err(GAME_IS_ALREADY_STOPPED_ERROR)
        }
    }

    pub fn width(&self) -> usize {
        self.table.width()
    }

    pub fn height(&self) -> usize {
        self.table.height()
    }

    pub fn get_elapsed(&self) -> Duration {
        self.stopwatch.elapsed()
    }
}

#[cfg(test)]
mod test {
    use super::super::table::MockTable;
    use super::*;
    use std::collections::HashMap;
    use std::thread;
    use std::time::Instant;

    fn check_game_is_already_stopped_error<T>(result: Result<T, &'static str>) {
        assert!(result.is_err());
        assert_eq!(GAME_IS_ALREADY_STOPPED_ERROR, result.err().unwrap());
    }

    fn check_close_to(expected_value: f64, value: f64, tolerance: f64) {
        let absolute_tolerance = expected_value * tolerance;
        if expected_value >= 0.0 {
            assert!(expected_value - absolute_tolerance < value);
            assert!(expected_value + absolute_tolerance > value);
        } else {
            assert!(expected_value - absolute_tolerance > value);
            assert!(expected_value + absolute_tolerance < value);
        }
    }

    fn create_default_open_result(row: usize, col: usize) -> Result<OpenInfo, &'static str> {
        let mut field_infos = HashMap::new();
        field_infos.insert((row, col), crate::FieldType::Numbered(1));
        Ok(OpenInfo {
            result: OpenResult::Ok,
            field_infos,
        })
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
            assert_eq!(game.height(), *height);
            assert_eq!(game.width(), *width);
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
        let game = Game::new_from_table(Box::new(MockTable::new()));
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
        assert_eq!(FlagResult::Flagged, game.toggle_flag(0, 0).unwrap());
        assert_eq!(FlagResult::FlagRemoved, game.toggle_flag(0, 0).unwrap());
        game.open(0, 0).unwrap();
        assert_eq!(FlagResult::AlreadyOpened, game.toggle_flag(0, 0).unwrap());
        assert_eq!(
            FlagResult::Flagged,
            game.toggle_flag(height - 1, width - 1).unwrap()
        );
        assert_eq!(
            FlagResult::FlagRemoved,
            game.toggle_flag(height - 1, width - 1).unwrap()
        );
    }

    #[test]
    fn elapsed() {
        const SLEEPING_MILLIS: u64 = 300;
        let mut mock_table = MockTable::new();
        mock_table
            .expect_open_field()
            .times(2)
            .returning(create_default_open_result);

        mock_table
            .expect_open_field()
            .times(1)
            .returning(|row, col| {
                let mut open_info = create_default_open_result(row, col)?;
                open_info.result = OpenResult::WINNER;
                Ok(open_info)
            });

        let mut game = Game::new_from_table(Box::new(mock_table));
        let first_elapsed = game.get_elapsed();
        assert_eq!(0, first_elapsed.as_nanos());
        let second_elapsed = game.get_elapsed();
        assert_eq!(0, second_elapsed.as_nanos());
        let _ = game.open(0, 0).unwrap();
        let start_time = Instant::now();

        thread::sleep(Duration::from_millis(SLEEPING_MILLIS));
        let elapsed_first_checkpoint = game.get_elapsed().as_nanos() as f64;

        let _ = game.open(1, 1).unwrap();
        thread::sleep(Duration::from_millis(SLEEPING_MILLIS));

        let open_info = game.open(3, 3).unwrap();
        let end_time = Instant::now();

        assert_eq!(OpenResult::WINNER, open_info.result);

        let expected_first_checkpoint = Duration::from_millis(SLEEPING_MILLIS).as_nanos() as f64;
        check_close_to(expected_first_checkpoint, elapsed_first_checkpoint, 0.1);

        thread::sleep(Duration::from_millis(SLEEPING_MILLIS));

        let expected_game_time = end_time - start_time;
        let actual_game_time = game.get_elapsed();

        check_close_to(
            expected_game_time.as_nanos() as f64,
            actual_game_time.as_nanos() as f64,
            0.1,
        );

        assert_eq!(actual_game_time, game.get_elapsed());
    }

    #[test]
    fn test_open_field_calls_right_function() {
        let mut mock_table = MockTable::new();
        mock_table
            .expect_open_field()
            .times(1)
            .returning(create_default_open_result);
        let mut game = Game::new_from_table(Box::new(mock_table));
        let _ = game.open(1, 1);
    }

    #[test]
    fn test_open_neighbors_calls_right_function() {
        let mut mock_table = MockTable::new();
        mock_table
            .expect_open_neighbors()
            .times(1)
            .returning(create_default_open_result);
        let mut game = Game::new_from_table(Box::new(mock_table));
        let _ = game.open_neighbors(1, 1);
    }
}
