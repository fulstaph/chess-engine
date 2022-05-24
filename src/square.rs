use crate::color::Color;
use crate::direction::MoveOffset;
use crate::piece::Piece;
use std::fmt;
use std::fmt::{format, Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub struct Square {
    pub file: usize,
    pub rank: usize,
    pub piece: Option<Piece>,
}

impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        self.file == other.file && self.rank == other.rank
    }
}

impl Square {
    pub fn new(file: usize, rank: usize, piece: Option<Piece>) -> Self {
        Square { piece, file, rank }
    }

    pub fn get_color(&self) -> Color {
        if (self.file + self.rank) % 2 == 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn get_square_string(&self) -> String {
        let rank_letter = match self.rank {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => ' ',
        };

        let file_number = self.file + 1;

        format!("{}{}", rank_letter, file_number)
    }

    pub fn move_to(&self, offset: MoveOffset) -> Option<Self> {
        // check if the resulting square is in board bounds
        let file = self.file as i8 + offset.0;
        let rank = self.rank as i8 + offset.1;

        if !(0..8).contains(&file) || !(0..8).contains(&rank) {
            return None;
        }

        Some(Self {
            file: file as usize,
            rank: rank as usize,
            piece: None,
        })
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            if let Some(piece) = self.piece {
                piece.to_string()
            } else if self.get_color() == Color::White {
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
    use crate::color::Color::{Black, White};
    use crate::piece::PieceType::Bishop;
    use crate::piece::{Piece, PieceType};
    use crate::square::Square;

    #[test]
    fn trying_to_move_out_of_board_returns_none() {
        let test_square = Square {
            file: 0,
            rank: 0,
            piece: Some(Piece {
                kind: Bishop,
                color: Black,
            }),
        };

        assert!(test_square.move_to((-1, -1)).is_none());
    }

    #[test]
    fn squares_have_correct_string_representation() {
        let expected_str_representations = vec!["K", "O", "X", "q"];
        let squares = vec![
            Square {
                piece: Some(Piece {
                    color: Color::White,
                    kind: PieceType::King,
                }),
                rank: 0,
                file: 0,
            },
            Square {
                piece: None,
                rank: 0,
                file: 0,
            },
            Square {
                piece: None,
                rank: 1,
                file: 0,
            },
            Square {
                piece: Some(Piece {
                    color: Color::Black,
                    kind: PieceType::Queen,
                }),
                rank: 1,
                file: 0,
            },
        ];

        for i in 0..squares.len() {
            assert_eq!(format!("{}", squares[i]), expected_str_representations[i])
        }
    }
}
