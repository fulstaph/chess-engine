use core::fmt::{self, Display};

enum PieceType {
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
            Pawn => 100,
            Knight => 280,
            Bishop => 320,
            Rook => 479,
            Queen => 929,
            King => 60000,
        }
    }
}

enum Color {
    White,
    Black,
}

struct Piece {
    kind: PieceType,
    color: Color,
}

struct Square {
    color: Color,
    piece: Option<Piece>,
}

impl Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let square_str_repr = match self.color {
            White => match self.piece {
                None => ".",
                Some(piece) => match piece {
                    Pawn => "P",
                    Knight => "K",
                    Bishop => "B",
                    Rook => "R",
                    Queen => "Q",
                    King => "K",
                },
            },
            Black => match self.piece {
                None => ".",
                Some(piece) => match piece {
                    Pawn => "p",
                    Knight => "k",
                    Bishop => "b",
                    Rook => "r",
                    Queen => "q",
                    King => "k",
                },
            },
        };

        write!(f, "{}", square_str_repr)
    }
}

#[derive(Debug, Copy, Clone)]
struct Board {
    inner: [[Square; 8]; 8],
}

impl Board {
    pub fn new() -> Board {
        Board {
            inner: [[Square {
                color: Color::White,
                piece: None,
            }; 8]; 8],
        }
    }
}

impl Copy for Board {}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rank in &self.inner {
            for square in rank {
                write!(f, "{} ", square);
            }
            writeln!(f);
        }
        Ok(())
    }
}
// Position holds state of the game at the given move
struct Position {}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::Color;
    use super::Piece;
    use super::PieceType;
    use super::Square;

    #[test]
    fn squares_have_correct_string_representation() {
        let expected_str_representations = vec!["K"];
        let squares = vec![Square {
            color: Color::White,
            piece: Some(Piece {
                color: Color::White,
                kind: PieceType::King,
            }),
        }];

        for i in 0..squares.len() {
            assert_eq!(format!("{}", squares[i]), expected_str_representations[i])
        }
    }
}
