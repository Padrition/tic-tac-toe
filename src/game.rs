const BOARD_SIZE: usize = 3;
const VACANT_CELL: char = '\u{25A2}';

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
    PositionTaken,
}

impl Cell {
    pub fn new() -> Cell {
        Cell { x: 0, y: 0 }
    }
    pub fn from_index(&mut self, i: usize) -> Result<(), ErrCoordinates> {
        let i = i.checked_sub(1);
        match i {
            Some(u) => {
                self.x = u / BOARD_SIZE;
                self.y = u % BOARD_SIZE;
                Ok(())
            }
            None => Err(ErrCoordinates::InvalidSub),
        }
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: vec![vec![VACANT_CELL; BOARD_SIZE]; BOARD_SIZE],
        }
    }
    pub fn print(&self) {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                print!("{:2}", self.board[i][j]);
            }
            println!();
        }
    }
    pub fn sign(&mut self, cell: &mut Cell, sign: &mut char) -> Result<(), ErrBoard> {
        match self.board.get(cell.x) {
            Some(_) => match self.board[cell.x].get(cell.y) {
                Some(_) => {
                    if self.board[cell.x][cell.y] == VACANT_CELL {
                        self.board[cell.x][cell.y] = *sign;
                        if *sign == 'X' {
                            *sign = 'O';
                        } else {
                            *sign = 'X';
                        }
                    } else {
                        return Err(ErrBoard::PositionTaken);
                    }
                    Ok(())
                }
                None => Err(ErrBoard::OutOfBounds),
            },
            None => Err(ErrBoard::OutOfBounds),
        }
    }
    pub fn check_for_win(&mut self) -> Option<char> {
        if let Some(win) = self.horizontal_win() {
            return Some(win);
        };
        if let Some(win) = self.vertical_win() {
            return Some(win);
        };
        if let Some(win) = self.upleft_downright_diagonal_win() {
            return Some(win);
        };
        if let Some(win) = self.downleft_upright_diagonal_win() {
            return Some(win);
        };
        if let Some(win) = self.tie_check() {
            return Some(win);
        };

        None
    }
    fn horizontal_win(&self) -> Option<char> {
        for row in self.board.iter() {
            if !row.iter().any(|i| *i == VACANT_CELL) && row.iter().all(|&x| x == row[0]) {
                return Some(row[0]);
            }
        }
        None
    }
    fn vertical_win(&self) -> Option<char> {
        for (i, enumerator) in self.board.iter().enumerate() {
            if !self.board.iter().any(|row| row[i] == VACANT_CELL)
                && self.board.iter().all(|x| x[i] == enumerator[i])
            {
                return Some(enumerator[i]);
            }
        }
        None
    }
    fn upleft_downright_diagonal_win(&self) -> Option<char> {
        for (i, enumerator) in self.board.iter().enumerate() {
            if !self
                .board
                .iter()
                .enumerate()
                .any(|(i_inner, row)| row[i_inner] == VACANT_CELL)
                && self
                    .board
                    .iter()
                    .enumerate()
                    .all(|(i_inner, row)| row[i_inner] == enumerator[i])
            {
                return Some(enumerator[i]);
            }
        }
        None
    }
    fn downleft_upright_diagonal_win(&self) -> Option<char> {
        for (i, enumerator) in self.board.iter().rev().enumerate() {
            if !self
                .board
                .iter()
                .rev()
                .enumerate()
                .any(|(i_inner, row)| row[i_inner] == VACANT_CELL)
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
        None
    }
    fn tie_check(&self) -> Option<char> {
        if self
            .board
            .iter()
            .all(|x| x.iter().all(|&y| y != VACANT_CELL))
        {
            return Some('T');
        }
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn win_horizontal() {
        let mut b = Board::new();
        b.board[0] = vec!['X'; 3];
        println!("{:?}", b.board);
        assert_eq!(b.horizontal_win(), Some('X'));
    }
    #[test]
    fn win_vertical() {
        let mut b = Board::new();
        b.board[0][0] = 'O';
        b.board[1][0] = 'O';
        b.board[2][0] = 'O';
        println!("{:?}", b.board);
        assert_eq!(b.vertical_win(), Some('O'));
    }
    #[test]
    fn win_diagonal_upl_downr() {
        let mut b = Board::new();
        b.board[0][0] = 'X';
        b.board[1][1] = 'X';
        b.board[2][2] = 'X';
        println!("{:?}", b.board);
        assert_eq!(b.upleft_downright_diagonal_win(), Some('X'));
    }
    #[test]
    fn win_diagonal_downl_upr() {
        let mut b = Board::new();
        b.board[0][2] = 'O';
        b.board[1][1] = 'O';
        b.board[2][0] = 'O';
        println!("{:?}", b.board);
        assert_eq!(b.downleft_upright_diagonal_win(), Some('O'));
    }
    #[test]
    fn tie() {
        let mut b = Board::new();
        b.board[0] = vec!['O', 'X', 'O'];
        b.board[1] = vec!['O', 'O', 'X'];
        b.board[2] = vec!['X', 'X', 'O'];
        println!("{:?}", b.board);
        assert_eq!(b.tie_check(), Some('T'));
    }
}
