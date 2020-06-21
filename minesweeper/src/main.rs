use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use std::io;
use std::vec::Vec;

mod logic;
use logic::game::{Game, GameLevel};
use logic::table::{FieldFlagResult, FieldType, OpenResult};

fn create_empty_field(width: usize, height: usize) -> Vec<Vec<char>> {
    let mut fields = Vec::new();
    for _r in 0..height {
        let mut row = Vec::new();
        for _c in 0..width {
            row.push('O');
        }
        fields.push(row);
    }
    return fields;
}

fn print_fields(fields: &Vec<Vec<char>>) {
    let col_count = fields[0].len();
    let print_horizontal_line = || {
        for _col in 0..col_count * 2 + 3 {
            print!("-",);
        }
        println!();
    };
    print_horizontal_line();
    print!("  |");
    for col in 0..col_count {
        print!(" {}", col);
    }
    println!();
    print_horizontal_line();
    let mut row_id = 0;
    for row in fields.iter() {
        print!("{} |", row_id);
        row_id = row_id + 1;
        for cell in row.iter() {
            print!(" {}", cell);
        }
        println!("");
    }
    print_horizontal_line();
}

#[derive(Debug)]
enum Action {
    Open,
    Flag,
}

struct DisplayFieldInfos(HashMap<(usize, usize), FieldType>);

impl Display for DisplayFieldInfos {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut comma_separated = String::new();

        for (coords, field_type) in self.0.iter() {
            comma_separated.push_str(&format!(
                "[{}, {}] = {}({}),\n",
                coords.0,
                coords.1,
                field_type,
                field_type.get_char_repr()
            ));
        }

        writeln!(f, "{}", comma_separated)
    }
}

fn main() {
    let mut g = Game::new(GameLevel::Beginner);
    let mut last_result = OpenResult::Ok;
    let mut fields = create_empty_field(10, 10);
    while last_result != OpenResult::WINNER && last_result != OpenResult::Boom {
        print_fields(&fields);

        let read_input = || -> Result<(Action, usize, usize), &'static str> {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Unable to read input!");
            let inputs: Vec<&str> = input.trim().split(' ').collect();
            if inputs.len() != 3 {
                return Err("Invalid number of arguments!");
            }
            let first_char = inputs[0]
                .chars()
                .next()
                .expect("First parameter cannot be empty");
            let action = match first_char {
                'o' => Action::Open,
                'f' => Action::Flag,
                _ => return Err("The possible actions are *o*pen and *f*lag!"),
            };
            let row_result = inputs[1].trim().parse::<usize>();
            if !row_result.is_ok() {
                return Err("Row id is not parsable!");
            }
            let col_result = inputs[2].trim().parse::<usize>();
            if !col_result.is_ok() {
                return Err("Col id is not parsable!");
            }
            Ok((action, row_result.unwrap(), col_result.unwrap()))
        };

        let mut read_result = read_input();
        while !read_result.is_ok() {
            eprintln!("{}", read_result.unwrap_err());
            read_result = read_input();
        }
        match read_result {
            Ok((Action::Open, r, c)) => {
                let open_result = g.open(r, c).expect("Unable to open field");
                for (coords, field_type) in open_result.field_infos.iter() {
                    fields[coords.0][coords.1] = field_type.get_char_repr();
                }
                last_result = open_result.result;
            }
            Ok((Action::Flag, r, c)) => {
                match g.toggle_flag(r, c).expect("Unable to toggle flag!") {
                    FieldFlagResult::Flagged => fields[r][c] = 'F',
                    FieldFlagResult::FlagRemoved => fields[r][c] = 'O',
                    _ => (),
                };
            }
            _ => panic!("Unexpected error happened!"),
        }
    }

    print_fields(&fields);
    println!("{}", last_result);
}
