use super::field_type::FieldType;
use super::results::{FieldFlagResult, OpenInfo, OpenResult};
use indexmap::IndexSet;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq)]
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

#[derive(PartialEq)]
enum FieldOpenResult {
    AlreadyOpened,
    SimpleOpen,
    MultiOpen,
    Boom,
    IsFlagged,
}

struct FieldInner {
    field_type: FieldType,
    state: FieldState,
}

static INVALID_VALUE_ERROR: &'static str = "Invalid value!";
static OPENED_FIELD_CAN_NOT_BE_UPDATED_PANIC: &'static str = "An opened field can not be updated!";

impl FieldInner {
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
        if value < 1 || value > 9 {
            Err(INVALID_VALUE_ERROR)
        } else {
            Ok(FieldInner::new_with_field_type(FieldType::Numbered(value)))
        }
    }

    fn update_type_to_mine(&mut self) {
        if self.state == FieldState::Opened {
            panic!(OPENED_FIELD_CAN_NOT_BE_UPDATED_PANIC);
        }
        self.field_type = FieldType::Mine;
    }

    fn update_type_to_empty(&mut self) {
        if self.state == FieldState::Opened {
            panic!(OPENED_FIELD_CAN_NOT_BE_UPDATED_PANIC);
        }
        self.field_type = FieldType::Empty;
    }

    fn update_type_with_value(&mut self, value: u8) -> Result<(), &'static str> {
        if self.state == FieldState::Opened {
            panic!(OPENED_FIELD_CAN_NOT_BE_UPDATED_PANIC);
        }
        if value < 1 || value > 9 {
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
    width: usize,
    height: usize,
    fields_to_visit: IndexSet<(usize, usize)>,
    visited_fields: HashSet<(usize, usize)>,
}

impl FieldVisiter {
    fn new(
        width: usize,
        height: usize,
        row: usize,
        col: usize,
    ) -> Result<FieldVisiter, &'static str> {
        if row >= height || col >= width {
            Err("Invalid index!")
        } else {
            let mut fields_to_visit = IndexSet::new();
            fields_to_visit.insert((row, col));
            Ok(FieldVisiter {
                width,
                height,
                fields_to_visit,
                visited_fields: HashSet::new(),
            })
        }
    }

    fn extend_with_unvisited_neighbors(&mut self, row: usize, col: usize) {
        let fields_to_extend: HashSet<(usize, usize)> =
            get_neighbor_fields(self.width, self.height, row, col)
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
    width: usize,
    height: usize,
    number_of_mines: usize,
) -> Result<HashSet<(usize, usize)>, &'static str> {
    let max_number_of_mines = (width * height - 1) as usize;
    let min_number_of_mines = 1;

    if max_number_of_mines < number_of_mines {
        return Err("Too much mines!");
    }
    if min_number_of_mines > number_of_mines {
        return Err("Too few mines!");
    }

    let mut mine_locations = HashSet::new();
    while (mine_locations.len() as usize) < number_of_mines {
        mine_locations.insert((
            rand::random::<usize>() % height,
            rand::random::<usize>() % width,
        ));
    }
    Ok(mine_locations)
}

fn get_neighbor_fields(
    width: usize,
    height: usize,
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
    width: usize,
    height: usize,
    row: usize,
    col: usize,
    mine_locations: &HashSet<(usize, usize)>,
) -> Result<u8, &'static str> {
    if mine_locations.contains(&(row, col)) {
        return Err("Mine does not have value!");
    }
    let mut field_value: u8 = 0;

    for (r, c) in get_neighbor_fields(width, height, row, col) {
        if mine_locations.contains(&(r, c)) {
            field_value = field_value + 1;
        }
    }

    Ok(field_value)
}

fn generate_fields(
    width: usize,
    height: usize,
    mine_locations: &HashSet<(usize, usize)>,
) -> Result<Vec<Vec<FieldInner>>, &'static str> {
    let mut fields = Vec::new();

    for r in 0..height {
        let mut row = Vec::new();
        for c in 0..width {
            if mine_locations.contains(&(r, c)) {
                row.push(FieldInner::new_mine());
            } else {
                let value = get_field_value(width, height, r, c, mine_locations)?;
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
    width: usize,
    height: usize,
    mine_locations: HashSet<(usize, usize)>,
    number_of_opened_fields: usize,
    fields: Vec<Vec<FieldInner>>,
}

impl Table {
    fn with_custom_mines(
        width: usize,
        height: usize,
        mine_locations: HashSet<(usize, usize)>,
    ) -> Result<Table, &'static str> {
        let fields = generate_fields(width, height, &mine_locations)?;
        Ok(Table {
            width,
            height,
            mine_locations,
            number_of_opened_fields: 0,
            fields,
        })
    }

    pub fn new(width: usize, height: usize, number_of_mines: usize) -> Result<Table, &'static str> {
        let mine_locations = generate_mine_locations(width, height, number_of_mines)?;
        Table::with_custom_mines(width, height, mine_locations)
    }

    fn get_neighbor_fields(&self, row: usize, col: usize) -> HashSet<(usize, usize)> {
        get_neighbor_fields(self.width, self.height, row, col)
    }

    fn get_field_value(&self, row: usize, col: usize) -> Result<u8, &'static str> {
        get_field_value(self.width, self.height, row, col, &self.mine_locations)
    }

    fn all_fields_are_open(&self) -> bool {
        self.width * self.height == self.mine_locations.len() + self.number_of_opened_fields
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for row in self.fields.iter() {
            for cell in row.iter() {
                print!("{}", cell.get_char_repr());
            }
            println!("");
        }

        if self.all_fields_are_open() {
            println!("You are the winner!");
        } else {
            println!(
                "Number of fields that are needed to be opened: {}",
                self.width * self.height - self.mine_locations.len() - self.number_of_opened_fields
            );
        }
    }

    fn move_mine(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
        if self.fields[row][col].field_type.is_mine() {
            let mut new_place = (0, 0);
            let mut visiter = FieldVisiter::new(self.width, self.height, row, col)?;
            while let Some((r, c)) = visiter.next() {
                if !self.fields[r][c].field_type.is_mine() {
                    new_place = (r, c);
                    break;
                }
                visiter.extend_with_unvisited_neighbors(r, c);
            }

            self.fields[new_place.0][new_place.1].update_type_to_mine();
            self.fields[row][col].update_type_to_empty();
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
                        0 => self.fields[r][c].update_type_to_empty(),
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
        if self.fields[row][col].get_field_state().is_flagged() {
            return Ok(OpenInfo {
                result: OpenResult::IsFlagged,
                field_infos: HashMap::new(),
            });
        }

        if self.number_of_opened_fields == 0 && self.fields[row][col].field_type.is_mine() {
            self.move_mine(row, col)?;
        }

        let mut visiter = FieldVisiter::new(self.width, self.height, row, col)?;
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
            Err("Invalid index!")
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
        width: usize,
        height: usize,
        table: RefCell<Table>,
        mine_locations: HashSet<(usize, usize)>,
        fields: Vec<Vec<FieldType>>,
    }

    // O O 1 M 1
    // O 1 2 2 1
    // 1 2 M 2 1
    // M 3 3 M 1
    // 2 M 2 1 1
    fn create_test_info_5x5() -> TestInfo {
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
            ],
            vec![
                FieldType::Empty,
                FieldType::Numbered(1),
                FieldType::Numbered(2),
                FieldType::Numbered(2),
                FieldType::Numbered(1),
            ],
            vec![
                FieldType::Numbered(1),
                FieldType::Numbered(2),
                FieldType::Mine,
                FieldType::Numbered(2),
                FieldType::Numbered(1),
            ],
            vec![
                FieldType::Mine,
                FieldType::Numbered(3),
                FieldType::Numbered(3),
                FieldType::Mine,
                FieldType::Numbered(1),
            ],
            vec![
                FieldType::Numbered(2),
                FieldType::Mine,
                FieldType::Numbered(2),
                FieldType::Numbered(1),
                FieldType::Numbered(1),
            ],
        ];
        TestInfo {
            width: 5,
            height: 5,
            table: RefCell::new(Table::with_custom_mines(5, 5, mine_locations.clone()).unwrap()),
            mine_locations,
            fields,
        }
    }

    #[test]
    fn field_visiter() {
        let table = Table::new(10, 10, 10).unwrap();
        let mut visiter = FieldVisiter::new(table.width, table.height, 5, 5).unwrap();
        let mut expected_fields_to_visit = IndexSet::new();
        expected_fields_to_visit.insert((5, 5));
        assert_eq!(expected_fields_to_visit, visiter.fields_to_visit);
        visiter.extend_with_unvisited_neighbors(5, 5);
        for i in 4..7 {
            for j in 4..7 {
                expected_fields_to_visit.insert((i, j));
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
        visiter.extend_with_unvisited_neighbors(5, 5);
        assert_eq!(expected_fields_to_visit, visiter.fields_to_visit);
        while let Some(item) = visiter.next() {
            expected_fields_to_visit.remove(&item);
            visiter.extend_with_unvisited_neighbors(5, 5);
        }
        assert_eq!(expected_fields_to_visit, visiter.fields_to_visit);
    }

    #[test]
    fn open_everything() {
        let test_info = create_test_info_5x5();
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
        let test_info = create_test_info_5x5();
        let mut table = test_info.table.borrow_mut();
        for row in 0..test_info.height {
            for col in 0..test_info.width {
                if !test_info.mine_locations.contains(&(row, col)) && !(row == 4 && col == 4) {
                    let open_result = table.open_field(row, col).unwrap();
                    assert_eq!(OpenResult::Ok, open_result.result);
                }
            }
        }
        assert!(table.open_field(4, 4).unwrap().result == OpenResult::WINNER);
    }

    #[test]
    fn open_mine_first() {
        let test_info = create_test_info_5x5();
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
        let test_info = create_test_info_5x5();
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
        let test_info = create_test_info_5x5();
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
                    test_info.fields[row][col],
                    open_result.field_infos.get(&(row, col),)
                );
            }
        }
    }

    #[test]
    fn flag_and_unflag() {
        let test_info = create_test_info_5x5();
        let mut table = test_info.table.borrow_mut();
        let flag_result = table.toggle_flag(0, 1).unwrap();
        assert_eq!(FieldFlagResult::Flagged, flag_result);
        let unflag_result = table.toggle_flag(0, 1).unwrap();
        assert_eq!(FieldFlagResult::FlagRemoved, unflag_result);
    }

    #[test]
    fn open_flagged() {
        let test_info = create_test_info_5x5();
        let mut table = test_info.table.borrow_mut();
        let toggle_result = table.toggle_flag(1, 1).unwrap();
        assert_eq!(FieldFlagResult::Flagged, toggle_result);
        let open_result = table.open_field(1, 1).unwrap();
        assert_eq!(OpenResult::IsFlagged, open_result.result);
        assert_eq!(0, open_result.field_infos.len());
    }

    #[test]
    fn flagged_bubble_is_not_opened() {
        let test_info = create_test_info_5x5();
        let mut table = test_info.table.borrow_mut();
        let toggle_result = table.toggle_flag(0, 1).unwrap();
        assert_eq!(FieldFlagResult::Flagged, toggle_result);
        let open_result = table.open_field(1, 0).unwrap();
        assert_eq!(OpenResult::Ok, open_result.result);
        let field_infos = open_result.field_infos;
        assert_eq!(5, field_infos.len());

        assert_eq!(FieldType::Empty, open_result.field_infos.get(&(0, 0)));

        for row in 1..3 {
            for column in 0..2 {
                assert_eq!(
                    test_info.fields[row][column],
                    open_result.field_infos.get(&(row, column))
                );
            }
        }
    }

    // TODO Write test to full game
}
