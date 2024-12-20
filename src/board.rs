use crate::color::Color;
use crate::piece::{Piece, PieceType};
use crate::square::Square;
use std::fmt;
use std::fmt::Display;

// constants used to arrange white pieces
const FIRST_RANK_INDEX: usize = 0;
const SECOND_RANK_INDEX: usize = 1;
// constants used to arrange black pieces
const SEVENTH_RANK_INDEX: usize = 6;
const EIGHT_RANK_INDEX: usize = 7;

#[derive(Debug, Clone)]
pub struct Board {
    inner: [[Square; 8]; 8],
}

impl Board {
    // TODO: try to shorten this
    pub fn new() -> Board {
        let mut board = Self::default();

        let white_rank_pieces = vec![
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        let black_rank_pieces = vec![
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        for file in 0..board.inner.len() {
            board.inner[FIRST_RANK_INDEX][file].piece = Some(Piece {
                color: Color::White,
                kind: white_rank_pieces[file],
            });
            board.inner[SECOND_RANK_INDEX][file].piece = Some(Piece {
                color: Color::White,
                kind: PieceType::Pawn,
            });

            board.inner[SEVENTH_RANK_INDEX][file].piece = Some(Piece {
                color: Color::Black,
                kind: PieceType::Pawn,
            });
            board.inner[EIGHT_RANK_INDEX][file].piece = Some(Piece {
                color: Color::Black,
                kind: black_rank_pieces[file],
            });
        }

        board
    }

    pub fn flip(&self) -> Board {
        let mut flipped = Self::default();

        for (i, file) in self.inner.iter().rev().enumerate() {
            for (j, square) in file.iter().rev().enumerate() {
                flipped.inner[i][j] = *square;
            }
        }

        flipped
    }

    pub fn files(&self) -> impl Iterator<Item = (usize, &[Square; 8])> {
        self.inner
            .iter()
            .enumerate()
            .map(|(idx, square)| (idx, square))
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board {
            inner: [[Square {
                color: Color::White,
                piece: None,
                rank: 0,
                file: 0,
            }; 8]; 8],
        };

        for file in 0..board.inner.len() {
            for rank in 0..board.inner.len() {
                board.inner[file][rank] = Square {
                    color: if (file + rank) % 2 == 0 {
                        Color::White
                    } else {
                        Color::Black
                    },
                    piece: None,
                    rank,
                    file,
                }
            }
        }

        board
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, rank) in self.inner.iter().enumerate() {
            for square in rank {
                write!(f, "{}", square).expect("TODO: panic message");
            }

            if i != EIGHT_RANK_INDEX {
                writeln!(f).expect("TODO: panic message");
            }
        }

        Ok(())
    }
}

impl Copy for Board {}

#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn empty_board_has_correct_string_representation() {
        let empty_board_expected =
            r#"OXOXOXOX|XOXOXOXO|OXOXOXOX|XOXOXOXO|OXOXOXOX|XOXOXOXO|OXOXOXOX|XOXOXOXO"#
                .to_string();

        assert_eq!(
            empty_board_expected,
            Board::default()
                .to_string()
                .split('\n')
                .map(String::from)
                .collect::<Vec<String>>()
                .join("|")
        )
    }

    #[test]
    fn initial_board_state_has_correct_string_representation() {
        let board_initial_state =
            r#"RNBQKBNR|PPPPPPPP|OXOXOXOX|XOXOXOXO|OXOXOXOX|XOXOXOXO|pppppppp|rnbqkbnr"#
                .to_string();

        assert_eq!(
            board_initial_state,
            Board::new()
                .to_string()
                .split('\n')
                .map(String::from)
                .collect::<Vec<String>>()
                .join("|")
        );
    }

    #[test]
    fn board_is_flipped_correctly() {
        let board_reversed =
            r#"RNBQKBNR|PPPPPPPP|OXOXOXOX|XOXOXOXO|OXOXOXOX|XOXOXOXO|pppppppp|rnbqkbnr"#
                .to_string()
                .chars()
                .rev()
                .collect::<String>();

        let board_flipped = Board::new().flip();
        println!("{}", board_flipped);

        assert_eq!(
            board_reversed,
            board_reversed
                .to_string()
                .split('\n')
                .map(String::from)
                .collect::<Vec<String>>()
                .join("|")
        )
    }
}
