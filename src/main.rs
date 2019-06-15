mod game;

use crate::game::board3::Board3;
use crate::game::analysis::Analyzer;
use crate::game::commons::Turn;
use std::path::Path;

fn main() {
    let board = Board3::new(Turn::Red);
    let analyzer = Analyzer::new();
    let result = analyzer.analyze(&board);
    println!("3x3 Red first -> {}", result);

    let board = Board3::new(Turn::Yellow);
    let result = analyzer.analyze(&board); // reuse "cache"
    println!("3x3 Yellow first -> {}", result);

    let json_content = analyzer.emit_map_as_json().expect("Invalid structure.");
    let path = Path::new("./results/board3.json");

    std::fs::write(path, json_content).expect("Failed to write file.");

    println!("Finish writing to json file.");
}