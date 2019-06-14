use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use crate::game::commons::{GameResult, Code, Board};

pub struct BoardRelation {
    pub game_result: GameResult,
    pub previous_boards: Vec<Code>,
    pub next_boards: Rc<RefCell<Vec<Code>>>,
}

pub struct Analyzer {
    map: RefCell<HashMap<Code, BoardRelation>>,
}

impl Analyzer {
    pub fn new() -> Analyzer {
        Analyzer {
            map: RefCell::new(HashMap::new())
        }
    }

    pub fn analyze<B>(self: &Self, board: &B) where B: Board {
        self.search(board);

        // TODO: pre-order previous_board: is necessary?

        // TODO: post-order game_result: collecting
    }

    fn search<B>(self: &Self, current_board: &B) where B: Board {
        let code = current_board.encode();
        let next_boards = Rc::new(RefCell::new(Vec::new()));

        self.map.borrow_mut().insert(code.clone(), BoardRelation { game_result: GameResult::Unknown, previous_boards: Vec::new(), next_boards: next_boards.clone() });

        for i in 0..B::get_board_size() {
            let child_code = self._search(current_board, i);
            if let Some(child_code) = child_code {
                next_boards.try_borrow_mut().unwrap().push(child_code);
            }
        }
    }

    fn _search<B>(self: &Self, current_board: &B, piece_index: usize) -> Option<Code> where B: Board {
        let next = current_board.move_at(piece_index);

        let previous_code = current_board.encode();
        if let Some(board) = next {
            let code = board.encode();
            {
                let mut map = self.map.borrow_mut();

                if let Some(relation) = map.get_mut(&code) {
                    relation.previous_boards.push(previous_code);

                    return Option::Some(code);
                }

                let result = board.get_result();

                let next_boards = Rc::new(RefCell::new(Vec::new()));

                map.insert(board.encode(), BoardRelation { game_result: result, previous_boards: vec![previous_code], next_boards: next_boards.clone() });

                if result != GameResult::Unknown {
                    return Option::Some(code);
                }
            }

            let mut codes = Vec::new();

            for i in 0..B::get_board_size() {
                let child_code = self._search(&board, i);
                if let Some(child_code) = child_code {
                    codes.push(child_code);
                }
            }

            let mut map = self.map.borrow_mut();
            let relation = map.get_mut(&code).unwrap();
            relation.next_boards.try_borrow_mut().unwrap().extend_from_slice(&codes);

            return Option::Some(code);
        } else {
            return Option::None;
        };
    }
}