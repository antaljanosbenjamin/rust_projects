use indexmap::IndexSet;
use std::char;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq)]
pub enum FieldState {
    Closed,
    Opened,
    Flagged,
}

#[derive(PartialEq)]
enum FieldType {
    Empty,
    Numbered(u8),
    Mine,
}

impl FieldState {
    fn is_opened(&self) -> bool {
        self == &FieldState::Opened
    }

    fn is_flagged(&self) -> bool {
        self == &FieldState::Flagged
    }
}

#[derive(PartialEq)]
pub enum FieldFlagResult {
    Flagged,
    Unflagged,
    AlreadyOpened,
}

#[derive(PartialEq)]
enum FieldOpenResult {
    AlreadyOpened,
    SimpleOpen,
    MultiOpen,
    Boom,
    IsFlagged,
}

pub trait Field {
    fn get_field_state(&self) -> FieldState;
    fn get_char_repr(&self) -> char;
    fn is_mine(&self) -> bool;
}

struct FieldInner {
    field_type: FieldType,
    state: FieldState,
}

impl FieldInner {
    fn new_mine() -> FieldInner {
        FieldInner {
            field_type: FieldType::Mine,
            state: FieldState::Closed,
        }
    }

    fn new_with_value(value: u8) -> FieldInner {
        FieldInner {
            field_type: FieldInner::create_field_type_with_value(value),
            state: FieldState::Closed,
        }
    }

    fn create_field_type_with_value(value: u8) -> FieldType {
        if value == 0 {
            FieldType::Empty
        } else {
            FieldType::Numbered(value)
        }
    }

    fn update_type_to_mine(&mut self) {
        self.field_type = FieldType::Mine;
    }

    fn update_type_with_value(&mut self, value: u8) {
        self.field_type = FieldInner::create_field_type_with_value(value);
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
            FieldFlagResult::Unflagged
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

    fn get_char_repr(&self) -> char {
        if self.state.is_flagged() {
            'H'
        } else if !self.state.is_opened() {
            'O'
        } else {
            match self.field_type {
                FieldType::Empty => ' ',
                FieldType::Numbered(x) => std::char::from_digit(x as u32, 10).unwrap(),
                FieldType::Mine => 'X',
            }
        }
    }

    fn is_mine(&self) -> bool {
        self.field_type == FieldType::Mine
    }
}

pub struct Table {
    width: usize,
    height: usize,
    mine_locations: HashSet<(usize, usize)>,
    number_of_opened_fields: usize,
    fields: Vec<Vec<FieldInner>>,
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

pub enum OpenResult {
    Ok,
    Boom,
    WINNER,
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
                row.push(FieldInner::new_with_value(value))
            }
        }
        fields.push(row);
    }
    Ok(fields)
}

impl Table {
    pub fn new(width: usize, height: usize, number_of_mines: usize) -> Result<Table, &'static str> {
        let mine_locations = generate_mine_locations(width, height, number_of_mines)?;
        let fields = generate_fields(width, height, &mine_locations)?;
        Ok(Table {
            width,
            height,
            mine_locations,
            number_of_opened_fields: 0,
            fields,
        })
    }

    fn get_neighbor_fields(&self, row: usize, col: usize) -> HashSet<(usize, usize)> {
        get_neighbor_fields(self.width, self.height, row, col)
    }

    fn all_fields_are_open(&self) -> bool {
        self.width * self.height == self.mine_locations.len() + self.number_of_opened_fields
    }

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
        if self.fields[row][col].is_mine() {
            let mut new_place = (0, 0);
            let mut visiter = FieldVisiter::new(self.width, self.height, row, col)?;
            while let Some((r, c)) = visiter.next() {
                if !self.fields[r][c].is_mine() {
                    new_place = (r, c);
                    break;
                }
                visiter.extend_with_unvisited_neighbors(r, c);
            }

            self.fields[new_place.0][new_place.1].update_type_to_mine();
            self.fields[row][col].update_type_with_value(0);
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
                if !self.fields[r][c].is_mine() {
                    self.fields[r][c].update_type_with_value(
                        get_field_value(self.width, self.height, r, c, &self.mine_locations)
                            .unwrap(),
                    );
                }
            }
        }

        Ok(())
    }

    pub fn open_field(&mut self, row: usize, col: usize) -> Result<OpenResult, &'static str> {
        if self.number_of_opened_fields == 0 && self.fields[row][col].is_mine() {
            self.move_mine(row, col)?;
        }

        let mut visiter = FieldVisiter::new(self.width, self.height, row, col)?;

        while let Some((r, c)) = visiter.next() {
            match self.fields[r][c].open() {
                FieldOpenResult::MultiOpen => {
                    self.number_of_opened_fields = self.number_of_opened_fields + 1;
                    visiter.extend_with_unvisited_neighbors(r, c);
                }
                FieldOpenResult::SimpleOpen => {
                    self.number_of_opened_fields = self.number_of_opened_fields + 1;
                }
                FieldOpenResult::Boom => return Ok(OpenResult::Boom),
                _ => (),
            }
        }

        if self.all_fields_are_open() {
            Ok(OpenResult::WINNER)
        } else {
            Ok(OpenResult::Ok)
        }
    }

    pub fn toggle_flag(&mut self, row: usize, col: usize) -> Result<bool, &'static str> {
        if row >= self.height || col >= self.width {
            Err("Invalid index!")
        } else {
            Ok(self.fields[row][col].toggle_flag() == FieldFlagResult::Flagged)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn field_visiter() {
        let table = Table::new(10, 10, 10).unwrap();
        let mut visiter = FieldVisiter::new(table.width, table.height, 5, 5).unwrap();
        let mut expected_fields_to_visit = IndexSet::new();
        expected_fields_to_visit.insert((5, 5));
        assert!(visiter.fields_to_visit == expected_fields_to_visit);
        visiter.extend_with_unvisited_neighbors(5, 5);
        for i in 4..7 {
            for j in 4..7 {
                expected_fields_to_visit.insert((i, j));
            }
        }
        assert!(visiter.fields_to_visit == expected_fields_to_visit);

        let mut item = visiter.next().unwrap();
        expected_fields_to_visit.remove(&item);
        item = visiter.next().unwrap();
        expected_fields_to_visit.remove(&item);
        item = visiter.next().unwrap();
        expected_fields_to_visit.remove(&item);
        assert!(visiter.fields_to_visit == expected_fields_to_visit);
        visiter.extend_with_unvisited_neighbors(5, 5);
        assert!(visiter.fields_to_visit == expected_fields_to_visit);
        while let Some(item) = visiter.next() {
            expected_fields_to_visit.remove(&item);
            visiter.extend_with_unvisited_neighbors(5, 5);
        }
        assert!(visiter.fields_to_visit == expected_fields_to_visit);
    }
}
