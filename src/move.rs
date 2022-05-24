use crate::square::Square;
use std::fmt::{Display, Formatter};

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
mod tests {
    #[test]
    fn kings_pawn_first_moves_are_correct() {}
}
