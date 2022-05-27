use crate::piece::PieceType;
use crate::square::Square;
use enum_iterator::IntoEnumIterator;
use std::fmt::{Display, Formatter};

/// `MovesFinder` returns all possible moves from a given square
pub struct MovesFinder {
    from: Square,
}

impl MovesFinder {
    pub fn list(&self) -> Moves {
        let mut moves = vec![];
        if let Some(offsets) = Direction::offsets(self.from) {
            for offset in offsets {
                moves.push(Move {
                    from: self.from,
                    to: offset,
                })
            }
        }

        Moves(moves)
    }

    pub fn new(from: Square) -> Self {
        Self { from }
    }
}

pub type MoveOffset = (i8, i8);

#[derive(Debug, PartialEq, IntoEnumIterator)]
pub enum Direction {
    Ray(RayDirection),
    Knight(KnightDirection),
}

impl Direction {
    pub fn offsets(sq: Square) -> Option<Vec<Square>> {
        let piece = sq.piece?;

        let mut offsets = vec![];
        if piece.kind == PieceType::Knight {
            for direction in KnightDirection::into_enum_iter() {
                if let Some(s) = sq.move_to(direction.offset()) {
                    offsets.push(s);
                }
            }
        } else {
            for offset in RayDirection::rays(piece.kind) {
                if let Some(s) = sq.move_to(offset) {
                    offsets.push(s);
                }
            }
        }

        Some(offsets)
    }
}

#[derive(Debug, PartialEq, IntoEnumIterator)]
pub enum KnightDirection {
    NorthNorthWest,
    NorthNorthEast,
    NorthEastEast,
    SouthEastEast,
    SouthSouthEast,
    SouthSouthWest,
    SouthWestWest,
    NorthWestWest,
}

impl KnightDirection {
    fn offset(&self) -> MoveOffset {
        use KnightDirection::*;
        match self {
            NorthNorthWest => (2, 1),
            NorthNorthEast => (2, -1),
            NorthEastEast => (1, -2),
            SouthEastEast => (-1, -2),
            SouthSouthEast => (-2, -1),
            SouthSouthWest => (-2, 1),
            SouthWestWest => (-1, 2),
            NorthWestWest => (1, 2),
        }
    }
}

#[derive(Debug, PartialEq, IntoEnumIterator)]
pub enum RayDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl RayDirection {
    fn multiplier(pt: PieceType) -> i8 {
        match pt {
            PieceType::Pawn => 2,
            PieceType::King | PieceType::Knight => 1,
            PieceType::Bishop | PieceType::Rook | PieceType::Queen => 8,
        }
    }

    fn offset(&self) -> MoveOffset {
        use RayDirection::*;
        match self {
            North => (1, 0),
            NorthEast => (1, 1),
            East => (0, 1),
            SouthEast => (-1, 1),
            South => (-1, 0),
            SouthWest => (-1, -1),
            West => (0, -1),
            NorthWest => (1, -1),
        }
    }

    pub fn rays(piece: PieceType) -> Vec<MoveOffset> {
        let mut res = vec![];
        for direction in RayDirection::into_enum_iter() {
            let offset = direction.offset();
            for mult in 1..=RayDirection::multiplier(piece) {
                res.push((mult * offset.0, mult * offset.1));
            }
        }

        res
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{}",
            self.from.get_square_string(),
            self.to.get_square_string()
        )
    }
}

pub struct Moves(pub Vec<Move>);
impl Display for Moves {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .fold(Ok(()), |result, mv| {
                result.and_then(|_| write!(f, "{} ", mv))
            })
            .expect("TODO: panic message");
        Ok(())
    }
}

#[cfg(test)]
mod test {
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
    fn rook_move_directions_are_correct() {}

    #[test]
    fn bishop_move_directions_are_correct() {}
}
