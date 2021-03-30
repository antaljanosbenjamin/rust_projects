use minesweeper::{FieldType, FlagResult, Game, GameLevel, OpenResult, SizeType};

use std::io;
use std::vec::Vec;

fn create_empty_field(width: SizeType, height: SizeType) -> Vec<Vec<char>> {
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

fn get_char_repr(field_type: &FieldType) -> char {
    match field_type {
        FieldType::Empty => ' ',
        FieldType::Numbered(x) => std::char::from_digit(*x as u32, 10).unwrap(),
        FieldType::Mine => 'X',
    }
}

#[derive(Debug)]
enum Action {
    Open,
    Flag,
}

fn main() {
    let mut g = Game::new(GameLevel::Beginner);
    let mut last_result = OpenResult::Ok;
    let mut fields = create_empty_field(g.width(), g.height());
    while last_result != OpenResult::WINNER && last_result != OpenResult::Boom {
        print_fields(&fields);

        let read_input = || -> Result<(Action, SizeType, SizeType), &'static str> {
            println!("Please type your next move (<o|f> <row> <column>): ");
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
            let row_result = inputs[1].trim().parse::<SizeType>();
            if !row_result.is_ok() {
                return Err("Row id is not parsable!");
            }
            let col_result = inputs[2].trim().parse::<SizeType>();
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
                for (coords, field_type) in &open_result.newly_opened_fields {
                    fields[coords.0 as usize][coords.1 as usize] = get_char_repr(field_type);
                }
                last_result = open_result.result;
            }
            Ok((Action::Flag, r, c)) => {
                match g.toggle_flag(r, c).expect("Unable to toggle flag!") {
                    FlagResult::Flagged => fields[r as usize][c as usize] = 'F',
                    FlagResult::FlagRemoved => fields[r as usize][c as usize] = 'O',
                    _ => (),
                };
            }
            _ => panic!("Unexpected error happened!"),
        }
    }

    print_fields(&fields);
    println!("{}", last_result);
}
