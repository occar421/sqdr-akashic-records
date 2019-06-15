use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashMap, HashSet};
use crate::game::commons::{GameResult, Code, Board, Turn};
use std::marker::PhantomData;
use serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Debug)]
pub struct AnalysisTreeNode {
    game_result: GameResult,
    next_boards: Rc<RefCell<Vec<Code>>>,
}

impl Serialize for AnalysisTreeNode {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut state = serializer.serialize_struct("AnalysisTreeNode", 2)?;
        state.serialize_field("result", &self.game_result)?;
        state.serialize_field("next", &self.next_boards.borrow().to_vec())?;
        state.end()
    }
}

impl Serialize for GameResult {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str(self.to_string())
    }
}

impl GameResult {
    fn to_string(self: &Self) -> &str {
        match self {
            GameResult::Unknown => "unknown",
            GameResult::RedWins => "red",
            GameResult::YellowWins => "yellow",
            GameResult::Drawn => "drawn",
            GameResult::Invalid => "invalid"
        }
    }
}

impl Serialize for Code {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str(&self.0)
    }
}

pub struct Analyzer<B: Board> {
    map: RefCell<HashMap<Code, AnalysisTreeNode>>,
    checked_set: RefCell<HashSet<Code>>,
    _marker: PhantomData<fn() -> B>,
}

impl<B> Analyzer<B> where B: Board {
    pub fn new() -> Analyzer<B> {
        Analyzer {
            map: RefCell::new(HashMap::new()),
            checked_set: RefCell::new(HashSet::new()),
            _marker: PhantomData,
        }
    }

    pub fn analyze(self: &Self, board: &B) -> GameResult {
        println!("Start searching leaves.");

        let first_board_code = board.encode();

        self.search(board);

        println!("Finish searching leaves.");

        println!("Start solving.");

        let result = self.solve(&first_board_code);

        println!("Finish solving.");

        return result;
    }

    fn search(self: &Self, current_board: &B) {
        let code = current_board.encode();
        let next_boards = Rc::new(RefCell::new(Vec::new()));

        self.map.borrow_mut().insert(code.clone(), AnalysisTreeNode { game_result: GameResult::Unknown, next_boards: next_boards.clone() });

        for i in 0..B::get_board_size() {
            let child_code = self._search(current_board, i);
            if let Some(child_code) = child_code {
                next_boards.borrow_mut().push(child_code);
            }
        }
    }

    fn _search(self: &Self, current_board: &B, piece_index: usize) -> Option<Code> {
        let next = current_board.move_at(piece_index);

        if let Some(board) = next {
            let code = board.encode();
            let result = board.get_result();
            {
                let mut map = self.map.borrow_mut();

                if map.contains_key(&code) {
                    return Option::Some(code);
                }


                let next_boards = Rc::new(RefCell::new(Vec::new()));

                map.insert(board.encode(), AnalysisTreeNode { game_result: result, next_boards: next_boards.clone() });
            }

            if result != GameResult::Unknown {
                // leaf
                return Option::Some(code);
            }

            let mut codes = Vec::new();

            for i in 0..B::get_board_size() {
                let child_code = self._search(&board, i);
                if let Some(child_code) = child_code {
                    codes.push(child_code);
                }
            }

            let mut map = self.map.borrow_mut();
            let tree_node = map.get_mut(&code).unwrap(); // pick already inserted value
            tree_node.next_boards.borrow_mut().extend_from_slice(&codes);

            return Option::Some(code);
        } else {
            return Option::None;
        };
    }

    fn solve(self: &Self, board_code: &Code) -> GameResult {
        let mut next_boards;
        // check if already memoized in map
        {
            let mut map = self.map.borrow_mut();
            let tree_node = map.get_mut(board_code);

            if let Some(tree_node) = tree_node {
                if tree_node.game_result != GameResult::Unknown {
                    return tree_node.game_result;
                }
                // game_result is Unknown

                if self.checked_set.borrow().contains(board_code) {
                    // cyclic part
                    let result = GameResult::Drawn;
                    tree_node.game_result = result;
                    return result;
                }

                next_boards = { tree_node.next_boards.borrow().clone() };
            } else {
                return GameResult::Unknown;
            }
        };

        {
            self.checked_set.borrow_mut().insert(board_code.clone());
        }

        // calc
        let results: Vec<_> = next_boards.iter().map(|b| self.solve(b)).collect(); // because side effect function
        let current_turn = board_code.get_turn::<B>();

        let (win_turn, win_opposite) = if current_turn == Turn::Red { (GameResult::RedWins, GameResult::YellowWins) } else { (GameResult::YellowWins, GameResult::RedWins) };
        let result =
            if results.iter().any(|r| *r == win_turn) {
                win_turn
            } else if results.iter().any(|r| *r == GameResult::Drawn) {
                GameResult::Drawn
            } else {
                win_opposite
            };

        // memoization
        self.map.borrow_mut().get_mut(board_code).unwrap().game_result = result; // pick already inserted value

        return result;
    }

    pub fn emit_map_as_json(self: &Self) -> serde_json::Result<String> {
        serde_json::to_string(&self.map)
    }

    pub fn emit_nodes_and_links(self: &Self) -> (Vec<(String, String)>, Vec<(String, String)>) {
        let mut nodes = Vec::new();
        let mut links = Vec::new();

        for (k, v) in self.map.borrow().iter() {
            nodes.push((k.0.clone(), v.game_result.to_string().to_string()));
            for n in v.next_boards.borrow().iter() {
                links.push((k.0.clone(), n.0.clone()));
            }
        }

        return (nodes, links);
    }
}