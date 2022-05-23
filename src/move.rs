use crate::color::Color;
use crate::piece::PieceType;
use crate::position::Position;
use crate::square::Square;

// TODO: passing both `square` and whole game `state` is uber cringe.
pub trait MoveGenerator {
    fn moves(&self, square: &Square, state: &Position) -> Vec<Move>;
}

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}

// impl MoveGenerator for Pawn {
//     fn moves(&self, square: &Square, state: &Position) -> Vec<Move> {
//
//         let mut moves = Vec::new();
//         for direction in PieceType::Pawn(*self).directions() {
//             let mut to_file: usize = square.file;
//             let mut to_rank: usize = square.rank;
//
//             let is_white = square.piece.unwrap().color == Color::White;
//
//             // only one move up satisfies
//             if direction.len() == 1 {
//                 to_file = if is_white {
//                     square.file + 1
//                 } else {
//                     square.file - 1
//                 };
//             } else {
//                 (to_file, to_rank) = match (direction[0], direction[1]) {
//                     (Up, Up) => {
//                         if is_white {
//                             (square.file + 2, square.rank)
//                         } else {
//                             (square.file - 2, square.rank)
//                         }
//                     }
//                     (Up, Left) => {
//                         if is_white {
//                             (square.file + 1, square.rank + 1)
//                         } else {
//                             (square.file - 1, square.rank - 1)
//                         }
//                     }
//                     (Up, Right) => {
//                         if is_white {
//                             (square.file + 1, square.rank - 1)
//                         } else {
//                             (square.file - 1, square.rank + 1)
//                         }
//                     }
//                     _ => (square.file, square.rank),
//                 }
//             }
//
//             let mv = Move {
//                 from: *square,
//                 to: Square {
//                     file: to_file,
//                     rank: to_rank,
//                     piece: None,
//                 },
//             };
//
//             // TODO:
//             //      [] check if the move is 1st to skip `(Up, Up)` case
//             //      [] check that `to` square isn't occupied, unless an attack is possible
//             //      [] en passant but who cares
//             //      [] pawn promotion on opposite ranks
//
//             moves.push(mv);
//         }
//
//         moves
//     }
// }
//

#[cfg(test)]
mod tests {
    #[test]
    fn kings_pawn_first_moves_are_correct() {}
}
