mod game;

use crate::game::board3::Board3;
use crate::game::analysis::Analyzer;
use crate::game::commons::Turn;

fn main() {
    let board = Board3::new(Turn::Red);
    let searcher = Analyzer::new();
    searcher.analyze(&board);
}