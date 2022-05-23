use crate::board::Board;
use crate::r#move::Move;

/// Position holds state of the game at any given move
pub struct Position {
    pub board: Board,
    pub score: usize,
}

impl Position {
    pub fn moves(&self) -> Vec<Move> {
        let mut moves = vec![];

        for (_, file) in self.board.files() {
            for square in file.iter() {
                dbg!(square);
                if let Some(piece) = square.piece {
                    moves.append(&mut piece.kind.move_generator().moves(square, &self.board));
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
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn getting_correct_first_moves_for_whites() {}
}
