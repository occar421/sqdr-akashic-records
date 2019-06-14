mod game;

use crate::game::board3::Board3;
use crate::game::analysis::Analyzer;
use crate::game::commons::Turn;

fn main() {
    let board = Board3::new(Turn::Red);
    let searcher = Analyzer::new();
    let result = searcher.analyze(&board);
    println!("3x3 Red first -> {}", result);

    let board = Board3::new(Turn::Yellow);
    let searcher = Analyzer::new();
    let result = searcher.analyze(&board);
    println!("3x3 Yellow first -> {}", result);
}