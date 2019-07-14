use crate::logic::field::{DummyField, EmptyField, Field};

pub struct Table {
    values: Vec<Vec<Box<dyn Field>>>,
}

impl Table {
    pub fn new(width: u8, height: u8) -> Table {
        let mut table = Table{values: Vec::<Vec<Box<dyn Field>>>::new()}; 
        for r in 0..height {
            let mut row = Vec::<Box<dyn Field>>::new();
            for c in 0..width {
                match (r + c) % 2 {
                    1 => row.push(Box::new(DummyField {})),
                    _ => row.push(Box::new(EmptyField {}))
                }
            }
            table.values.push(row);
        }
        table
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
