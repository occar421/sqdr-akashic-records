use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use crate::game::commons::{GameResult, Code, Board};

pub struct BoardRelation {
    pub game_result: GameResult,
    pub previous_boards: Vec<Code>,
    pub next_boards: Rc<RefCell<Vec<Code>>>,
}

pub struct Searcher {
    map: RefCell<HashMap<Code, BoardRelation>>,
}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher {
            map: RefCell::new(HashMap::new())
        }
    }

    pub fn search<B>(self: &Self, board: B) where B: Board {
        let code = board.encode();
        let next_boards = Rc::new(RefCell::new(Vec::new()));

        self.map.borrow_mut().insert(code.clone(), BoardRelation { game_result: GameResult::Unknown, previous_boards: Vec::new(), next_boards: next_boards.clone() });

        for i in 0..B::get_board_size() {
            let child_code = self.search_recur(&board, i);
            if let Some(child_code) = child_code {
                next_boards.try_borrow_mut().unwrap().push(child_code);
            }
        }
    }

    fn search_recur<B>(self: &Self, current_board: &B, piece_index: usize) -> Option<Code> where B: Board {
//        println!("{:?}", current_board);

        let next = current_board.move_at(piece_index);

        let previous_code = current_board.encode();
        return if let Some(board) = next {
            let code = board.encode();
            if let Some(relation) = self.map.borrow_mut().get_mut(&code) {
                relation.previous_boards.push(previous_code);

                if relation.game_result != GameResult::Unknown {
                    return Option::Some(code);
                }

                for i in 0..B::get_board_size() {
                    let child_code = self.search_recur(&board, i);
                    if let Some(child_code) = child_code {
                        relation.next_boards.try_borrow_mut().unwrap().push(child_code);
                    }
                }
            } else {
                let result = board.get_result();

                let next_boards = Rc::new(RefCell::new(Vec::new()));

                self.map.borrow_mut().insert(board.encode(), BoardRelation { game_result: result, previous_boards: vec![previous_code], next_boards: next_boards.clone() });

                println!("{:?}", result);
                if result != GameResult::Unknown {
                    return Option::Some(code);
                }

                for i in 0..B::get_board_size() {
                    let child_code = self.search_recur(&board, i);
                    if let Some(child_code) = child_code {
                        next_boards.try_borrow_mut().unwrap().push(child_code);
                    }
                }
            }

            Option::Some(code)
        } else {
            Option::None
        };
    }
}