use std::env;

struct Door {
    contains_jackpot: bool,
    is_opened: bool,
}

impl Door {
    fn new() -> Door {
        Door {
            contains_jackpot: false,
            is_opened: false,
        }
    }
}

struct DoorGame {
    doors: Vec<Door>,
    chosen_door: usize,
}

impl DoorGame {
    fn new(door_count: usize) -> DoorGame {
        let mut new_game = DoorGame {
            doors: Vec::with_capacity(door_count),
            chosen_door: 0,
        };
        for _i in 0..door_count {
            new_game.doors.push(Door::new());
        }
        new_game.chosen_door = new_game.rand_door();
        let jackpot_holder = new_game.rand_door();
        new_game.doors[jackpot_holder].contains_jackpot = true;
        return new_game;
    }

    fn open_doors(self: &mut DoorGame) {
        let mut num_of_opened_doors = 0;
        for _i in 0..self.doors.len() {
            if _i != self.chosen_door
                && !self.doors[_i].is_opened
                && !self.doors[_i].contains_jackpot
            {
                self.doors[_i].is_opened = true;
                num_of_opened_doors += 1;
                if num_of_opened_doors == self.doors.len() - 2 {
                    break;
                }
            }
        }

        assert_eq!(self.doors.len() - 2, num_of_opened_doors);
    }

    fn change_chosen(self: &mut DoorGame) {
        let old_chosen = self.chosen_door;
        for _i in (0..self.doors.len()).rev() {
            if _i != self.chosen_door && !self.doors[_i].is_opened {
                self.chosen_door = _i;
                break;
            }
        }
        assert_ne!(old_chosen, self.chosen_door);
        assert!(!self.doors[self.chosen_door].is_opened);
    }

    fn is_user_won(self: &DoorGame) -> bool {
        self.doors[self.chosen_door].contains_jackpot
    }

    fn print(self: &DoorGame) {
        for door in &self.doors {
            print!(
                "[{}]",
                if door.contains_jackpot {
                    'J'
                } else if door.is_opened {
                    ' '
                } else {
                    'X'
                }
            )
        }
        println!()
    }

    fn print_with_chosen(self: &DoorGame) {
        self.print();
        print!(
            "{}",
            (0..self.chosen_door).map(|_| "   ").collect::<String>()
        );
        println!(" ^ ");
    }

    fn rand_door(self: &DoorGame) -> usize {
        rand::random::<usize>() % self.doors.len()
    }
}

fn convert_string_to_bool(arg: &String) -> Option<bool> {
    if !(arg == "t" || arg == "f") {
        return None;
    } else {
        Some(arg == "t")
    }
}

fn convert_arg_to_bool(arg: &Option<&String>) -> Option<bool> {
    match arg {
        None => None,
        Some(str_arg) => convert_string_to_bool(str_arg),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    let n = args[1]
        .parse::<usize>()
        .expect("Please provide the number of doors as the first parameter!");

    let game_count = args[2]
        .parse::<usize>()
        .expect("Please provide the number of games as second parameter!");

    let switch_choice = convert_arg_to_bool(&args.get(3))
        .expect("Please provide either 't' (true) or 'f' (false) as third parameter to determine whether the user switches his choice or not!");

    let print_dbg = convert_arg_to_bool(&args.get(4)).unwrap_or(false);

    let mut win_count = 0;
    
    for _count in 0..game_count {
        let mut game = DoorGame::new(n);
        
        game.open_doors();
        if print_dbg {
            println!("Doors after opening:");
            game.print_with_chosen();
        }

        if switch_choice {
            if print_dbg {
                println!("The user switches his choice:");
            }
            game.change_chosen();
            if print_dbg {
                game.print_with_chosen();
            }
        }

        let win = game.is_user_won();
        if win {
            win_count += 1;
        }
        if print_dbg {
            println!(
                "{}",
                if win {
                    "The user wins!"
                } else {
                    "The user loses!"
                }
            );
        }
    }
    println!("The user won {} times!", win_count);
}
