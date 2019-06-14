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
        if self.turn == Turn::Red {
            self.move_red_at(piece_index)
        } else {
            self.move_yellow_at(piece_index)
        }
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

    fn move_red_at(self: &Self, piece_index: usize) -> Option<Self> {
        let mut cloned = self.clone();

        cloned.red_pieces[piece_index] = match cloned.red_pieces[piece_index] {
            Position::Outward(n) => {
                let base_moves = RED_SPEEDS_OUTWARD[piece_index];
                if n + base_moves >= BOARD_SIZE as u8 { Position::Homeward(BOARD_SIZE as u8 + 1) } else { Position::Outward(n + base_moves) }
            }
            Position::Homeward(n) => {
                let base_moves = RED_SPEEDS_HOMEWARD[piece_index];
                if n - base_moves <= 0 { Position::Finished } else { Position::Homeward(n - base_moves) }
            }
            Position::Finished => return Option::None
        };

        cloned.turn = Turn::Yellow;

        Option::Some(cloned)
    }

    fn move_yellow_at(self: &Self, piece_index: usize) -> Option<Self> {
        let mut cloned = self.clone();

        cloned.yellow_pieces[piece_index] = match cloned.yellow_pieces[piece_index] {
            Position::Outward(n) => {
                let base_moves = YELLOW_SPEEDS_OUTWARD[piece_index];
                if n + base_moves >= BOARD_SIZE as u8 { Position::Homeward(BOARD_SIZE as u8 + 1) } else { Position::Outward(n + base_moves) }
            }
            Position::Homeward(n) => {
                let base_moves = YELLOW_SPEEDS_HOMEWARD[piece_index];
                if n - base_moves <= 0 { Position::Finished } else { Position::Homeward(n - base_moves) }
            }
            Position::Finished => return Option::None
        };

        cloned.turn = Turn::Red;

        Option::Some(cloned)
    }
}