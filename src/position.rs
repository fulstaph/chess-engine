use crate::board::Board;
use crate::color::Color;
use crate::r#move::Move;

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
                    // moves.append(&mut piece.kind.move_generator().moves(square, self));
                }
            }
        }

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
    #[test]
    fn getting_correct_first_moves_for_whites() {}
}
