use std::io;

mod game;

use game::*;

fn main() {
    println!("\tTIC-TAC-TOE");
    println!(
        "The rules:
            \rEvery turn you asked where you want to place your sign(O or X).
            \rIf you fill any row column or horizontal with your signs you win."
    );

    let mut board = Board::new();
    let mut sign: char = 'X';
    let mut cell = Cell::new();
    loop {
        board.print();

        eprint!("Enter a cell number:");

        let mut cell_index = String::new();

        io::stdin()
            .read_line(&mut cell_index)
            .expect("Error reading the input!");

        match cell_index.trim().parse::<usize>() {
            Ok(i) => match cell.from_index(i) {
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
            'X' => match board.sign(&mut cell, &mut sign) {
                Ok(a) => a,
                Err(ErrBoard::PositionTaken) => {
                    eprintln!("This position is already taken! Try another one!");
                    continue;
                }
                Err(ErrBoard::OutOfBounds) => {
                    eprintln!("Enter a valid number!");
                    continue;
                }
            },
            'O' => match board.sign(&mut cell, &mut sign) {
                Ok(a) => a,
                Err(ErrBoard::PositionTaken) => {
                    eprintln!("This position is already taken! Try another one!");
                    continue;
                }
                Err(ErrBoard::OutOfBounds) => {
                    eprintln!("Enter a valid number!");
                    continue;
                }
            },
            _ => panic!("Unrepresented player "),
        }

        if let Some(winner) = board.check_for_win() {
            board.print();
            if winner == 'T' {
                println!("It's a tie!");
                break;
            }
            println!("{} won the game!", winner);
            break;
        }
    }
}
