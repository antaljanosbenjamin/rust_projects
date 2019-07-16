mod logic;
use logic::table::{Table};

fn main() {
    let d = Table::new(25, 25, 100).unwrap();
    d.print();
}
