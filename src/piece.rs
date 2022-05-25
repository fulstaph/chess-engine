use crate::color::Color;
use core::fmt::{self, Display};
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
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

    pub fn str(&self) -> &str {
        match (self.kind, self.color) {
            (PieceType::Pawn, Color::White) => "\u{2659}",
            (PieceType::Pawn, Color::Black) => "\u{265F}",
            (PieceType::Rook, Color::White) => "\u{2656}",
            (PieceType::Rook, Color::Black) => "\u{265C}",
            (PieceType::Knight, Color::White) => "\u{2658}",
            (PieceType::Knight, Color::Black) => "\u{265E}",
            (PieceType::Bishop, Color::White) => "\u{2657}",
            (PieceType::Bishop, Color::Black) => "\u{265D}",
            (PieceType::Queen, Color::White) => "\u{2655}",
            (PieceType::Queen, Color::Black) => "\u{265B}",
            (PieceType::King, Color::White) => "\u{2654}",
            (PieceType::King, Color::Black) => "\u{265A}",
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str())
    }
    // fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    //     let piece_letter_code = format!("{}", self.kind);
    //
    //     let _ = write!(
    //         f,
    //         "{}",
    //         if self.color == Color::Black {
    //             piece_letter_code.to_lowercase()
    //         } else {
    //             piece_letter_code
    //         }
    //     );
    //
    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {}
