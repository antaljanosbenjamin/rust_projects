use std::fmt::{Display, Error, Formatter};

mod logic;
use logic::game::{Game, GameLevel};

struct DisplayVec<T>(Vec<T>);

impl<T> Display for DisplayVec<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut comma_separated = String::new();

        for num in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(&num.to_string());
            comma_separated.push_str("\n");
        }

        comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
        writeln!(f, "{}", comma_separated)
    }
}

fn main() {
    let mut g = Game::new(GameLevel::Expert);
    print!("{}", DisplayVec(g.open(1, 1).unwrap().field_infos));
    g.open(9, 9).unwrap();
    print!("{}", DisplayVec(g.open(9, 9).unwrap().field_infos));
}
