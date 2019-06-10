mod game;

use crate::game::board3::Board3;
use crate::game::search::Searcher;

fn main() {
    let board = Board3::new();
    let searcher = Searcher::new();
    searcher.search(board);
}