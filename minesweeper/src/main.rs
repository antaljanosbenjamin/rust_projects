mod logic;
use logic::table::{Table};

fn main() {
    let d = Table::new(10, 10, 10).unwrap();
    d.print();
    println!();
}