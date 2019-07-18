use crate::logic::field::{Field, FieldOpenResult};
use indexmap::IndexSet;
use std::collections::HashSet;

pub struct Table {
    width: usize,
    height: usize,
    number_of_mines: usize,
    mine_locations: Option<HashSet<(usize, usize)>>,
    fields: Option<Vec<Vec<Box<dyn Field>>>>,
}

pub enum OpenResult {
    Ok,
    Boom,
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

impl Table {
    fn get_mine_locations(&self) -> Result<&HashSet<(usize, usize)>, &'static str> {
        match self.mine_locations.as_ref() {
            Some(ml) => return Ok(&ml),
            _ => return Err("Field informations are missing!"),
        }
    }

    fn get_fields_mut(&mut self) -> Result<&mut Vec<Vec<Box<dyn Field>>>, &'static str> {
        match self.fields.as_mut() {
            Some(f) => return Ok(f),
            _ => return Err("Field informations are missing!"),
        }
    }

    fn generate_mine_locations(&mut self) -> Result<(), &'static str> {
        assert!(self.mine_locations == None);
        let max_number_of_mines = (self.width as f32 * self.height as f32 * 0.5) as usize;
        let min_number_of_mines = (self.width as f32 * self.height as f32 * 0.05) as usize;
        if max_number_of_mines < self.number_of_mines {
            return Err("Too much mines!");
        }
        if min_number_of_mines > self.number_of_mines {
            return Err("Too few mines!");
        }

        let mut mine_location = HashSet::new();
        while (mine_location.len() as usize) < self.number_of_mines {
            mine_location.insert((
                rand::random::<usize>() % self.height,
                rand::random::<usize>() % self.width,
            ));
        }
        self.mine_locations = Some(mine_location);
        Ok(())
    }

    fn get_field_mut(
        &mut self,
        row: usize,
        col: usize,
    ) -> Result<&mut Box<dyn Field>, &'static str> {
        if self.height <= row {
            return Err("The row does not exist!");
        }
        if self.width <= col {
            return Err("The column does not exist!");
        }
        let fields = self.get_fields_mut()?;

        Ok(&mut fields[row][col])
    }

    fn get_neighbor_fields(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
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
                (Some(r), Some(c)) if r < self.height && c < self.width => {
                    neighbors.push((r, c));
                }
                _ => (),
            }
        }

        neighbors
    }

    fn get_field_value(&self, row: usize, col: usize) -> Result<usize, &'static str> {
        let mine_locations = self.get_mine_locations()?;
        if mine_locations.contains(&(row, col)) {
            return Err("Mine does not have value!");
        }
        let mut field_value: usize = 0;
        for (r, c) in self.get_neighbor_fields(row, col) {
            if mine_locations.contains(&(r, c)) {
                field_value = field_value + 1;
            }
        }

        Ok(field_value)
    }

    fn generate_fields(&mut self) -> Result<(), &'static str> {
        let mine_locations = self.get_mine_locations()?;

        let mut fields = Vec::<Vec<Box<dyn Field>>>::new();
        for r in 0..self.height {
            let mut row = Vec::<Box<dyn Field>>::new();
            for c in 0..self.width {
                if mine_locations.contains(&(r, c)) {
                    row.push(Field::new(true, 0));
                } else {
                    let value = &self.get_field_value(r, c)?;
                    row.push(Field::new(false, *value as u8))
                }
            }
            fields.push(row);
        }
        self.fields = Some(fields);
        Ok(())
    }

    pub fn new(width: usize, height: usize, number_of_mines: usize) -> Result<Table, &'static str> {
        let mut table = Table {
            width,
            height,
            number_of_mines,
            mine_locations: None,
            fields: None,
        };
        table.generate_mine_locations()?;
        table.generate_fields()?;
        Ok(table)
    }

    pub fn print(&self) {
        match &self.fields.as_ref() {
            Some(fields) => {
                for row in fields.iter() {
                    for cell in row.iter() {
                        print!("{}", cell.get_char_repr());
                    }
                    println!("");
                }
            }
            None => println!("Field informations are missing!"),
        }
    }

    pub fn open_field(&mut self, row: usize, col: usize) -> Result<OpenResult, &'static str> {
        let mut fields_to_open = IndexSet::new();
        let mut recently_opened_fields = HashSet::new();
        fields_to_open.insert((row, col));

        while !fields_to_open.is_empty() {
            let (r, c) = fields_to_open.pop().unwrap();
            let field = self.get_field_mut(r, c)?;

            match field.as_mut().open() {
                FieldOpenResult::MultiOpen => {
                    fields_to_open.extend(self.get_neighbor_fields(r, c).into_iter());
                }
                FieldOpenResult::Boom => return Ok(OpenResult::Boom),
                _ => (),
            }

            recently_opened_fields.insert((r, c));
            if recently_opened_fields.contains(&(r, c)) {
                continue;
            }
        }

        Ok(OpenResult::Ok)
    }
}
