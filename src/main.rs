struct Board{
    board : Vec<char>,
}

impl Board{
    fn print_a_board(self){
        let mut index = 0;
        for _ in 0..3{
            for _ in 0..3{
                print!("{:2}|", self.board[index]);
                index += 1;
            }
            print!("\n");
        }
    }
    fn place_a_sign(&mut self, option : Option<usize>, sign : char){
        match option{
            Some(i) => {
                self.board[i] = sign;
            },
            None => println!("None"),
        }
    }
    fn new()-> Board{
        Board{
            board: vec!['1','2','3','4','5','6','7','8','9']
        }
    }
}

fn main() {
    //print an empty board
    //every iteration ask for a number of a cell player wants to place his sign to
    let mut b = Board::new();
    b.place_a_sign(Some(0), 'X');
    b.print_a_board();
}
