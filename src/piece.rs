use crate::color::Color;
use crate::direction::Direction;
use crate::r#move::MoveGenerator;
use core::fmt::{self, Display};
use std::fmt::Formatter;

// Hack because for now Rust doesn't allow
// to implement traits on enum variants.
// And I haven't come up with a better idea *yet*.
#[derive(Debug, Copy, Clone)]
pub struct Pawn {}

#[derive(Debug, Copy, Clone)]
pub struct Knight {}

#[derive(Debug, Copy, Clone)]
pub struct Bishop {}

#[derive(Debug, Copy, Clone)]
pub struct Rook {}

#[derive(Debug, Copy, Clone)]
pub struct Queen {}

#[derive(Debug, Copy, Clone)]
pub struct King {}

#[derive(Debug, Copy, Clone)]
pub enum PieceType {
    Pawn(Pawn),
    Knight(Knight),
    Bishop(Bishop),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

impl PieceType {
    pub fn value(self) -> usize {
        match self {
            PieceType::Pawn(_) => 100,
            PieceType::Knight(_) => 280,
            PieceType::Bishop(_) => 320,
            PieceType::Rook(_) => 479,
            PieceType::Queen(_) => 929,
            PieceType::King(_) => 60000,
        }
    }

    pub fn move_generator(&self) -> Box<dyn MoveGenerator> {
        match self {
            PieceType::Pawn(_) => Box::new(Pawn {}),
            PieceType::Knight(_) => Box::new(Knight {}),
            PieceType::Bishop(_) => Box::new(Bishop {}),
            PieceType::Rook(_) => Box::new(Rook {}),
            PieceType::Queen(_) => Box::new(Queen {}),
            PieceType::King(_) => Box::new(King {}),
        }
    }

    pub fn directions(self) -> Vec<Vec<Direction>> {
        use Direction::*;

        match self {
            PieceType::Pawn(_) => vec![vec![Up], vec![Up, Up], vec![Up, Left], vec![Up, Right]],
            PieceType::Knight(_) => vec![
                vec![Up, Up, Left],
                vec![Up, Up, Right],
                vec![Left, Left, Up],
                vec![Left, Left, Down],
                vec![Right, Right, Up],
                vec![Right, Right, Down],
                vec![Down, Down, Right],
                vec![Down, Down, Left],
            ],
            PieceType::Bishop(_) => vec![
                vec![Up, Left],
                vec![Up, Right],
                vec![Down, Left],
                vec![Down, Right],
            ],
            PieceType::Rook(_) => vec![vec![Up], vec![Down], vec![Left], vec![Right]],
            PieceType::Queen(_) | PieceType::King(_) => vec![
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
                PieceType::Pawn(_) => "P",
                PieceType::Knight(_) => "N",
                PieceType::Bishop(_) => "B",
                PieceType::Rook(_) => "R",
                PieceType::Queen(_) => "Q",
                PieceType::King(_) => "K",
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
mod tests {}
