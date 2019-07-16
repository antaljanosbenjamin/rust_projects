use crate::logic::field::Field;
use std::collections::HashSet;

pub struct Table {
    width: u8,
    height: u8,
    number_of_mines: u16,
    mine_locations: Option<HashSet<(u8, u8)>>,
    fields: Option<Vec<Vec<Box<dyn Field>>>>,
}

impl Table {
    fn generate_mine_locations(&mut self) -> Result<(), &'static str> {
        assert!(self.mine_locations == None);
        let max_number_of_mines = (self.width as f32 * self.height as f32 * 0.5) as u16;
        let min_number_of_mines = (self.width as f32 * self.height as f32 * 0.05) as u16;
        if max_number_of_mines < self.number_of_mines {
            return Err("Too much mines!");
        }
        if min_number_of_mines > self.number_of_mines {
            return Err("Too few mines!");
        }

        let mut mine_location = HashSet::new();
        while (mine_location.len() as u16) < self.number_of_mines {
            mine_location.insert((
                rand::random::<u8>() % self.height,
                rand::random::<u8>() % self.width,
            ));
        }
        self.mine_locations = Some(mine_location);
        Ok(())
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
                    row.push(Field::new(false, 0))
                }
            }
            fields.push(row);
        }
        self.fields = Some(fields);
        Ok(())
    }

    pub fn new(width: u8, height: u8, number_of_mines: u16) -> Result<Table, &'static str> {
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
