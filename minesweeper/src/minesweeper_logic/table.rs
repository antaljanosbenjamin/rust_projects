use super::field_type::FieldType;
use super::results::{FieldFlagResult, OpenInfo, OpenResult};
use indexmap::IndexSet;
use std::collections::{HashMap, HashSet};
use strum_macros::Display;

static INVALID_VALUE_ERROR: &'static str = "Invalid value!";
static INVALID_INDEX_ERROR: &'static str = "Invalid index!";
static INVALID_SIZE_ERROR: &'static str = "Invalid size!";
static TOO_MUCH_MINES_ERROR: &'static str = "Too much mines!";
static TOO_FEW_MINES_ERROR: &'static str = "Too few mines!";
static MINE_DOES_NOT_HAVE_VALUE_ERROR: &'static str = "Mine does not have value!";
static OPENED_FIELD_CAN_NOT_BE_UPDATED_ERROR: &'static str = "An opened field can not be updated!";

#[derive(Eq, PartialEq, Display, Debug, Clone, Copy)]
enum FieldState {
    Closed,
    Opened,
    Flagged,
}

impl FieldState {
    fn is_opened(&self) -> bool {
        self == &FieldState::Opened
    }

    fn is_flagged(&self) -> bool {
        self == &FieldState::Flagged
    }
}

trait Field {
    fn get_field_state(&self) -> FieldState;
    fn get_field_type(&self) -> FieldType;
    fn get_char_repr(&self) -> char;
}

#[derive(Eq, PartialEq, Display, Debug)]
enum FieldOpenResult {
    AlreadyOpened,
    SimpleOpen,
    MultiOpen,
    Boom,
    IsFlagged,
}

#[derive(Debug)]
struct FieldInner {
    field_type: FieldType,
    state: FieldState,
}

impl FieldInner {
    fn is_valid_value(value: u8) -> bool {
        value > 0 && value < 9
    }

    fn new_with_field_type(field_type: FieldType) -> FieldInner {
        FieldInner {
            field_type,
            state: FieldState::Closed,
        }
    }

    fn new_mine() -> FieldInner {
        FieldInner::new_with_field_type(FieldType::Mine)
    }

    fn new_empty() -> FieldInner {
        FieldInner::new_with_field_type(FieldType::Empty)
    }

    fn new_numbered(value: u8) -> Result<FieldInner, &'static str> {
        if !FieldInner::is_valid_value(value) {
            Err(INVALID_VALUE_ERROR)
        } else {
            Ok(FieldInner::new_with_field_type(FieldType::Numbered(value)))
        }
    }

    fn update_type_to_mine(&mut self) -> Result<(), &'static str> {
        if self.state == FieldState::Opened {
            Err(OPENED_FIELD_CAN_NOT_BE_UPDATED_ERROR)
        } else {
            self.field_type = FieldType::Mine;
            Ok(())
        }
    }

    fn update_type_to_empty(&mut self) -> Result<(), &'static str> {
        if self.state == FieldState::Opened {
            Err(OPENED_FIELD_CAN_NOT_BE_UPDATED_ERROR)
        } else {
            self.field_type = FieldType::Empty;
            Ok(())
        }
    }

    fn update_type_with_value(&mut self, value: u8) -> Result<(), &'static str> {
        if self.state == FieldState::Opened {
            Err(OPENED_FIELD_CAN_NOT_BE_UPDATED_ERROR)
        } else if !FieldInner::is_valid_value(value) {
            Err(INVALID_VALUE_ERROR)
        } else {
            self.field_type = FieldType::Numbered(value);
            Ok(())
        }
    }

    fn get_open_result_inner(&self) -> FieldOpenResult {
        match self.field_type {
            FieldType::Empty => FieldOpenResult::MultiOpen,
            FieldType::Numbered(_) => FieldOpenResult::SimpleOpen,
            FieldType::Mine => FieldOpenResult::Boom,
        }
    }

    fn open(&mut self) -> FieldOpenResult {
        if self.get_field_state().is_flagged() {
            FieldOpenResult::IsFlagged
        } else if self.get_field_state().is_opened() {
            FieldOpenResult::AlreadyOpened
        } else {
            self.state = FieldState::Opened;
            self.get_open_result_inner()
        }
    }

    fn toggle_flag(&mut self) -> FieldFlagResult {
        if self.state.is_flagged() {
            self.state = FieldState::Closed;
            FieldFlagResult::FlagRemoved
        } else {
            if !self.get_field_state().is_opened() {
                self.state = FieldState::Flagged;
                FieldFlagResult::Flagged
            } else {
                FieldFlagResult::AlreadyOpened
            }
        }
    }
}

impl Field for FieldInner {
    fn get_field_state(&self) -> FieldState {
        self.state
    }

    fn get_field_type(&self) -> FieldType {
        self.field_type
    }

    fn get_char_repr(&self) -> char {
        if self.state.is_flagged() {
            'H'
        } else if !self.state.is_opened() {
            'O'
        } else {
            self.field_type.get_char_repr()
        }
    }
}

struct FieldVisiter {
    height: usize,
    width: usize,
    fields_to_visit: IndexSet<(usize, usize)>,
    visited_fields: HashSet<(usize, usize)>,
}

impl FieldVisiter {
    fn new(
        height: usize,
        width: usize,
        row: usize,
        col: usize,
    ) -> Result<FieldVisiter, &'static str> {
        if row >= height || col >= width {
            Err(INVALID_INDEX_ERROR)
        } else {
            let mut fields_to_visit = IndexSet::new();
            fields_to_visit.insert((row, col));
            Ok(FieldVisiter {
                height,
                width,
                fields_to_visit,
                visited_fields: HashSet::new(),
            })
        }
    }

    fn extend_with_unvisited_neighbors(&mut self, row: usize, col: usize) {
        let fields_to_extend: HashSet<(usize, usize)> =
            get_neighbor_fields(self.height, self.width, row, col)
                .difference(&self.visited_fields)
                .cloned()
                .collect();
        self.fields_to_visit.extend(fields_to_extend);
    }

    // This can be rewritten as an Iterator?
    fn next(&mut self) -> Option<(usize, usize)> {
        if !self.fields_to_visit.is_empty() {
            let result = self.fields_to_visit.pop().unwrap();
            self.visited_fields.insert(result);
            Some(result)
        } else {
            None
        }
    }
}

const NEIGHBOR_OFFSETS: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn generate_mine_locations(
    height: usize,
    width: usize,
    number_of_mines: usize,
) -> Result<HashSet<(usize, usize)>, &'static str> {
    let max_number_of_mines = (width * height - 1) as usize;
    let min_number_of_mines = 1;

    if max_number_of_mines < number_of_mines {
        return Err(TOO_MUCH_MINES_ERROR);
    }
    if min_number_of_mines > number_of_mines {
        return Err(TOO_FEW_MINES_ERROR);
    }

    let mut mine_locations = HashSet::new();
    while (mine_locations.len() as usize) < number_of_mines {
        mine_locations.insert((
            rand::random::<usize>() % width,
            rand::random::<usize>() % height,
        ));
    }
    Ok(mine_locations)
}

fn get_neighbor_fields(
    height: usize,
    width: usize,
    row: usize,
    col: usize,
) -> HashSet<(usize, usize)> {
    fn add(u: usize, i: i8) -> Option<usize> {
        if i.is_negative() {
            u.checked_sub(i.wrapping_abs() as u8 as usize)
        } else {
            u.checked_add(i as usize)
        }
    };

    let mut neighbors = HashSet::new();

    for offset in NEIGHBOR_OFFSETS.iter() {
        match (add(row, offset.0), add(col, offset.1)) {
            (Some(r), Some(c)) if r < height && c < width => {
                neighbors.insert((r, c));
            }
            _ => (),
        }
    }

    neighbors
}

fn get_field_value(
    height: usize,
    width: usize,
    row: usize,
    col: usize,
    mine_locations: &HashSet<(usize, usize)>,
) -> Result<u8, &'static str> {
    if mine_locations.contains(&(row, col)) {
        return Err(MINE_DOES_NOT_HAVE_VALUE_ERROR);
    }
    let mut field_value: u8 = 0;

    for (r, c) in get_neighbor_fields(height, width, row, col) {
        if mine_locations.contains(&(r, c)) {
            field_value = field_value + 1;
        }
    }

    Ok(field_value)
}

fn generate_fields(
    height: usize,
    width: usize,
    mine_locations: &HashSet<(usize, usize)>,
) -> Result<Vec<Vec<FieldInner>>, &'static str> {
    let mut fields = Vec::new();

    for r in 0..height {
        let mut row = Vec::new();
        for c in 0..width {
            if mine_locations.contains(&(r, c)) {
                row.push(FieldInner::new_mine());
            } else {
                let value = get_field_value(height, width, r, c, mine_locations)?;
                if value == 0 {
                    row.push(FieldInner::new_empty());
                } else {
                    row.push(FieldInner::new_numbered(value).unwrap())
                }
            }
        }
        fields.push(row);
    }
    Ok(fields)
}

pub struct Table {
    height: usize,
    width: usize,
    mine_locations: HashSet<(usize, usize)>,
    number_of_opened_fields: usize,
    fields: Vec<Vec<FieldInner>>,
}

impl Table {
    fn with_custom_mines(
        height: usize,
        width: usize,
        mine_locations: HashSet<(usize, usize)>,
    ) -> Result<Table, &'static str> {
        if width.checked_mul(height).is_none() {
            return Err(INVALID_SIZE_ERROR);
        }
        let fields = generate_fields(height, width, &mine_locations)?;
        Ok(Table {
            height,
            width,
            mine_locations,
            number_of_opened_fields: 0,
            fields,
        })
    }

    pub fn new(height: usize, width: usize, number_of_mines: usize) -> Result<Table, &'static str> {
        let mine_locations = generate_mine_locations(height, width, number_of_mines)?;
        Table::with_custom_mines(height, width, mine_locations)
    }

    fn get_neighbor_fields(&self, row: usize, col: usize) -> HashSet<(usize, usize)> {
        get_neighbor_fields(self.height, self.width, row, col)
    }

    fn get_field_value(&self, row: usize, col: usize) -> Result<u8, &'static str> {
        get_field_value(self.height, self.width, row, col, &self.mine_locations)
    }

    fn all_fields_are_open(&self) -> bool {
        self.width * self.height == self.mine_locations.len() + self.number_of_opened_fields
    }

    fn move_mine(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
        if self.fields[row][col].field_type.is_mine() {
            let mut new_place = (0, 0);
            let mut visiter = FieldVisiter::new(self.height, self.width, row, col)?;
            while let Some((r, c)) = visiter.next() {
                if !self.fields[r][c].field_type.is_mine() {
                    new_place = (r, c);
                    break;
                }
                visiter.extend_with_unvisited_neighbors(r, c);
            }

            self.fields[new_place.0][new_place.1]
                .update_type_to_mine()
                .unwrap();
            self.fields[row][col].update_type_to_empty().unwrap();
            self.mine_locations.remove(&(row, col));
            self.mine_locations.insert(new_place);
            let mut fields_to_recalculate = HashSet::new();
            fields_to_recalculate.extend(self.get_neighbor_fields(row, col).into_iter());
            fields_to_recalculate.extend(
                self.get_neighbor_fields(new_place.0, new_place.1)
                    .into_iter(),
            );
            fields_to_recalculate.insert((row, col));
            for (r, c) in fields_to_recalculate {
                if !self.fields[r][c].field_type.is_mine() {
                    let field_value = self.get_field_value(r, c).unwrap();
                    match field_value {
                        0 => self.fields[r][c].update_type_to_empty().unwrap(),
                        _ => self.fields[r][c]
                            .update_type_with_value(field_value)
                            .unwrap(),
                    };
                }
            }
        }

        Ok(())
    }

    fn construct_boom_result(&self) -> OpenInfo {
        let mut boom_result = OpenInfo {
            result: OpenResult::Boom,
            field_infos: HashMap::new(),
        };
        for row in 0..self.height {
            for column in 0..self.width {
                boom_result
                    .field_infos
                    .insert((row, column), self.fields[row][column].get_field_type());
            }
        }
        boom_result
    }

    pub fn open_field(&mut self, row: usize, col: usize) -> Result<OpenInfo, &'static str> {
        if row >= self.height || col >= self.width {
            return Err(INVALID_INDEX_ERROR);
        }
        if self.fields[row][col].get_field_state().is_flagged() {
            return Ok(OpenInfo {
                result: OpenResult::IsFlagged,
                field_infos: HashMap::new(),
            });
        }

        if self.number_of_opened_fields == 0 && self.fields[row][col].field_type.is_mine() {
            self.move_mine(row, col)?;
        }

        let mut visiter = FieldVisiter::new(self.height, self.width, row, col)?;
        let mut field_infos = HashMap::new();

        while let Some((r, c)) = visiter.next() {
            match self.fields[r][c].open() {
                FieldOpenResult::MultiOpen => {
                    self.number_of_opened_fields = self.number_of_opened_fields + 1;
                    visiter.extend_with_unvisited_neighbors(r, c);
                }
                FieldOpenResult::SimpleOpen => {
                    self.number_of_opened_fields = self.number_of_opened_fields + 1;
                }
                FieldOpenResult::Boom => return Ok(self.construct_boom_result()),
                _ => continue,
            };

            field_infos.insert((r, c), self.fields[r][c].get_field_type());
        }

        if self.all_fields_are_open() {
            for mine_coords in self.mine_locations.iter() {
                field_infos.insert(
                    (mine_coords.0, mine_coords.1),
                    self.fields[mine_coords.0][mine_coords.1].get_field_type(),
                );
            }
            Ok(OpenInfo {
                result: OpenResult::WINNER,
                field_infos,
            })
        } else {
            Ok(OpenInfo {
                result: OpenResult::Ok,
                field_infos,
            })
        }
    }

    #[allow(dead_code)]
    pub fn toggle_flag(&mut self, row: usize, col: usize) -> Result<FieldFlagResult, &'static str> {
        if row >= self.height || col >= self.width {
            Err(INVALID_INDEX_ERROR)
        } else {
            Ok(self.fields[row][col].toggle_flag())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;

    struct TestInfo {
        height: usize,
        width: usize,
        table: RefCell<Table>,
        mine_locations: HashSet<(usize, usize)>,
        fields: Vec<Vec<FieldType>>,
    }

    // O O 1 M 1 0
    // O 1 2 2 1 0
    // 1 2 M 2 1 0
    // M 3 3 M 1 0
    // 2 M 2 1 1 0
    fn create_test_info_5x6() -> TestInfo {
        let mut mine_locations = HashSet::new();
        mine_locations.insert((0, 3));
        mine_locations.insert((3, 0));
        mine_locations.insert((3, 3));
        mine_locations.insert((2, 2));
        mine_locations.insert((4, 1));
        let fields = vec![
            vec![
                FieldType::Empty,
                FieldType::Empty,
                FieldType::Numbered(1),
                FieldType::Mine,
                FieldType::Numbered(1),
                FieldType::Empty,
            ],
            vec![
                FieldType::Empty,
                FieldType::Numbered(1),
                FieldType::Numbered(2),
                FieldType::Numbered(2),
                FieldType::Numbered(1),
                FieldType::Empty,
            ],
            vec![
                FieldType::Numbered(1),
                FieldType::Numbered(2),
                FieldType::Mine,
                FieldType::Numbered(2),
                FieldType::Numbered(1),
                FieldType::Empty,
            ],
            vec![
                FieldType::Mine,
                FieldType::Numbered(3),
                FieldType::Numbered(3),
                FieldType::Mine,
                FieldType::Numbered(1),
                FieldType::Empty,
            ],
            vec![
                FieldType::Numbered(2),
                FieldType::Mine,
                FieldType::Numbered(2),
                FieldType::Numbered(1),
                FieldType::Numbered(1),
                FieldType::Empty,
            ],
        ];
        let width = 6;
        let height = 5;
        TestInfo {
            height,
            width,
            table: RefCell::new(
                Table::with_custom_mines(height, width, mine_locations.clone()).unwrap(),
            ),
            mine_locations,
            fields,
        }
    }

    fn check_invalid_size_error(result: Result<Table, &'static str>) {
        assert!(result.is_err());
        assert_eq!(INVALID_SIZE_ERROR, result.err().unwrap());
    }

    fn check_invalid_value_error(result: Result<FieldInner, &'static str>) {
        assert!(result.is_err());
        assert_eq!(INVALID_VALUE_ERROR, result.err().unwrap());
    }

    fn check_opened_field_cannot_be_updated_error(result: Result<(), &'static str>) {
        assert!(result.is_err());
        assert_eq!(OPENED_FIELD_CAN_NOT_BE_UPDATED_ERROR, result.err().unwrap());
    }

    #[test]
    fn new_field_with_invalid_number() {
        check_invalid_value_error(FieldInner::new_numbered(0));
        check_invalid_value_error(FieldInner::new_numbered(9));
    }

    #[test]
    fn update_type_to_empty() {
        let mut field = FieldInner::new_numbered(1).unwrap();
        assert_eq!(FieldType::Numbered(1), field.get_field_type());
        assert_eq!(Ok(()), field.update_type_to_empty());
        assert_eq!(FieldType::Empty, field.get_field_type());
        assert_eq!(field.open(), FieldOpenResult::MultiOpen);
        check_opened_field_cannot_be_updated_error(field.update_type_to_empty());
    }

    #[test]
    fn update_type_to_numbered() {
        let mut field = FieldInner::new_numbered(2).unwrap();
        assert_eq!(FieldType::Numbered(2), field.get_field_type());
        assert_eq!(Ok(()), field.update_type_with_value(3));
        assert_eq!(FieldType::Numbered(3), field.get_field_type());
        assert_eq!(field.open(), FieldOpenResult::SimpleOpen);
        check_opened_field_cannot_be_updated_error(field.update_type_with_value(4));
    }

    #[test]
    fn update_type_to_mine() {
        let mut field = FieldInner::new_numbered(5).unwrap();
        assert_eq!(FieldType::Numbered(5), field.get_field_type());
        assert_eq!(Ok(()), field.update_type_to_mine());
        assert_eq!(FieldType::Mine, field.get_field_type());
        assert_eq!(field.open(), FieldOpenResult::Boom);
        check_opened_field_cannot_be_updated_error(field.update_type_with_value(8));
    }

    #[test]
    fn update_type_from_mine() {
        let mut field = FieldInner::new_mine();
        assert_eq!(FieldType::Mine, field.get_field_type());
        assert_eq!(Ok(()), field.update_type_to_empty());
        assert_eq!(FieldType::Empty, field.get_field_type());
        field = FieldInner::new_mine();
        assert_eq!(FieldType::Mine, field.get_field_type());
        assert_eq!(Ok(()), field.update_type_with_value(3));
        assert_eq!(FieldType::Numbered(3), field.get_field_type());
    }

    #[test]
    fn update_type_from_emtpy() {
        let mut field = FieldInner::new_empty();
        assert_eq!(FieldType::Empty, field.get_field_type());
        assert_eq!(Ok(()), field.update_type_to_mine());
        assert_eq!(FieldType::Mine, field.get_field_type());
        field = FieldInner::new_empty();
        assert_eq!(FieldType::Empty, field.get_field_type());
        assert_eq!(Ok(()), field.update_type_with_value(8));
        assert_eq!(FieldType::Numbered(8), field.get_field_type());
    }

    #[test]
    fn get_value_of_mine() {
        let mut mine_locations = HashSet::new();
        mine_locations.insert((1, 1));
        let result = get_field_value(10, 10, 1, 1, &mine_locations);
        assert!(result.is_err());
        assert_eq!(MINE_DOES_NOT_HAVE_VALUE_ERROR, result.err().unwrap());
    }

    #[test]
    // 1 2 3 2 3 M
    // 2 M M M 5 M
    // 3 M 8 M 6 M
    // 2 M M M 5 M
    // 1 2 3 2 4 M
    // 0 0 0 0 2 M
    fn get_value_general_test() {
        let width = 6;
        let height = 6;
        let mut mine_locations = HashSet::new();
        mine_locations.insert((0, 5));
        mine_locations.insert((1, 1));
        mine_locations.insert((1, 2));
        mine_locations.insert((1, 3));
        mine_locations.insert((1, 5));
        mine_locations.insert((2, 1));
        mine_locations.insert((2, 3));
        mine_locations.insert((2, 5));
        mine_locations.insert((3, 1));
        mine_locations.insert((3, 2));
        mine_locations.insert((3, 3));
        mine_locations.insert((3, 5));
        mine_locations.insert((4, 5));
        mine_locations.insert((5, 5));
        let mut expected_values = HashSet::new();
        expected_values.insert((0, 0, 1));
        expected_values.insert((0, 1, 2));
        expected_values.insert((0, 2, 3));
        expected_values.insert((0, 3, 2));
        expected_values.insert((0, 4, 3));
        expected_values.insert((1, 0, 2));
        expected_values.insert((1, 4, 5));
        expected_values.insert((2, 0, 3));
        expected_values.insert((2, 2, 8));
        expected_values.insert((2, 4, 6));
        expected_values.insert((3, 0, 2));
        expected_values.insert((3, 4, 5));
        expected_values.insert((4, 0, 1));
        expected_values.insert((4, 1, 2));
        expected_values.insert((4, 2, 3));
        expected_values.insert((4, 3, 2));
        expected_values.insert((4, 4, 4));
        expected_values.insert((5, 0, 0));
        expected_values.insert((5, 1, 0));
        expected_values.insert((5, 2, 0));
        expected_values.insert((5, 3, 0));
        expected_values.insert((5, 4, 2));
        for (row, col, expected_value) in expected_values.iter() {
            assert_eq!(
                expected_value,
                &get_field_value(height, width, *row, *col, &mine_locations).unwrap()
            )
        }
    }

    #[test]
    fn create_game_with_invalid_sizes() {
        check_invalid_size_error(Table::with_custom_mines(
            usize::MAX,
            usize::MAX,
            HashSet::new(),
        ));
        check_invalid_size_error(Table::with_custom_mines(
            usize::MAX / 2 + 1,
            2,
            HashSet::new(),
        ));
        check_invalid_size_error(Table::with_custom_mines(
            usize::MAX / 5 + 4,
            5,
            HashSet::new(),
        ));
    }

    #[test]
    fn field_visiter() {
        let table = Table::new(10, 15, 10).unwrap();
        let base_row = 5;
        let base_col = 10;
        let mut visiter = FieldVisiter::new(table.height, table.width, base_row, base_col).unwrap();
        let mut expected_fields_to_visit = IndexSet::new();
        expected_fields_to_visit.insert((base_row, base_col));
        assert_eq!(expected_fields_to_visit, visiter.fields_to_visit);
        visiter.extend_with_unvisited_neighbors(base_row, base_col);
        for row in base_row - 1..base_row + 2 {
            for col in base_col - 1..base_col + 2 {
                expected_fields_to_visit.insert((row, col));
            }
        }
        assert_eq!(expected_fields_to_visit, visiter.fields_to_visit);

        let mut item = visiter.next().unwrap();
        expected_fields_to_visit.remove(&item);
        item = visiter.next().unwrap();
        expected_fields_to_visit.remove(&item);
        item = visiter.next().unwrap();
        expected_fields_to_visit.remove(&item);
        assert_eq!(expected_fields_to_visit, visiter.fields_to_visit);
        visiter.extend_with_unvisited_neighbors(base_row, base_col);
        assert_eq!(expected_fields_to_visit, visiter.fields_to_visit);
        while let Some(item) = visiter.next() {
            expected_fields_to_visit.remove(&item);
            visiter.extend_with_unvisited_neighbors(base_row, base_col);
            assert_eq!(expected_fields_to_visit, visiter.fields_to_visit);
        }
        assert!(visiter.fields_to_visit.is_empty());
    }

    #[test]
    fn open_everything() {
        let test_info = create_test_info_5x6();
        let mut table = test_info.table.borrow_mut();
        for row in 0..test_info.height {
            for col in 0..test_info.width {
                let open_result = table.open_field(row, col).unwrap();
                let is_mine = test_info.mine_locations.contains(&(row, col));
                assert_eq!(is_mine, (open_result.result == OpenResult::Boom));
            }
        }
        // Everything is opened, so no Boom and no Ok
        for row in 0..test_info.height {
            for col in 0..test_info.width {
                let open_result = table.open_field(row, col).unwrap();
                assert_eq!(OpenResult::WINNER, open_result.result);
            }
        }
    }

    #[test]
    fn win() {
        let test_info = create_test_info_5x6();
        let mut table = test_info.table.borrow_mut();
        for row in 0..test_info.height {
            for col in 0..test_info.width - 1 {
                if !test_info.mine_locations.contains(&(row, col)) {
                    let open_result = table.open_field(row, col).unwrap();
                    assert_eq!(OpenResult::Ok, open_result.result);
                }
            }
        }
        assert!(table.open_field(0, 5).unwrap().result == OpenResult::WINNER);
    }

    #[test]
    fn open_mine_first() {
        let test_info = create_test_info_5x6();
        let mut table = test_info.table.borrow_mut();

        let mut iter = test_info.mine_locations.iter();
        let first_mine_location = iter.next().unwrap();
        assert_eq!(
            OpenResult::Ok,
            table
                .open_field(first_mine_location.0, first_mine_location.1)
                .unwrap()
                .result
        );
        let second_mine_location = iter.next().unwrap();
        assert_eq!(
            OpenResult::Boom,
            table
                .open_field(second_mine_location.0, second_mine_location.1)
                .unwrap()
                .result
        );
    }

    #[test]
    fn open_mine_second() {
        let test_info = create_test_info_5x6();
        let mut table = test_info.table.borrow_mut();
        assert_eq!(OpenResult::Ok, table.open_field(3, 2).unwrap().result);
        let first_mine_location = test_info.mine_locations.iter().next().unwrap();
        assert_eq!(
            OpenResult::Boom,
            table
                .open_field(first_mine_location.0, first_mine_location.1)
                .unwrap()
                .result
        );
    }

    #[test]
    fn open_bubble() {
        let test_info = create_test_info_5x6();
        let mut table = test_info.table.borrow_mut();
        let open_result = table.open_field(1, 0).unwrap();
        assert_eq!(OpenResult::Ok, open_result.result);
        assert_eq!(8, open_result.field_infos.len());
        for row in 0..3 {
            for col in 0..3 {
                if row == 2 && col == 2 {
                    continue;
                }
                assert_eq!(
                    Some(&test_info.fields[row][col]),
                    open_result.field_infos.get(&(row, col))
                );
            }
        }
    }

    #[test]
    fn flag_and_unflag() {
        let test_info = create_test_info_5x6();
        let mut table = test_info.table.borrow_mut();
        let flag_result = table.toggle_flag(0, 1).unwrap();
        assert_eq!(FieldFlagResult::Flagged, flag_result);
        let unflag_result = table.toggle_flag(0, 1).unwrap();
        assert_eq!(FieldFlagResult::FlagRemoved, unflag_result);
    }

    #[test]
    fn open_flagged() {
        let test_info = create_test_info_5x6();
        let mut table = test_info.table.borrow_mut();
        let toggle_result = table.toggle_flag(1, 1).unwrap();
        assert_eq!(FieldFlagResult::Flagged, toggle_result);
        let open_result = table.open_field(1, 1).unwrap();
        assert_eq!(OpenResult::IsFlagged, open_result.result);
        assert_eq!(0, open_result.field_infos.len());
    }

    #[test]
    fn flagged_bubble_is_not_opened() {
        let test_info = create_test_info_5x6();
        let mut table = test_info.table.borrow_mut();
        let toggle_result = table.toggle_flag(0, 1).unwrap();
        assert_eq!(FieldFlagResult::Flagged, toggle_result);
        let open_result = table.open_field(1, 0).unwrap();
        assert_eq!(OpenResult::Ok, open_result.result);
        let field_infos = &open_result.field_infos;
        assert_eq!(5, field_infos.len());

        assert_eq!(Some(&FieldType::Empty), field_infos.get(&(0, 0)));

        for row in 1..3 {
            for column in 0..2 {
                assert_eq!(
                    Some(&test_info.fields[row][column]),
                    field_infos.get(&(row, column))
                );
            }
        }
    }

    #[test]
    fn first_open_is_always_successful() {
        let table_size = 30;
        for test_index in 1..10 {
            let mut table =
                Table::new(table_size, table_size, table_size * table_size - 1).unwrap();
            let open_info = table.open_field(test_index, test_index * 2).unwrap();
            assert_eq!(OpenResult::WINNER, open_info.result);
        }
    }

    // TODO Write test to full game
}
