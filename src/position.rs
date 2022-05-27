use crate::board::Board;
use crate::color::Color;
use crate::direction::{Move, Moves, MovesFinder};
use crate::piece::PieceType;
use log::debug;
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
        if self.turn % 2 == 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    // returns all valid moves
    pub fn moves(&self) -> Moves {
        let mut moves = vec![];

        let cur_color = self.turn();

        for square in self.board.flattened_iter() {
            let mut all_possible_moves = MovesFinder::new(*square).list();
            debug!("all possible moves {}", all_possible_moves);
        }

        Moves(moves)
    }

    pub fn from_fen(fen: &str) -> Self {
        Position::default()
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
    use crate::board::Board;
    use crate::color::Color;
    use crate::direction::Moves;
    use crate::piece::{Piece, PieceType};
    use crate::position::Position;
    use crate::square::Square;
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

    #[test]
    fn pawns_cant_move_two_squares_from_non_home_rows() {
        let mut board = Board::default();

        board.inner[2][0].piece = Some(Piece {
            kind: PieceType::Pawn,
            color: Color::White,
        });

        println!(
            "put white pawn on square {}",
            board.inner[2][0].get_square_string()
        );
        println!("board:\n{}", board);

        let p = Position {
            board,
            score: 0,
            turn: 1,
            white_castling: vec![],
            black_castling: vec![],
        };

        let moves = p.moves();

        println!("{}", moves);
    }
}
