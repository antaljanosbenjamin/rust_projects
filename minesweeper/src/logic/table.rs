use crate::logic::field::{DummyField, Field};
use std::collections::HashSet;

pub struct Table {
    values: Vec<Vec<Box<dyn Field>>>,
    number_of_mines: u16
}

impl Table {
    pub fn new(width: u8, height: u8) -> Table {
        let mut table = Table{values: Vec::<Vec<Box<dyn Field>>>::new()}; 
        let mut table = Table{values: Vec::<Vec<Box<dyn Field>>>::new(), number_of_mines};
        let mut mines_location = HashSet::new();
        while (mines_location.len() as u16) < number_of_mines {
            mines_location.insert((rand::random::<u8>() % height, rand::random::<u8>() % width ));
        }
        for r in 0..height {
            let mut row = Vec::<Box<dyn Field>>::new();
            for c in 0..width {
                if  mines_location.contains(&(r,c)) {
                    row.push(Field::new(true, 0));
                } else {
                    row.push(Field::new(false, 0))
                }
            }
            table.values.push(row);
        }
        Ok(table)
    }

    pub fn print(&self) {
        for row in self.values.iter() {
            for cell in row.iter() {
                print!("{}", cell.get_char_repr());
            }
            println!("");
        }
    }
}
