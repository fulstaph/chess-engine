use crate::piece::{Piece, PieceType};
use crate::square::Square;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub type MoveOffset = (i8, i8);

lazy_static! {
    static ref DIRECTIONS: HashMap<PieceType, Vec<MoveOffset>> = {
        let mut map = HashMap::new();

        map.insert(PieceType::Pawn, vec![(1, 0), (2, 0), (1, 1), (1, -1)]);
        map.insert(
            PieceType::Knight,
            vec![
                (1, 2),
                (1, -2),
                (2, 1),
                (2, -1),
                (-1, 2),
                (-1, -2),
                (-2, 1),
                (-2, -1),
            ],
        );
        map.insert(PieceType::Bishop, vec![(1, 1), (1, -1), (-1, 1), (-1, -1)]);
        map.insert(PieceType::Rook, vec![(1, 0), (0, 1), (-1, 0), (0, -1)]);
        map.insert(
            PieceType::Queen,
            vec![
                (1, 0),
                (1, 1),
                (1, -1),
                (0, 1),
                (0, -1),
                (-1, 0),
                (-1, 1),
                (-1, -1),
            ],
        );
        map.insert(
            PieceType::King,
            vec![
                (1, 0),
                (1, 1),
                (1, -1),
                (0, 1),
                (0, -1),
                (-1, 0),
                (-1, 1),
                (-1, -1),
            ],
        );

        map
    };
}
