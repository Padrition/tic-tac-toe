fn main() {
    //print an empty board
    //every iteration ask for a number of a cell player wants to place his sign to 
    let mut cells = vec!['1','2','3','4','5','6','7','8','9'];
    place_a_sign(&mut cells, Some(0), 'X');
    print_a_board(&mut cells);
}
fn place_a_sign(board :&mut Vec<char>, option : Option<usize>, sign : char){
    match option{
        Some(i) => {
            board[i] = sign;
        },
        None => println!("None"),
    }
}
fn print_a_board(board :&mut Vec<char>){
    let mut index = 0;
    for _ in 0..3{
        for _ in 0..3{
            print!("{:2}|", board[index]);
            index += 1;
        }
        print!("\n");
    }
}
