use crate::logic::field::Field;
use std::collections::HashSet;

pub struct Table {
    width: usize,
    height: usize,
    number_of_mines: usize,
    mine_locations: Option<HashSet<(usize, usize)>>,
    fields: Option<Vec<Vec<Box<dyn Field>>>>,
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

    fn get_field_value(&self, row: usize, col: usize) -> Result<usize, &'static str> {
        if self.mine_locations.is_none() {
            return Err("Mine locations are missing");
        }

        let mine_locations = self.mine_locations.as_ref().unwrap();
        let mut field_value: usize = 0;

        fn add(u: usize, i: i8) -> Option<usize> {
            if i.is_negative() {
                u.checked_sub(i.wrapping_abs() as u8 as usize)
            } else {
                u.checked_add(i as usize)
            }
        };

        for offset in NEIGHBOR_OFFSETS.iter() {
            match (add(row, offset.0), add(col, offset.1)) {
                (Some(r), Some(c)) => {
                    if mine_locations.contains(&(r, c)) {
                        field_value = field_value + 1;
                    }
                }
                _ => (),
            }
        }

        Ok(field_value)
    }

    fn generate_fields(&mut self) -> Result<(), &'static str> {
        if self.mine_locations.is_none() {
            return Err("Mine locations are missing!");
        }
        let &mine_locations = &self.mine_locations.as_ref().unwrap();

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
}
