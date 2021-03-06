use super::basic_types::SizeType;
use super::field_info::FieldInfo;
use super::results::{FlagResult, OpenInfo, OpenResult};
use super::table::{BasicTable, Table};
use hrsw::Stopwatch;
use std::time::Duration;
use strum_macros::Display;

static GAME_IS_ALREADY_STOPPED_ERROR: &'static str = "Game is already stopped!";

#[repr(C)]
#[allow(dead_code)]
#[derive(Clone, Copy, Eq, PartialEq, Display, Debug)]
pub enum GameLevel {
    Beginner,
    Intermediate,
    Expert,
}

#[derive(Clone, Copy, Eq, PartialEq, Display, Debug)]
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
        height: SizeType,
        width: SizeType,
        number_of_mines: SizeType,
    ) -> Result<Game, &'static str> {
        let table = Box::new(BasicTable::new(height, width, number_of_mines)?);
        Ok(Game {
            table,
            stopwatch: Stopwatch::new(),
            state: GameState::NotStarted,
        })
    }

    #[allow(dead_code)]
    fn new_from_table(table: Box<dyn Table>) -> Game {
        Game {
            table,
            stopwatch: Stopwatch::new(),
            state: GameState::NotStarted,
        }
    }

    fn start_game_if_needed(&mut self) -> Result<(), &'static str> {
        match self.state {
            GameState::Started => Ok(()),
            GameState::NotStarted => {
                self.stopwatch.start();
                self.state = GameState::Started;
                Ok(())
            }
            GameState::Stopped { win: _ } => Err(GAME_IS_ALREADY_STOPPED_ERROR),
        }
    }

    fn stop_game(&mut self, win: bool) {
        self.stopwatch.stop();
        self.state = GameState::Stopped { win };
    }

    fn execute_open(
        &mut self,
        open_func: impl Fn(&mut dyn Table) -> Result<OpenInfo, &'static str>,
    ) -> Result<OpenInfo, &'static str> {
        self.start_game_if_needed()?;

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
    }

    pub fn open(&mut self, row: SizeType, col: SizeType) -> Result<OpenInfo, &'static str> {
        self.execute_open(|table| table.open_field(row, col))
    }

    pub fn open_neighbors(
        &mut self,
        row: SizeType,
        col: SizeType,
    ) -> Result<OpenInfo, &'static str> {
        self.execute_open(|table| table.open_neighbors(row, col))
    }

    pub fn toggle_flag(
        &mut self,
        row: SizeType,
        col: SizeType,
    ) -> Result<FlagResult, &'static str> {
        self.start_game_if_needed()?;

        self.table.toggle_flag(row, col)
    }

    pub fn width(&self) -> SizeType {
        self.table.width()
    }

    pub fn height(&self) -> SizeType {
        self.table.height()
    }

    pub fn get_elapsed(&self) -> Duration {
        self.stopwatch.elapsed()
    }

    pub fn get_field_info(&self, row: SizeType, col: SizeType) -> Result<FieldInfo, &'static str> {
        self.table.get_field_info(row, col)
    }
}

#[cfg(test)]
mod test {
    use super::super::field_info::{FieldState, FieldType};
    use super::super::table::MockTable;
    use super::*;
    use mockall::predicate::eq;
    use mockall::Sequence;
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
            assert!(
                expected_value + absolute_tolerance > value,
                "{:.1} + {:.1} == {:.1} > {:.1}",
                expected_value / 1_000_000.0,
                absolute_tolerance / 1_000_000.0,
                (expected_value + absolute_tolerance) / 1_000_000.0,
                value / 1_000_000.0
            );
        } else {
            assert!(expected_value - absolute_tolerance > value);
            assert!(expected_value + absolute_tolerance < value);
        }
    }

    fn create_default_open_result(row: SizeType, col: SizeType) -> Result<OpenInfo, &'static str> {
        let mut newly_opened_fields = HashMap::new();
        newly_opened_fields.insert((row, col), crate::FieldType::Numbered(1));
        Ok(OpenInfo {
            result: OpenResult::Ok,
            newly_opened_fields,
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
        const HUGE_TOLERANCE_DUE_TO_TINY_SLEEP_TIME: f64 = 0.5;
        check_close_to(
            expected_first_checkpoint,
            elapsed_first_checkpoint,
            HUGE_TOLERANCE_DUE_TO_TINY_SLEEP_TIME,
        );

        thread::sleep(Duration::from_millis(SLEEPING_MILLIS));

        let expected_game_time = end_time - start_time;
        let actual_game_time = game.get_elapsed();

        const BIG_TOLERANCE_DUE_TO_TINY_SLEEP_TIME: f64 = 0.3;
        check_close_to(
            expected_game_time.as_nanos() as f64,
            actual_game_time.as_nanos() as f64,
            BIG_TOLERANCE_DUE_TO_TINY_SLEEP_TIME,
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

    #[test]
    fn test_get_field_info() {
        let mut mock_table = MockTable::new();
        let expected_field_info_1 = FieldInfo {
            state: FieldState::Flagged,
            field_type: FieldType::Numbered(4),
        };
        let expected_field_info_2 = FieldInfo {
            state: FieldState::Closed,
            field_type: FieldType::Mine,
        };
        let expected_field_info_3 = FieldInfo {
            state: FieldState::Opened,
            field_type: FieldType::Empty,
        };
        let row_1 = 0;
        let col_1 = 1;
        let row_2 = 2;
        let col_2 = 3;
        let row_3 = 5;
        let col_3 = 4;
        let mut seq = Sequence::new();

        mock_table
            .expect_get_field_info()
            .with(eq(row_1), eq(col_1))
            .times(1)
            .in_sequence(&mut seq)
            .return_const(Ok(expected_field_info_1.clone()));

        mock_table
            .expect_open_field()
            .with(eq(row_1), eq(col_1))
            .times(1)
            .in_sequence(&mut seq)
            .returning(create_default_open_result);

        mock_table
            .expect_get_field_info()
            .with(eq(row_2), eq(col_2))
            .times(1)
            .in_sequence(&mut seq)
            .return_const(Ok(expected_field_info_2.clone()));

        mock_table
            .expect_open_field()
            .with(eq(row_2), eq(col_2))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_, _| {
                Ok(OpenInfo {
                    result: OpenResult::Boom,
                    newly_opened_fields: HashMap::new(),
                })
            });

        mock_table
            .expect_get_field_info()
            .with(eq(row_3), eq(col_3))
            .times(1)
            .in_sequence(&mut seq)
            .return_const(Ok(expected_field_info_3.clone()));

        let mut game = Game::new_from_table(Box::new(mock_table));

        let mut field_info = game.get_field_info(row_1, col_1).unwrap();
        assert_eq!(expected_field_info_1, field_info);

        let _ = game.open(row_1, col_1);

        field_info = game.get_field_info(row_2, col_2).unwrap();
        assert_eq!(expected_field_info_2, field_info);

        let _ = game.open(row_2, col_2);

        field_info = game.get_field_info(row_3, col_3).unwrap();
        assert_eq!(expected_field_info_3, field_info);
    }
}
