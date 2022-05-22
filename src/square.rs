use crate::color::Color;
use crate::piece::Piece;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub struct Square {
    pub color: Color,
    pub piece: Option<Piece>,
    pub file: usize,
    pub rank: usize,
}

impl Square {
    pub fn new(color: Color, piece: Option<Piece>, file: usize, rank: usize) -> Self {
        Square {
            color,
            piece,
            file,
            rank,
        }
    }
}
impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            if let Some(piece) = self.piece {
                piece.to_string()
            } else if self.color == Color::White {
                String::from("O")
            } else {
                String::from("X")
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::piece::{Piece, PieceType};
    use crate::square::Square;

    #[test]
    fn squares_have_correct_string_representation() {
        let expected_str_representations = vec!["K", "O", "X", "q"];
        let squares = vec![
            Square {
                color: Color::White,
                piece: Some(Piece {
                    color: Color::White,
                    kind: PieceType::King,
                }),
                rank: 0,
                file: 0,
            },
            Square {
                color: Color::White,
                piece: None,
                rank: 0,
                file: 0,
            },
            Square {
                color: Color::Black,
                piece: None,
                rank: 0,
                file: 0,
            },
            Square {
                color: Color::White,
                piece: Some(Piece {
                    color: Color::Black,
                    kind: PieceType::Queen,
                }),
                rank: 0,
                file: 0,
            },
        ];

        for i in 0..squares.len() {
            assert_eq!(format!("{}", squares[i]), expected_str_representations[i])
        }
    }
}
