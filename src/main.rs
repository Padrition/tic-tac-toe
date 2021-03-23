use std::io;

struct Board {
    board: Vec<Vec<char>>,
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
    fn place_a_sign(&mut self, row: usize,column: usize, sign: char) {
        self.board[row][column] = sign;
    }
    fn new() -> Board {
        Board {
            board: vec![vec!['\u{25A2}'; 3];3],
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

    loop {
        b.print_a_board();

        println!("Enter the number of a row you want to sign:");

        let mut cell_number = String::new();

        io::stdin()
            .read_line(&mut cell_number)
            .expect("Faild to read the input.");

        let row: usize;

        match cell_number.trim().parse::<usize>() {
            Ok(i) => {
                row = i;
            }
            Err(_) => {
                println!("Enter a valid number!");
                continue;
            }
        }
        println!("Enter the number of a column you want to sign:");

        let mut cell_number = String::new();

        io::stdin()
            .read_line(&mut cell_number)
            .expect("Faild to read the input.");

        let column: usize;

        match cell_number.trim().parse::<usize>() {
            Ok(i) => {
                column = i;
            }
            Err(_) => {
                println!("Enter a valid number!");
                continue;
            }
        }

        match sign {
            'X' => {
                if b.board[row][column] == '\u{25A2}' {
                    b.place_a_sign(row, column, sign);
                    sign = 'O';
                } else {
                    println!("This possition is already taken! Try another one!");
                    continue;
                }
            }
            'O' => {
                if b.board[row][column] == '\u{25A2}' {
                    b.place_a_sign(row, column, sign);
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
