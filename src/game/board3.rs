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

    //               .   :   .  next-> Yellow
    //         +---+---+---+---+---+
    //         |===| v |   | v |===|
    //         +---+---+---+---+---+
    //     . 2 | > |   |   |   |   | :
    //         +---+---+---+---+---+
    //     : 1 |   |   |   |   | < | .
    //         +---+---+---+---+---+
    //     . 0 | > |   |   |   |   | :
    //         +---+---+---+---+---+
    //         |===|   | ^ |   |===|
    //         +---+---+---+---+---+
    // Yellow /      0   1   2
    //       / Red   :   .   :
    fn draw_ascii_art(self: &Self) -> String {
        const EMPTY: &str = "   ";
        const UP: &str = " ^ ";
        const RIGHT: &str = " > ";
        const DOWN: &str = " v ";
        const LEFT: &str = " < ";

        let r0t = if self.red_pieces[0] == Position::Homeward(0) { DOWN } else { EMPTY };
        let r1t = if self.red_pieces[1] == Position::Homeward(0) { DOWN } else { EMPTY };
        let r2t = if self.red_pieces[2] == Position::Homeward(0) { DOWN } else { EMPTY };

        let y0s = match self.yellow_pieces[0] {
            Position::Outward(0) => RIGHT,
            Position::Finished => LEFT,
            _ => EMPTY,
        };
        let y1s = match self.yellow_pieces[1] {
            Position::Outward(0) => RIGHT,
            Position::Finished => LEFT,
            _ => EMPTY,
        };
        let y2s = match self.yellow_pieces[2] {
            Position::Outward(0) => RIGHT,
            Position::Finished => LEFT,
            _ => EMPTY,
        };

        let r0s = match self.red_pieces[0] {
            Position::Outward(0) => UP,
            Position::Finished => DOWN,
            _ => EMPTY
        };
        let r1s = match self.red_pieces[1] {
            Position::Outward(0) => UP,
            Position::Finished => DOWN,
            _ => EMPTY
        };
        let r2s = match self.red_pieces[2] {
            Position::Outward(0) => UP,
            Position::Finished => DOWN,
            _ => EMPTY
        };

        let board = self;
        let get_square = |yellow_index: usize, red_index: usize| -> &'static str {
            match (board.red_pieces[red_index], board.yellow_pieces[yellow_index]) {
                (Position::Outward(n), _) if n == yellow_index as u8 + 1 => UP,
                (Position::Homeward(n), _) if n == (BOARD_SIZE - yellow_index) as u8 => DOWN,
                (_, Position::Outward(n)) if n == red_index as u8 + 1 => RIGHT,
                (_, Position::Homeward(n)) if n == (BOARD_SIZE - red_index) as u8 => LEFT,
                _ => EMPTY
            }
        };

        let y0t = if self.yellow_pieces[0] == Position::Homeward(0) { LEFT } else { EMPTY };
        let y1t = if self.yellow_pieces[1] == Position::Homeward(0) { LEFT } else { EMPTY };
        let y2t = if self.yellow_pieces[2] == Position::Homeward(0) { LEFT } else { EMPTY };

        return [
            format!("              .   :   .  next-> {}", if self.turn == Turn::Red { "Red" } else { "Yellow" }),
            format!("        +---+---+---+---+---+"),
            format!("        |===|{0}|{1}|{2}|===|", r0t, r1t, r2t),
            format!("        +---+---+---+---+---+"),
            format!("    . 2 |{0}|{1}|{2}|{3}|{4}| :", y2s, get_square(2, 0), get_square(2, 1), get_square(2, 2), y2t),
            format!("        +---+---+---+---+---+"),
            format!("    : 1 |{0}|{1}|{2}|{3}|{4}| .", y1s, get_square(1, 0), get_square(1, 1), get_square(1, 2), y1t),
            format!("        +---+---+---+---+---+"),
            format!("    . 0 |{0}|{1}|{2}|{3}|{4}| :", y0s, get_square(0, 0), get_square(0, 1), get_square(0, 2), y0t),
            format!("        +---+---+---+---+---+"),
            format!("        |===|{0}|{1}|{2}|===|", r0s, r1s, r2s),
            format!("        +---+---+---+---+---+"),
            format!("Yellow /      0   1   2"),
            format!("      / Red   :   .   :"),
        ].join("\n");
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
        use super::super::super::super::game::commons::{Turn, Board, Position};

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

    mod draw_ascii_art {
        use super::super::Board3;
        use super::super::super::commons::{Turn, Board, Position};

        #[test]
        fn initial_board() {
            let board = Board3::new(Turn::Red);

            assert_eq!(board.draw_ascii_art(), [
                "              .   :   .  next-> Red",
                "        +---+---+---+---+---+",
                "        |===|   |   |   |===|",
                "        +---+---+---+---+---+",
                "    . 2 | > |   |   |   |   | :",
                "        +---+---+---+---+---+",
                "    : 1 | > |   |   |   |   | .",
                "        +---+---+---+---+---+",
                "    . 0 | > |   |   |   |   | :",
                "        +---+---+---+---+---+",
                "        |===| ^ | ^ | ^ |===|",
                "        +---+---+---+---+---+",
                "Yellow /      0   1   2",
                "      / Red   :   .   :"].join("\n"));
        }

        #[test]
        fn all_in_turning_point() {
            let mut board = Board3::new(Turn::Yellow);

            board.red_pieces[0] = Position::Homeward(0);
            board.red_pieces[1] = Position::Homeward(0);
            board.red_pieces[2] = Position::Homeward(0);
            board.yellow_pieces[0] = Position::Homeward(0);
            board.yellow_pieces[1] = Position::Homeward(0);
            board.yellow_pieces[2] = Position::Homeward(0);

            assert_eq!(board.draw_ascii_art(), [
                "              .   :   .  next-> Yellow",
                "        +---+---+---+---+---+",
                "        |===| v | v | v |===|",
                "        +---+---+---+---+---+",
                "    . 2 |   |   |   |   | < | :",
                "        +---+---+---+---+---+",
                "    : 1 |   |   |   |   | < | .",
                "        +---+---+---+---+---+",
                "    . 0 |   |   |   |   | < | :",
                "        +---+---+---+---+---+",
                "        |===|   |   |   |===|",
                "        +---+---+---+---+---+",
                "Yellow /      0   1   2",
                "      / Red   :   .   :"].join("\n"));
        }

        #[test]
        fn all_in_finish_point() {
            let mut board = Board3::new(Turn::Red);

            board.red_pieces[0] = Position::Finished;
            board.red_pieces[1] = Position::Finished;
            board.red_pieces[2] = Position::Finished;
            board.yellow_pieces[0] = Position::Finished;
            board.yellow_pieces[1] = Position::Finished;
            board.yellow_pieces[2] = Position::Finished;

            assert_eq!(board.draw_ascii_art(), [
                "              .   :   .  next-> Red",
                "        +---+---+---+---+---+",
                "        |===|   |   |   |===|",
                "        +---+---+---+---+---+",
                "    . 2 | < |   |   |   |   | :",
                "        +---+---+---+---+---+",
                "    : 1 | < |   |   |   |   | .",
                "        +---+---+---+---+---+",
                "    . 0 | < |   |   |   |   | :",
                "        +---+---+---+---+---+",
                "        |===| v | v | v |===|",
                "        +---+---+---+---+---+",
                "Yellow /      0   1   2",
                "      / Red   :   .   :"].join("\n"));
        }

        #[test]
        fn vortex() {
            let mut board = Board3::new(Turn::Red);

            board.red_pieces[0] = Position::Outward(2);
            board.red_pieces[1] = Position::Outward(0);
            board.red_pieces[2] = Position::Homeward(2);
            board.yellow_pieces[0] = Position::Homeward(2);
            board.yellow_pieces[1] = Position::Outward(0);
            board.yellow_pieces[2] = Position::Outward(2);

            assert_eq!(board.draw_ascii_art(), [
                "              .   :   .  next-> Red",
                "        +---+---+---+---+---+",
                "        |===|   |   |   |===|",
                "        +---+---+---+---+---+",
                "    . 2 |   |   | > |   |   | :",
                "        +---+---+---+---+---+",
                "    : 1 | > | ^ |   | v |   | .",
                "        +---+---+---+---+---+",
                "    . 0 |   |   | < |   |   | :",
                "        +---+---+---+---+---+",
                "        |===|   | ^ |   |===|",
                "        +---+---+---+---+---+",
                "Yellow /      0   1   2",
                "      / Red   :   .   :"].join("\n"));
        }

        #[test]
        fn saw() {
            let mut board = Board3::new(Turn::Red);

            board.red_pieces[0] = Position::Homeward(1);
            board.red_pieces[1] = Position::Homeward(3);
            board.red_pieces[2] = Position::Homeward(2);
            board.yellow_pieces[0] = Position::Homeward(1);
            board.yellow_pieces[1] = Position::Homeward(3);
            board.yellow_pieces[2] = Position::Homeward(2);

            assert_eq!(board.draw_ascii_art(), [
                "              .   :   .  next-> Red",
                "        +---+---+---+---+---+",
                "        |===|   |   |   |===|",
                "        +---+---+---+---+---+",
                "    . 2 |   | v | < |   |   | :",
                "        +---+---+---+---+---+",
                "    : 1 |   | < |   | v |   | .",
                "        +---+---+---+---+---+",
                "    . 0 |   |   | v | < |   | :",
                "        +---+---+---+---+---+",
                "        |===|   |   |   |===|",
                "        +---+---+---+---+---+",
                "Yellow /      0   1   2",
                "      / Red   :   .   :"].join("\n"));
        }
    }
}