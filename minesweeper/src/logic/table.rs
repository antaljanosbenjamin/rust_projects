use crate::logic::field::{Field, FieldOpenResult};
use indexmap::IndexSet;
use std::collections::HashSet;

pub struct Table {
    width: usize,
    height: usize,
    number_of_mines: usize,
    number_of_opened_fields: usize,
    fields: Vec<Vec<Box<dyn Field>>>,
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
    let max_number_of_mines = (width as f32 * height as f32 * 0.5) as usize;
    let min_number_of_mines = (width as f32 * height as f32 * 0.05) as usize;

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

fn get_neighbor_fields(width: usize, height: usize, row: usize, col: usize) -> Vec<(usize, usize)> {
    fn add(u: usize, i: i8) -> Option<usize> {
        if i.is_negative() {
            u.checked_sub(i.wrapping_abs() as u8 as usize)
        } else {
            u.checked_add(i as usize)
        }
    };

    let mut neighbors = Vec::new();

    for offset in NEIGHBOR_OFFSETS.iter() {
        match (add(row, offset.0), add(col, offset.1)) {
            (Some(r), Some(c)) if r < height && c < width => {
                neighbors.push((r, c));
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
) -> Result<usize, &'static str> {
    if mine_locations.contains(&(row, col)) {
        return Err("Mine does not have value!");
    }
    let mut field_value: usize = 0;

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
) -> Result<Vec<Vec<Box<dyn Field>>>, &'static str> {
    let mut fields = Vec::<Vec<Box<dyn Field>>>::new();

    for r in 0..height {
        let mut row = Vec::<Box<dyn Field>>::new();
        for c in 0..width {
            if mine_locations.contains(&(r, c)) {
                row.push(Field::new(true, 0));
            } else {
                let value = get_field_value(width, height, r, c, mine_locations)?;
                row.push(Field::new(false, value as u8))
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
            number_of_mines,
            number_of_opened_fields: 0,
            fields,
        })
    }

    fn get_neighbor_fields(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        get_neighbor_fields(self.width, self.height, row, col)
    }

    fn all_fields_are_open(&self) -> bool {
        self.width * self.height - self.number_of_mines - self.number_of_opened_fields == 0
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
                self.width * self.height - self.number_of_mines - self.number_of_opened_fields
            );
        }
    }

    pub fn open_field(&mut self, row: usize, col: usize) -> Result<OpenResult, &'static str> {
        let mut fields_to_open = IndexSet::new();
        let mut recently_opened_fields = HashSet::new();
        fields_to_open.insert((row, col));

        while !fields_to_open.is_empty() {
            let (r, c) = fields_to_open.pop().unwrap();
            match self.fields[r][c].as_mut().open() {
                FieldOpenResult::MultiOpen => {
                    self.number_of_opened_fields = self.number_of_opened_fields + 1;
                    fields_to_open.extend(self.get_neighbor_fields(r, c).into_iter());
                }
                FieldOpenResult::SimpleOpen => {
                    self.number_of_opened_fields = self.number_of_opened_fields + 1;
                }
                FieldOpenResult::Boom => return Ok(OpenResult::Boom),
                _ => (),
            }
            recently_opened_fields.insert((r, c));

            if recently_opened_fields.contains(&(r, c)) {
                continue;
            }
        }

        if self.all_fields_are_open() {
            Ok(OpenResult::WINNER)
        } else {
            Ok(OpenResult::Ok)
        }
    }
}
