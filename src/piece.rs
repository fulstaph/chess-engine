use crate::color::Color;
use core::fmt::{self, Display};
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
