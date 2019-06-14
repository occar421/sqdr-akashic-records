use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use crate::game::commons::{GameResult, Code, Board, Turn};

#[derive(Debug)]
pub struct BoardRelation {
    pub game_result: GameResult,
    pub next_boards: Rc<RefCell<Vec<Code>>>,
}

pub struct Analyzer {
    map: RefCell<HashMap<Code, BoardRelation>>
}

impl Analyzer {
    pub fn new() -> Analyzer {
        Analyzer {
            map: RefCell::new(HashMap::new()),
        }
    }

    pub fn analyze<B>(self: &Self, board: &B) -> GameResult where B: Board {
        let first_board_code = board.encode();

        self.search(board);

        self.solve::<B>(&first_board_code)
    }

    fn search<B>(self: &Self, current_board: &B) where B: Board {
        let code = current_board.encode();
        let next_boards = Rc::new(RefCell::new(Vec::new()));

        self.map.borrow_mut().insert(code.clone(), BoardRelation { game_result: GameResult::Unknown, next_boards: next_boards.clone() });

        for i in 0..B::get_board_size() {
            let child_code = self._search(current_board, i);
            if let Some(child_code) = child_code {
                next_boards.try_borrow_mut().unwrap().push(child_code);
            }
        }
    }

    fn _search<B>(self: &Self, current_board: &B, piece_index: usize) -> Option<Code> where B: Board {
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

                map.insert(board.encode(), BoardRelation { game_result: result, next_boards: next_boards.clone() });
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
            let relation = map.get_mut(&code).unwrap();
            relation.next_boards.try_borrow_mut().unwrap().extend_from_slice(&codes);

            return Option::Some(code);
        } else {
            return Option::None;
        };
    }

    fn solve<B>(self: &Self, board_code: &Code) -> GameResult where B: Board {
        let mut next_boards;

        // check if already memoized
        {
            let map = self.map.borrow();
            let relation = map.get(board_code);

            if let Some(relation) = relation {
                if relation.game_result != GameResult::Unknown {
                    return relation.game_result;
                }

                next_boards = { relation.next_boards.borrow().clone() };
            } else {
                return GameResult::Unknown;
            }
        };

        // calc
        let mut results = next_boards.iter().map(|b| self.solve::<B>(b));
        let current_turn = board_code.get_turn::<B>();

        // if some record is invalid or unknown, 2-value is not proper.
        let red_wins = if current_turn == Turn::Red {
            results.any(|r| r == GameResult::RedWins)
        } else {
            results.all(|r| r == GameResult::YellowWins)
        };
        let yellow_wins = if current_turn == Turn::Red {
            results.all(|r| r == GameResult::YellowWins)
        } else {
            results.any(|r| r == GameResult::RedWins)
        };

        let result = match (red_wins, yellow_wins) {
            (false, false) => GameResult::Unknown,
            (true, false) => GameResult::RedWins,
            (false, true) => GameResult::YellowWins,
            (true, true) => GameResult::Invalid
        };

        // memoization
        self.map.borrow_mut().get_mut(board_code).unwrap().game_result = result;

        return result;
    }
}