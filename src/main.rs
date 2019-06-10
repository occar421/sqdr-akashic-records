mod game;

use crate::game::board3::Board3;
use crate::game::analysis::Analyzer;

fn main() {
    let board = Board3::new();
    let searcher = Analyzer::new();
    searcher.analyze(board);
}