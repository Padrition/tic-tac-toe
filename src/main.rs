use std::io;

mod game;

use game::*;

fn main() {
    println!("\tTIC-TAC-TOE");
    println!(
        "The rulse:
            \rEvery turn you asked where you want to place your sign(O or X).
            \rIf you fill any row column or horizontal with your signs you win."
    );

    let mut b = Board::new();
    let mut sign: char = 'X';
    let mut cell = Cell::new();
    loop {
        b.print_a_board();

        eprint!("Enter a cell number:");

        let mut cell_index = String::new();

        io::stdin()
            .read_line(&mut cell_index)
            .expect("Error reading the input!");

        match cell_index.trim().parse::<usize>() {
            Ok(i) => match cell.index_to_coordinates(i) {
                Ok(a) => a,
                Err(ErrCoordinates::InvalidSub) => {
                    eprintln!("Enter a valid number!");
                    continue;
                }
            },
            Err(_) => {
                eprintln!("Enter a valid number!");
                continue;
            }
        }

        match sign {
            'X' => match b.place_a_sign(&mut cell, &mut sign) {
                Ok(a) => a,
                Err(ErrBoard::PossitionTaken) => {
                    eprintln!("This possition is already taken! Try another one!");
                    continue;
                }
                Err(ErrBoard::OutOfBounds) => {
                    eprintln!("Enter a valid number!");
                    continue;
                }
            },
            'O' => match b.place_a_sign(&mut cell, &mut sign) {
                Ok(a) => a,
                Err(ErrBoard::PossitionTaken) => {
                    eprintln!("This possition is already taken! Try another one!");
                    continue;
                }
                Err(ErrBoard::OutOfBounds) => {
                    eprintln!("Enter a valid number!");
                    continue;
                }
            },
            _ => panic!("Unpresented player "),
        }

        if let Some(winer) = b.check_for_win() {
            b.print_a_board();
            if winer == 'T' {
                println!("It's a tie!");
                break;
            }
            println!("{} won the game!", winer);
            break;
        }
    }
}
