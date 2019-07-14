mod logic;
use logic::table::{Table};

fn main() {
    let d = Table::new(5, 5);
    d.print();
}
