use crate::color::Color;
use crate::direction::MoveOffset;
use crate::piece::PieceType;
use crate::position::Position;
use crate::square::Square;

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}

#[cfg(test)]
mod tests {
    #[test]
    fn kings_pawn_first_moves_are_correct() {}
}
