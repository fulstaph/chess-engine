use crate::board::Board;
use crate::color::Color;
use crate::direction::DIRECTIONS;
use crate::r#move::Move;
use std::borrow::Borrow;

pub enum Castling {
    Queenside,
    Kingside,
}

/// Position holds state of the game at any given move
pub struct Position {
    pub board: Board,
    pub score: usize,
    turn: usize,
    pub white_castling: Vec<Castling>,
    pub black_castling: Vec<Castling>,
}

impl Position {
    pub fn turn(&self) -> Color {
        if self.turn % 2 == 1 {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn moves(&self) -> Vec<Move> {
        let mut moves = vec![];

        for (_, file) in self.board.files() {
            for square in file.iter() {
                dbg!(square);
                if let Some(piece) = square.piece {
                    let directions = &DIRECTIONS[piece.kind.borrow()];
                    dbg!(directions);

                    let mut piece_moves = vec![];
                    for direction in directions {
                        let new_sq = square.move_to(*direction);
                        dbg!(new_sq);

                        piece_moves.push(Move {
                            from: square.clone(),
                            to: new_sq,
                        });
                    }

                    moves.append(&mut piece_moves);
                }
            }
        }

        dbg!(&moves);
        moves
    }
}

impl Default for Position {
    fn default() -> Self {
        Position {
            board: Board::new(),
            score: 0,
            turn: 1,
            white_castling: vec![Castling::Kingside, Castling::Queenside],
            black_castling: vec![Castling::Kingside, Castling::Queenside],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::position::Position;

    #[test]
    fn all_first_moves_for_whites_are_correct() {
        Position::default().moves();
    }
}
