pub struct Board {
    board: Vec<Vec<char>>,
}
pub struct Cell {
    x: usize,
    y: usize,
}

pub enum ErrCoordinates {
    InvalidSub,
}

pub enum ErrBoard {
    OutOfBounds,
    PossitionTaken,
}

impl Cell {
    pub fn new() -> Cell {
        Cell { x: 0, y: 0 }
    }
    pub fn index_to_coordinates(&mut self, i: usize) -> Result<(), ErrCoordinates> {
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
    pub fn new() -> Board {
        Board {
            board: vec![vec!['\u{25A2}'; 3]; 3],
        }
    }
    pub fn print_a_board(&self) {
        for i in 0..3 {
            for j in 0..3 {
                print!("{:2}", self.board[i][j]);
            }
            println!();
        }
    }
    pub fn place_a_sign(&mut self, cell: &mut Cell, sign: &mut char) -> Result<(), ErrBoard> {
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
    pub fn check_for_win(&mut self) -> Option<char> {
        //check for a horizontal win
        for row in self.board.iter() {
            if !row.iter().any(|i| *i == '\u{25A2}') && row.iter().all(|&x| x == row[0]) {
                return Some(row[0]);
            }
        }
        //check for a vertical win
        for (i, enumerator) in self.board.iter().enumerate() {
            if !self.board.iter().any(|row| row[i] == '\u{25A2}')
                && self.board.iter().all(|x| x[i] == enumerator[i])
            {
                return Some(enumerator[i]);
            }
        }
        //check for a diagonal(upper left to lower right) win
        for (i, enumerator) in self.board.iter().enumerate() {
            if !self
                .board
                .iter()
                .enumerate()
                .any(|(i_inner, row)| row[i_inner] == '\u{25A2}')
                && self
                    .board
                    .iter()
                    .enumerate()
                    .all(|(i_iner, row)| row[i_iner] == enumerator[i])
            {
                return Some(enumerator[i]);
            }
        }
        //check for a diagonal (lower left to upper right) win
        for (i, enumerator) in self.board.iter().rev().enumerate() {
            if !self
                .board
                .iter()
                .rev()
                .enumerate()
                .any(|(i_inner, row)| row[i_inner] == '\u{25A2}')
                && self
                    .board
                    .iter()
                    .rev()
                    .enumerate()
                    .all(|(i_inner, row)| row[i_inner] == enumerator[i])
            {
                return Some(enumerator[i]);
            }
        }
        //ceck for a tie
        if self
            .board
            .iter()
            .all(|x| x.iter().all(|&y| y != '\u{25A2}'))
        {
            return Some('T');
        }
        None
    }
}