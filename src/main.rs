use std::io;

struct Board {
    board: Vec<Vec<char>>,
}
struct Cell {
    x: usize,
    y: usize,
}

impl Cell {
    fn new() -> Cell {
        Cell { x: 0, y: 0 }
    }
    fn index_to_coordinates(&mut self, i: usize) {
        let i : usize = i - 1;
        self.x = i / 3;
        self.y = i % 3;
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
    fn place_a_sign(&mut self, cell: &mut Cell, sign: char) {
        self.board[cell.x][cell.y] = sign;
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

        println!("Enter a cell number:");

        let mut cell_index = String::new();

        io::stdin()
            .read_line(&mut cell_index)
            .expect("Error reading the input!");

        match cell_index.trim().parse::<usize>() {
            Ok(i) => {
                cell.index_to_coordinates(i);
            }
            Err(_) => {
                eprintln!("Enter a valid number!");
                continue;
            }
        }

        match sign {
            'X' => {
                if b.board[cell.x][cell.y] == '\u{25A2}' {
                    b.place_a_sign(&mut cell, sign);
                    sign = 'O';
                } else {
                    println!("This possition is already taken! Try another one!");
                    continue;
                }
            }
            'O' => {
                if b.board[cell.x][cell.y] == '\u{25A2}' {
                    b.place_a_sign(&mut cell, sign);
                    sign = 'X';
                } else {
                    println!("This possition is already taken! Try another one!");
                    continue;
                }
            }
            _ => panic!("How de fuck you managed to break it?"),
        }
    }
    //TODO: Write a win tie logic and do a check after each loop iteration
    //rewrite a vector to be a two dimentional
}
