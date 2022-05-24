use crate::piece::{Piece, PieceType};
use crate::square::Square;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub type MoveOffset = (i8, i8);

lazy_static! {
    pub static ref DIRECTIONS: HashMap<PieceType, Vec<MoveOffset>> = {
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

#[cfg(test)]
mod test {
    use crate::color::Color;
    use crate::direction::DIRECTIONS;
    use crate::piece::{Piece, PieceType};
    use crate::square::Square;
    use std::collections::HashSet;

    // TODO: refactor to use table driven testing here.
    #[test]
    fn pawn_move_directions_are_correct() {}

    #[test]
    fn knight_move_directions_are_correct() {}

    #[test]
    fn queen_move_directions_are_correct() {}

    #[test]
    fn king_move_directions_are_correct() {}

    #[test]
    fn rook_move_directions_are_correct() {
        let rook_sq = Square::new(
            3,
            4,
            Some(Piece {
                kind: PieceType::Rook,
                color: Color::White,
            }),
        );

        println!("white rook on the square: {}", rook_sq.get_square_string());

        let rook_directions = &DIRECTIONS[&PieceType::Rook];

        let mut possible_squares = vec![];

        for direction in rook_directions {
            let res_sq = rook_sq.move_to(*direction);
            println!("can move to square: {}", res_sq.get_square_string());

            possible_squares.push(res_sq);
        }

        assert_eq!(
            possible_squares
                .iter()
                .map(|sq| sq.get_square_string())
                .collect::<Vec<String>>()
                .iter()
                .collect::<HashSet<_>>(),
            Vec::from(vec![
                "e5".to_string(),
                "f4".to_string(),
                "e3".to_string(),
                "d4".to_string()
            ])
            .iter()
            .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn bishop_move_directions_are_correct() {
        let mut expected_directions = Vec::new();
        expected_directions.push(String::from("d5"));
        expected_directions.push(String::from("d3"));
        expected_directions.push(String::from("f5"));
        expected_directions.push(String::from("f3"));

        let bishop_sq = Square::new(
            3,
            4,
            Some(Piece {
                kind: PieceType::Bishop,
                color: Color::White,
            }),
        );

        println!(
            "white bishop on the square: {}",
            bishop_sq.get_square_string()
        );

        let rook_directions = &DIRECTIONS[&PieceType::Bishop];

        let mut possible_squares = vec![];

        for direction in rook_directions {
            let res_sq = bishop_sq.move_to(*direction);
            println!("can move to square: {}", res_sq.get_square_string());

            possible_squares.push(res_sq);
        }

        assert_eq!(
            possible_squares
                .iter()
                .map(|sq| sq.get_square_string())
                .collect::<Vec<String>>()
                .iter()
                .collect::<HashSet<_>>(),
            expected_directions.iter().collect::<HashSet<_>>()
        );
    }
}
