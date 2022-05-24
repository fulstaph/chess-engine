use crate::board::Board;
use crate::color::Color;
use crate::direction::DIRECTIONS;
use crate::piece::PieceType::Pawn;
use crate::r#move::{Move, Moves};
use log::debug;
use std::borrow::{Borrow, BorrowMut};

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

        let cur_color = self.turn();

        for square in self.board.inner.iter().flat_map(|arr| arr.iter()) {
            if let Some(piece) = square.piece {
                if piece.color != cur_color {
                    continue;
                }
                debug!(
                    "looking for moves for piece {} from square {}",
                    piece,
                    square.get_square_string()
                );

                let directions = &DIRECTIONS[piece.kind.borrow()];

                // find all possible squares that are on the board
                let mut piece_moves = vec![];
                for direction in directions {
                    if let Some(new_sq) = square.move_to(*direction) {
                        piece_moves.push(Move {
                            from: *square,
                            to: new_sq,
                        });
                    }
                }

                debug!("all possible moves: {}", Moves(piece_moves.clone()));

                let mut piece_moves = piece_moves
                    .into_iter()
                    // filter out moves to squares occupied by pieces of the same color
                    .filter(|mv| {
                        let to = self.board[[mv.to.file, mv.to.rank]];

                        if let Some(piece) = to.piece {
                            return piece.color != cur_color;
                        }

                        true
                    })
                    // filter out diagonal moves for pawns if there's no capture possibility
                    .filter(|mv| {
                        let to = self.board[[mv.to.file, mv.to.rank]];
                        // if diagonal move by pawn
                        if piece.kind == Pawn && (mv.to.rank as i8 - mv.from.rank as i8) == 0 {
                            return to.piece.is_none();
                        }
                        false
                    })
                    .collect();

                moves.append(&mut piece_moves);
            }
        }

        debug!("found {} moves: {}", moves.len(), Moves(moves.clone()));
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
    use log::info;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn all_first_moves_for_whites_are_correct() {
        init();
        info!("This record will be captured by `cargo test`");
        Position::default().moves();
    }
}
