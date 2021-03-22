fn main() {
    //print an empty board
    //every iteration ask for a number of a cell player wants to place his sign to 
    let mut cells = vec!['1','2','3','4','5','6','7','8','9'];
    print_a_board(&mut cells, Some(1), 'X');
}

fn print_a_board(board :&mut Vec<char>, option : Option<usize>, sign : char){
    match option{
        Some(i) => {
            board[i] = sign;
        },
        None => println!("None"),
    }
    let mut index = 0;
    for _ in 0..3{
        for _ in 0..3{
            print!("{:2}|", board[index]);
            index += 1;
        }
        print!("\n");
    }
}
