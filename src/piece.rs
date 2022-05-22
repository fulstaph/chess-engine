use crate::board::Board;
use crate::color::Color;
use crate::r#move::Move;
use crate::square::Square;
use core::fmt::{self, Display};
use log::{debug, info};
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug, Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn value(self) -> usize {
        match self {
            PieceType::Pawn => 100,
            PieceType::Knight => 280,
            PieceType::Bishop => 320,
            PieceType::Rook => 479,
            PieceType::Queen => 929,
            PieceType::King => 60000,
        }
    }

    pub fn directions(self) -> Vec<Vec<Direction>> {
        use Direction::*;
        use PieceType::*;

        match self {
            Pawn => vec![vec![Up], vec![Up, Up], vec![Up, Left], vec![Up, Right]],
            Knight => vec![
                vec![Up, Up, Left],
                vec![Up, Up, Right],
                vec![Left, Left, Up],
                vec![Left, Left, Down],
                vec![Right, Right, Up],
                vec![Right, Right, Down],
                vec![Down, Down, Right],
                vec![Down, Down, Left],
            ],
            Bishop => vec![
                vec![Up, Left],
                vec![Up, Right],
                vec![Down, Left],
                vec![Down, Right],
            ],
            Rook => vec![vec![Up], vec![Down], vec![Left], vec![Right]],
            Queen | King => vec![
                vec![Up],
                vec![Up, Left],
                vec![Up, Right],
                vec![Left],
                vec![Right],
                vec![Down],
                vec![Down, Left],
                vec![Down, Right],
            ],
        }
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PieceType::Pawn => "P",
                PieceType::Knight => "N",
                PieceType::Bishop => "B",
                PieceType::Rook => "R",
                PieceType::Queen => "Q",
                PieceType::King => "K",
            }
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub kind: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn flip(&self) -> Self {
        Piece {
            color: self.color.inverse(),
            kind: self.kind,
        }
    }

    /* TODO: passing the whole board and square
        to a piece is cringe. Use some pattern like Strategy maybe?
    */
    pub fn moves(&self, board: &Board, square: &Square) -> Vec<Move> {
        use PieceType::*;
        match self.kind {
            Pawn => self.pawn_moves(board, square),
            Knight => self.knight_moves(board, square),
            Bishop => self.bishop_moves(board, square),
            Rook => self.rook_moves(board, square),
            Queen => self.queen_moves(board, square),
            King => self.king_moves(board, square),
        }
    }

    fn pawn_moves(&self, board: &Board, square: &Square) -> Vec<Move> {
        use Direction::*;

        let mut moves = Vec::new();
        for direction in self.kind.directions() {
            let mut to_file: usize = 0;
            let mut to_rank: usize = 0;

            let white = self.color == Color::White;

            // only one move up satisfies
            if direction.len() == 1 {
                to_file = if white {
                    square.file + 1
                } else {
                    square.file - 1
                };
            } else {
                (to_file, to_rank) = match (direction[0], direction[1]) {
                    (Up, Up) => {
                        if white {
                            (square.file + 2, square.rank)
                        } else {
                            (square.file - 2, square.rank)
                        }
                    }
                    (Up, Left) => {
                        if white {
                            (square.file + 1, square.rank + 1)
                        } else {
                            (square.file - 1, square.rank - 1)
                        }
                    }
                    (Up, Right) => {
                        if white {
                            (square.file + 1, square.rank - 1)
                        } else {
                            (square.file - 1, square.rank + 1)
                        }
                    }
                    _ => (square.file, square.rank),
                }
            }

            let mv = Move {
                from: (square.file, square.rank),
                to: (to_file, to_rank),
            };
            // TODO: check that `to` square isn't occupied

            moves.push(mv);
        }
        vec![]
    }

    fn knight_moves(&self, board: &Board, square: &Square) -> Vec<Move> {
        let directions = self.kind.directions();
        for direction in directions {}
        vec![]
    }

    fn bishop_moves(&self, board: &Board, square: &Square) -> Vec<Move> {
        let directions = self.kind.directions();
        for direction in directions {}
        vec![]
    }

    fn rook_moves(&self, board: &Board, square: &Square) -> Vec<Move> {
        let directions = self.kind.directions();
        for direction in directions {}
        vec![]
    }

    fn queen_moves(&self, board: &Board, square: &Square) -> Vec<Move> {
        let directions = self.kind.directions();
        for direction in directions {}
        vec![]
    }

    fn king_moves(&self, board: &Board, square: &Square) -> Vec<Move> {
        let directions = self.kind.directions();
        for direction in directions {}
        vec![]
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let piece_letter_code = format!("{}", self.kind);

        let _ = write!(
            f,
            "{}",
            if self.color == Color::Black {
                piece_letter_code.to_lowercase()
            } else {
                piece_letter_code
            }
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::PieceType;

    #[test]
    fn piece_move_directions_are_valid() {
        dbg!(PieceType::Rook.directions());
    }
}
