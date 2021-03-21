fn main() {
    //print an empty board
    //every iteration ask for a number of a cell player wants to place his sign to 
    let mut cells = vec!['1','2','3','4','5','6','7','8','9'];
    let mut index = 0;
    for _ in 0..3{
        for _ in 0..3{
            print!("{:2}|", cells[index]);
            index += 1;
        }
        print!("\n");
    }

}
