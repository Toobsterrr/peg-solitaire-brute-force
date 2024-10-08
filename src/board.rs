use std::fmt::Display;


// const POSSIBLE_SOLUTION: [Move; 31] = [Move::Down(Piece(2, 4)), Move::Right(Piece(3, 2)), Move::Down(Piece(1, 3)), Move::Left(Piece(1, 5)), Move::Left(Piece(3, 4)), Move::Right(Piece(3, 1)), Move::Up(Piece(3, 5)), Move::Left(Piece(3, 7)), Move::Up(Piece(4, 3)), Move::Down(Piece(1, 3)), Move::Right(Piece(4, 1)), Move::Up(Piece(4, 3)), Move::Left(Piece(4, 5)), Move::Left(Piece(4, 7)), Move::Up(Piece(4, 5)), Move::Down(Piece(1, 5)), Move::Up(Piece(5, 3)), Move::Down(Piece(2, 3)), Move::Right(Piece(5, 1)), Move::Left(Piece(5, 4)), Move::Up(Piece(6, 5)), Move::Left(Piece(5, 7)), Move::Up(Piece(7, 3)), Move::Down(Piece(4, 3)), Move::Left(Piece(7, 5)), Move::Up(Piece(7, 3)), Move::Right(Piece(5, 2)), Move::Right(Piece(5, 4)), Move::Down(Piece(3, 5)), Move::Left(Piece(5, 6)), Move::Down(Piece(5, 4))];


#[derive(Clone, Copy)]
pub struct Board {
    board_size: u32,
    pieces: [[bool; 7]; 7],
    nb_pieces: u32
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.pieces.len() {
            // row except last character
            write!(f, "{} ", row+1).unwrap();
            for col in 0..self.pieces[row].len()-1 {
                if self.pieces[row][col] {
                    write!(f, "+ ").unwrap();
                } else {
                    write!(f, "  ").unwrap();
                }
            }

            // last character doesnt have a space after
            if self.pieces[row][self.pieces.len() as usize - 1] {
                write!(f, "+\n").unwrap();    
            } else {
                write!(f, " \n").unwrap();
            }
        }
        write!(f, " ").unwrap();
        for i in 1..=self.pieces[0].len() {
            write!(f, " {}", i).unwrap();
        }
        write!(f, "")
    }
}

impl Board {


    pub fn new() -> Self {
        let mut b = Board {
            board_size: 7, 
            pieces: [[true; 7]; 7], 
            nb_pieces: 32
        };


        for row in (0..=1).chain(5..=6) {
            for col in (0..=1).chain(5..=6) {
                b.pieces[row][col] = false;
            }
        }
        b.pieces[3][3] = false;

        b
    }


    // fn get_symmetries(&self) -> Symmetry {
    //     let mut res = Symmetry::default();


    //     // row wise
    //     res.horizontal = true;
    //     for row in 0..self.pieces.len()/2 {
    //         if self.pieces[row] != self.pieces[self.pieces.len() - 1 - row] {
    //             res.horizontal = false;
    //             break;
    //         }
    //     }

    //     // column wise
    //     res.vertical = true;
    //     for col in 0..self.pieces[0].len()/2 {
    //         for row in 0..self.pieces.len() {
    //             if self.pieces[row][col] != self.pieces[row][self.pieces[0].len() - 1 - col] {
    //                 res.vertical = false;
    //             }
    //         }
    //     }

    //     // left diagonal
    //     res.left_diagonal = true;
    //     for i in 0..self.pieces.len() {
    //         for j in 0..=i {
    //             if self.pieces[i-j][i] != self.pieces[i][i-j] {
    //                 res.left_diagonal = false;
    //             }
    //         }
    //     }

    //     // right diagonal
    //     res.right_diagonal = true;
    //     for i in 0..self.pieces.len() {
    //         for j in 0..=i {
    //             if self.pieces[i-j][self.pieces[0].len() - 1 - i] != self.pieces[i][self.pieces[0].len() - 1 - (i-j)] {
    //                 res.right_diagonal = false;
    //             }
    //         }
    //     }

    //     res
    // }

    pub fn is_finished(&self) -> bool {
        self.nb_pieces == 1
    }


    fn position_is_valid(&self, row: u32, col: u32) -> bool {

        if row < 1 || row > self.board_size || col < 1 || col > self.board_size {
            return false;
        } 

        if row < 3 && (col < 3 || col > self.board_size-2) {
            return false;
        }
        if row > self.board_size-2 && (col < 3 || col > self.board_size-2) {
            return false;
        }

        true
    }

    fn contains_piece(&self, row: u32 , col: u32) -> bool {
        self.pieces[row as usize - 1][col as usize - 1]
    }


    pub fn get_possible_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for row in 1..=self.board_size {
            for col in 1..=self.board_size {
                if self.position_is_valid(row, col+2) && self.contains_piece(row, col) && self.contains_piece(row, col + 1) && !self.contains_piece(row, col + 2) {
                    moves.push(Move::Right(Piece(row, col)));
                } else if col >= 2 && self.position_is_valid(row, col-2) && self.contains_piece(row, col) && self.contains_piece(row, col - 1) && !self.contains_piece(row, col - 2) {
                    moves.push(Move::Left(Piece(row, col)));
                } else if row >= 2 && self.position_is_valid(row-2, col) && self.contains_piece(row, col) && self.contains_piece(row - 1, col) && !self.contains_piece(row - 2, col) {
                    moves.push(Move::Up(Piece(row, col)));
                } else if self.position_is_valid(row+2, col) && self.contains_piece(row, col) && self.contains_piece(row + 1, col) && !self.contains_piece(row + 2, col) {
                    moves.push(Move::Down(Piece(row, col)));
                }
            }
        }
        moves
    }


    pub fn apply_move(&mut self, m: &Move) {
        self.nb_pieces -= 1;
        match m {
            Move::Right(p) => {
                let Piece(row, col) = *p;
                let row = row as usize - 1;
                let col = col as usize - 1;
                self.pieces[row][col] = false;
                self.pieces[row][col + 1] = false;
                self.pieces[row][col + 2] = true;
            }
            Move::Left(p) => {
                let Piece(row, col) = *p;
                let row = row as usize - 1;
                let col = col as usize - 1;
                self.pieces[row][col] = false;
                self.pieces[row][col - 1] = false;
                self.pieces[row][col - 2] = true;
            }
            Move::Up(p) => {
                let Piece(row, col) = *p;
                let row = row as usize - 1;
                let col = col as usize - 1;
                self.pieces[row][col] = false;
                self.pieces[row - 1][col] = false;
                self.pieces[row - 2][col] = true;
            }
            Move::Down(p) => {
                let Piece(row, col) = *p;
                let row = row as usize - 1;
                let col = col as usize - 1;
                self.pieces[row][col] = false;
                self.pieces[row + 1][col] = false;
                self.pieces[row + 2][col] = true;
            }
        };
    }


    pub fn undo_move(&mut self, m: &Move) {
        self.nb_pieces += 1;
        match m {
            Move::Right(p) => {
                let Piece(row, col) = *p;
                let row = row as usize - 1;
                let col = col as usize - 1;
                self.pieces[row][col] = true;
                self.pieces[row][col + 1] = true;
                self.pieces[row][col + 2] = false;
            }
            Move::Left(p) => {
                let Piece(row, col) = *p;
                let row = row as usize - 1;
                let col = col as usize - 1;
                self.pieces[row][col] = true;
                self.pieces[row][col - 1] = true;
                self.pieces[row][col - 2] = false;
            }
            Move::Up(p) => {
                let Piece(row, col) = *p;
                let row = row as usize - 1;
                let col = col as usize - 1;
                self.pieces[row][col] = true;
                self.pieces[row - 1][col] = true;
                self.pieces[row - 2][col] = false;
            }
            Move::Down(p) => {
                let Piece(row, col) = *p;
                let row = row as usize - 1;
                let col = col as usize - 1;
                self.pieces[row][col] = true;
                self.pieces[row + 1][col] = true;
                self.pieces[row + 2][col] = false;
            }
        };
    }

}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Piece(pub u32, pub u32);


// struct Move(Piece, Piece);
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    Right(Piece),
    Left(Piece),
    Up(Piece),
    Down(Piece),
}


// #[derive(Debug, PartialEq, Default)]
// pub struct Symmetry {
//     horizontal: bool,
//     vertical: bool,
//     left_diagonal: bool,
//     right_diagonal: bool,
// }


pub fn print_steps(board: &mut Board, moves: &Vec<Move>) {
    println!("{}", board);
    for m in moves {
        board.apply_move(m);
        println!("{:?}---------------------------\n{}", m, board);
    }
}
