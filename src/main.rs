mod game;

use crate::game::board3::Board3;
use crate::game::analysis::Analyzer;
use crate::game::commons::Turn;
//use std::path::Path;
use std::fs::File;
use std::io::{LineWriter, Write};
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

    let dump = analyzer.emit_nodes_and_links();
    let path = File::create("./results/nodes3.csv").expect("Failed to create nodes file.");
    let mut file = LineWriter::new(path);

    for node in dump.0 {
        writeln!(file, "{},{}", node.0, node.1).expect("Failed to write a line.");
    }

    file.flush().expect("Failed to flush nodes file");

    let path = File::create("./results/links3.csv").expect("Failed to create links file.");
    let mut file = LineWriter::new(path);

    for link in dump.1 {
        writeln!(file, "{},{}", link.0, link.1).expect("Failed to write a line.");
    }

    file.flush().expect("Failed to flush links file");

    println!("Finish writing to json file.");
}