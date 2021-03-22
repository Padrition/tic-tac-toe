use std::io;

struct Board{
    board : Vec<char>,
}

impl Board{
    fn print_a_board(&self){
        let mut index = 0;
        for _ in 0..3{
            for _ in 0..3{
                print!("{:2}", self.board[index]);
                index += 1;
            }
            print!("\n");
        }
    }
    fn place_a_sign(&mut self,index: usize, sign : char){
        self.board[index-1] = sign;
    }
    fn new()-> Board{
        Board{
            board: vec!['\u{25A2}';9],
        }
    }
}

fn main() {
    println!("\tTIC-TAC-TOE");
    println!("The rulse:
            \rEvery turn you asked where you want to place your sign(O or X).
            \rIf you fill any row column or horizontal with your signs you win.");

    let mut b = Board::new();

    loop{
        b.print_a_board();

        println!("Enter the number of a cell you want to sign:");

        let mut cell_number = String::new();

        io::stdin()
            .read_line(&mut cell_number)
            .expect("Faild to read the input.");

        let index : usize;

        match cell_number.trim().parse(){
            Ok(i) => index = i,
            Err(_) => {
                println!("Enter a valid number!");
                continue;
            }
        }

        b.place_a_sign(index, 'X');
    }
    
}
