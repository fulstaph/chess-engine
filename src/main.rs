use engine::position::Position;
use engine::*;
use log::info;

fn main() {
    env_logger::init();

    info!("initial board state:\n{}\n", board::Board::new());

    Position::default().moves();
}
