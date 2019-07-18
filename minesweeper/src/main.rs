mod logic;
use logic::table::{Table};

fn main() {
    let mut d = Table::new(25, 25, 50).unwrap();
    d.open_field(1, 1).unwrap();
    d.print();
}
