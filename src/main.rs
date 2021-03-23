use std::io;

struct Board {
    board: Vec<Vec<char>>,
}
struct Cell {
    x: usize,
    y: usize,
}

enum ErrCoordinates {
    InvalidSub,
}

enum ErrBoard {
    OutOfBounds,
    PossitionTaken,
}

impl Cell {
    fn new() -> Cell {
        Cell { x: 0, y: 0 }
    }
    fn index_to_coordinates(&mut self, i: usize) -> Result<(), ErrCoordinates> {
        let i = i.checked_sub(1);
        match i {
            Some(u) => {
                self.x = u / 3;
                self.y = u % 3;
                Ok(())
            }
            None => Err(ErrCoordinates::InvalidSub),
        }
    }
}

impl Board {
    fn print_a_board(&self) {
        for i in 0..3 {
            for j in 0..3 {
                print!("{:2}", self.board[i][j]);
            }
            println!();
        }
    }
    fn place_a_sign(&mut self, cell: &mut Cell, sign: &mut char) -> Result<(), ErrBoard> {
        match self.board.get(cell.x) {
            Some(_) => match self.board[cell.x].get(cell.y) {
                Some(_) => {
                    if self.board[cell.x][cell.y] == '\u{25A2}' {
                        self.board[cell.x][cell.y] = *sign;
                        if *sign == 'X' {
                            *sign = 'O';
                        } else {
                            *sign = 'X';
                        }
                    } else {
                        return Err(ErrBoard::PossitionTaken);
                    }
                    Ok(())
                }
                None => Err(ErrBoard::OutOfBounds),
            },
            None => Err(ErrBoard::OutOfBounds),
        }
    }
    fn new() -> Board {
        Board {
            board: vec![vec!['\u{25A2}'; 3]; 3],
        }
    }
}

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

        eprintln!("Enter a cell number:");

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
            _ => panic!("How de fuck you managed to break it?"),
        }
    }
}
