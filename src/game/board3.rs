use crate::game::commons::{Board, Position, Turn, GameResult, Code};

const BOARD_SIZE: usize = 3;

#[derive(Debug, Copy, Clone)]
pub struct Board3 {
    red_pieces: [Position; BOARD_SIZE],
    yellow_pieces: [Position; BOARD_SIZE],
    turn: Turn,
}

const RED_SPEEDS_OUTWARD: [u8; BOARD_SIZE] = [2, 1, 2];
const YELLOW_SPEEDS_OUTWARD: [u8; BOARD_SIZE] = [1, 2, 1];
const RED_SPEEDS_HOMEWARD: [u8; BOARD_SIZE] = [1, 2, 1];
const YELLOW_SPEEDS_HOMEWARD: [u8; BOARD_SIZE] = [2, 1, 2];

impl Board for Board3 {
    fn get_board_size() -> usize {
        BOARD_SIZE
    }

    fn move_at(self: &Self, piece_index: usize) -> Option<Self> {
        let mut cloned_self = self.clone();

        let (
            pieces_turn,
            pieces_opposite,
            speed_outward,
            speed_homeward,
            next_turn
        ) = if self.turn == Turn::Red {
            (&mut cloned_self.red_pieces, &mut cloned_self.yellow_pieces, &RED_SPEEDS_OUTWARD, &RED_SPEEDS_HOMEWARD, Turn::Yellow)
        } else {
            (&mut cloned_self.yellow_pieces, &mut cloned_self.red_pieces, &YELLOW_SPEEDS_OUTWARD, &YELLOW_SPEEDS_HOMEWARD, Turn::Red)
        };

        pieces_turn[piece_index] = match pieces_turn[piece_index] {
            Position::Outward(n) => {
                let base_moves = speed_outward[piece_index];
                let mut jumped_previously = false;

                let mut n_moves = 0;
                let mut path = n;
                while n_moves < base_moves || jumped_previously {
                    n_moves += 1;

                    if path >= BOARD_SIZE as u8 {
                        // reaches turning point
                        break;
                    }

                    let target_piece_index = path as usize;
                    let target_position = pieces_opposite[target_piece_index];
                    match target_position {
                        Position::Outward(m) if m == piece_index as u8 + 1 => {
                            pieces_opposite[target_piece_index] = Position::Outward(0);
                            jumped_previously = true;
                        }
                        Position::Homeward(m) if m == (BOARD_SIZE - piece_index) as u8 => {
                            pieces_opposite[target_piece_index] = Position::Homeward(0);
                            jumped_previously = true;
                        }
                        _ => { if jumped_previously { break; } }
                    }
                    path += 1;
                }
                if n + n_moves > BOARD_SIZE as u8 { Position::Homeward(0) } else { Position::Outward(n + n_moves) }
            }
            Position::Homeward(n) => {
                let base_moves = speed_homeward[piece_index];
                let mut jumped_previously = false;

                let mut n_moves = 0;
                let mut path = n;
                while n_moves < base_moves || jumped_previously {
                    n_moves += 1;

                    if path >= BOARD_SIZE as u8 {
                        // reaches finish point
                        break;
                    }

                    let target_piece_index = BOARD_SIZE - path as usize - 1;
                    let target_position = pieces_opposite[target_piece_index];
                    match target_position {
                        Position::Outward(m) if m == piece_index as u8 + 1 => {
                            pieces_opposite[target_piece_index] = Position::Outward(0);
                            jumped_previously = true;
                        }
                        Position::Homeward(m) if m == (BOARD_SIZE - piece_index) as u8 => {
                            pieces_opposite[target_piece_index] = Position::Homeward(0);
                            jumped_previously = true;
                        }
                        _ => { if jumped_previously { break; } }
                    }
                    path += 1;
                }
                if n + base_moves > BOARD_SIZE as u8 { Position::Finished } else { Position::Homeward(n + n_moves) }
            }
            Position::Finished => return Option::None
        };

        cloned_self.turn = next_turn;

        Option::Some(cloned_self)
    }

    fn encode(self: &Self) -> Code {
        Code(format!("r{}{}{}y{}{}{}t{}",
                     self.red_pieces[0],
                     self.red_pieces[1],
                     self.red_pieces[2],
                     self.yellow_pieces[0],
                     self.yellow_pieces[1],
                     self.yellow_pieces[2],
                     if self.turn == Turn::Red { "r" } else { "y" }
        ).to_string())
    }

    fn get_turn_from_code(code: &Code) -> Turn {
        if code.0.ends_with("r") { Turn::Red } else { Turn::Yellow }
    }

    fn get_result(self: &Self) -> GameResult {
        let is_red_finished = self.red_pieces.iter().filter(|&p| *p == Position::Finished).count() >= BOARD_SIZE - 1;
        let is_yellow_finished = self.yellow_pieces.iter().filter(|&p| *p == Position::Finished).count() >= BOARD_SIZE - 1;

        match (is_red_finished, is_yellow_finished) {
            (false, false) => GameResult::Unknown,
            (true, false) => GameResult::RedWins,
            (false, true) => GameResult::YellowWins,
            (true, true) => GameResult::Invalid
        }
    }
}

impl Board3 {
    pub fn new(the_first_move: Turn) -> Self {
        Board3 {
            red_pieces: [Position::Outward(0); BOARD_SIZE],
            yellow_pieces: [Position::Outward(0); BOARD_SIZE],
            turn: the_first_move,
        }
    }
}

#[cfg(test)]
mod tests {
    //           .   :   .
    //     +---+---+---+---+---+
    //     |===| h0| h0| h0|===|
    //     +---+---+---+---+---+
    // . 2 |o0f|   |   |   | h0| :
    //     +---+---+---+---+---+
    // : 1 |o0f|   |   |   | h0| .
    //     +---+---+---+---+---+
    // . 0 |o0f|   |   |   | h0| :
    //     +---+---+---+---+---+
    //     |===|o0f|o0f|o0f|===|
    //     +---+---+---+---+---+
    //           0   1   2
    //           :   .   :

    mod move_at {
        use super::super::Board3;
        use crate::game::commons::{Turn, Board, Position};

        #[test]
        fn move_r0o0() {
            let board = Board3::new(Turn::Red);

            let board = board.move_at(0).unwrap();

            assert_eq!(board.turn, Turn::Yellow);
            assert_eq!(board.red_pieces[0], Position::Outward(2));
            assert_eq!(board.red_pieces[1], Position::Outward(0));
            assert_eq!(board.red_pieces[2], Position::Outward(0));
            assert_eq!(board.yellow_pieces[0], Position::Outward(0));
            assert_eq!(board.yellow_pieces[1], Position::Outward(0));
            assert_eq!(board.yellow_pieces[2], Position::Outward(0));
        }

        #[test]
        fn move_y0o0() {
            let board = Board3::new(Turn::Yellow);

            let board = board.move_at(0).unwrap();

            assert_eq!(board.turn, Turn::Red);
            assert_eq!(board.red_pieces[0], Position::Outward(0));
            assert_eq!(board.red_pieces[1], Position::Outward(0));
            assert_eq!(board.red_pieces[2], Position::Outward(0));
            assert_eq!(board.yellow_pieces[0], Position::Outward(1));
            assert_eq!(board.yellow_pieces[1], Position::Outward(0));
            assert_eq!(board.yellow_pieces[2], Position::Outward(0));
        }

        #[test]
        fn move_r0o2() {
            let mut board = Board3::new(Turn::Red);

            board.red_pieces[0] = Position::Outward(2);

            let board = board.move_at(0).unwrap();

            assert_eq!(board.turn, Turn::Yellow);
            assert_eq!(board.red_pieces[0], Position::Homeward(0));
            assert_eq!(board.red_pieces[1], Position::Outward(0));
            assert_eq!(board.red_pieces[2], Position::Outward(0));
            assert_eq!(board.yellow_pieces[0], Position::Outward(0));
            assert_eq!(board.yellow_pieces[1], Position::Outward(0));
            assert_eq!(board.yellow_pieces[2], Position::Outward(0));
        }

        #[test]
        fn move_r0o3() {
            let mut board = Board3::new(Turn::Red);

            board.red_pieces[0] = Position::Outward(3);

            let board = board.move_at(0).unwrap();

            assert_eq!(board.turn, Turn::Yellow);
            assert_eq!(board.red_pieces[0], Position::Homeward(0));
            assert_eq!(board.red_pieces[1], Position::Outward(0));
            assert_eq!(board.red_pieces[2], Position::Outward(0));
            assert_eq!(board.yellow_pieces[0], Position::Outward(0));
            assert_eq!(board.yellow_pieces[1], Position::Outward(0));
            assert_eq!(board.yellow_pieces[2], Position::Outward(0));
        }

        #[test]
        fn move_r1o3() {
            let mut board = Board3::new(Turn::Red);

            board.red_pieces[1] = Position::Outward(3);

            let board = board.move_at(1).unwrap();

            assert_eq!(board.turn, Turn::Yellow);
            assert_eq!(board.red_pieces[0], Position::Outward(0));
            assert_eq!(board.red_pieces[1], Position::Homeward(0));
            assert_eq!(board.red_pieces[2], Position::Outward(0));
            assert_eq!(board.yellow_pieces[0], Position::Outward(0));
            assert_eq!(board.yellow_pieces[1], Position::Outward(0));
            assert_eq!(board.yellow_pieces[2], Position::Outward(0));
        }

        #[test]
        fn move_r0o0_then_jump_y0o1() {
            let mut board = Board3::new(Turn::Red);

            board.yellow_pieces[0] = Position::Outward(1);

            let board = board.move_at(0).unwrap();

            assert_eq!(board.turn, Turn::Yellow);
            assert_eq!(board.red_pieces[0], Position::Outward(2));
            assert_eq!(board.red_pieces[1], Position::Outward(0));
            assert_eq!(board.red_pieces[2], Position::Outward(0));
            assert_eq!(board.yellow_pieces[0], Position::Outward(0));
            assert_eq!(board.yellow_pieces[1], Position::Outward(0));
            assert_eq!(board.yellow_pieces[2], Position::Outward(0));
        }

        #[test]
        fn move_r0o0_then_jump_y1o1() {
            let mut board = Board3::new(Turn::Red);

            board.yellow_pieces[1] = Position::Outward(1);

            let board = board.move_at(0).unwrap();

            assert_eq!(board.turn, Turn::Yellow);
            assert_eq!(board.red_pieces[0], Position::Outward(3));
            assert_eq!(board.red_pieces[1], Position::Outward(0));
            assert_eq!(board.red_pieces[2], Position::Outward(0));
            assert_eq!(board.yellow_pieces[0], Position::Outward(0));
            assert_eq!(board.yellow_pieces[1], Position::Outward(0));
            assert_eq!(board.yellow_pieces[2], Position::Outward(0));
        }

        #[test]
        fn move_r1o0_then_jump_y0o2_and_y1o2() {
            let mut board = Board3::new(Turn::Red);

            board.yellow_pieces[0] = Position::Outward(2);
            board.yellow_pieces[1] = Position::Outward(2);

            let board = board.move_at(1).unwrap();

            assert_eq!(board.turn, Turn::Yellow);
            assert_eq!(board.red_pieces[0], Position::Outward(0));
            assert_eq!(board.red_pieces[1], Position::Outward(3));
            assert_eq!(board.red_pieces[2], Position::Outward(0));
            assert_eq!(board.yellow_pieces[0], Position::Outward(0));
            assert_eq!(board.yellow_pieces[1], Position::Outward(0));
            assert_eq!(board.yellow_pieces[2], Position::Outward(0));
        }

        #[test]
        fn move_r1o0_then_jump_y0o2_but_not_y2o2() {
            let mut board = Board3::new(Turn::Red);

            board.yellow_pieces[0] = Position::Outward(2);
            board.yellow_pieces[2] = Position::Outward(2);

            let board = board.move_at(1).unwrap();

            assert_eq!(board.turn, Turn::Yellow);
            assert_eq!(board.red_pieces[0], Position::Outward(0));
            assert_eq!(board.red_pieces[1], Position::Outward(2));
            assert_eq!(board.red_pieces[2], Position::Outward(0));
            assert_eq!(board.yellow_pieces[0], Position::Outward(0));
            assert_eq!(board.yellow_pieces[1], Position::Outward(0));
            assert_eq!(board.yellow_pieces[2], Position::Outward(2));
        }

        #[test]
        fn move_r1o0_then_jump_y0o2_and_y1o2_and_y2o2() {
            let mut board = Board3::new(Turn::Red);

            board.yellow_pieces[0] = Position::Outward(2);
            board.yellow_pieces[1] = Position::Outward(2);
            board.yellow_pieces[2] = Position::Outward(2);

            let board = board.move_at(1).unwrap();

            assert_eq!(board.turn, Turn::Yellow);
            assert_eq!(board.red_pieces[0], Position::Outward(0));
            assert_eq!(board.red_pieces[1], Position::Homeward(0));
            assert_eq!(board.red_pieces[2], Position::Outward(0));
            assert_eq!(board.yellow_pieces[0], Position::Outward(0));
            assert_eq!(board.yellow_pieces[1], Position::Outward(0));
            assert_eq!(board.yellow_pieces[2], Position::Outward(0));
        }

        #[test]
        fn move_r0o0_then_jump_y1h3() {
            let mut board = Board3::new(Turn::Red);

            board.yellow_pieces[1] = Position::Homeward(3);

            let board = board.move_at(0).unwrap();

            assert_eq!(board.turn, Turn::Yellow);
            assert_eq!(board.red_pieces[0], Position::Outward(3));
            assert_eq!(board.red_pieces[1], Position::Outward(0));
            assert_eq!(board.red_pieces[2], Position::Outward(0));
            assert_eq!(board.yellow_pieces[0], Position::Outward(0));
            assert_eq!(board.yellow_pieces[1], Position::Homeward(0));
            assert_eq!(board.yellow_pieces[2], Position::Outward(0));
        }

        #[test]
        fn move_r0h0() {
            let mut board = Board3::new(Turn::Red);

            board.red_pieces[0] = Position::Homeward(0);

            let board = board.move_at(0).unwrap();

            assert_eq!(board.turn, Turn::Yellow);
            assert_eq!(board.red_pieces[0], Position::Homeward(1));
            assert_eq!(board.red_pieces[1], Position::Outward(0));
            assert_eq!(board.red_pieces[2], Position::Outward(0));
            assert_eq!(board.yellow_pieces[0], Position::Outward(0));
            assert_eq!(board.yellow_pieces[1], Position::Outward(0));
            assert_eq!(board.yellow_pieces[2], Position::Outward(0));
        }

        #[test]
        fn move_r0h3() {
            let mut board = Board3::new(Turn::Red);

            board.red_pieces[0] = Position::Homeward(3);

            let board = board.move_at(0).unwrap();

            assert_eq!(board.turn, Turn::Yellow);
            assert_eq!(board.red_pieces[0], Position::Finished);
            assert_eq!(board.red_pieces[1], Position::Outward(0));
            assert_eq!(board.red_pieces[2], Position::Outward(0));
            assert_eq!(board.yellow_pieces[0], Position::Outward(0));
            assert_eq!(board.yellow_pieces[1], Position::Outward(0));
            assert_eq!(board.yellow_pieces[2], Position::Outward(0));
        }
    }
}