use std::fmt::Debug;

pub trait Board where Self: Sized + Clone + Debug {
    fn get_board_size() -> usize;

    fn move_at(self: &Self, piece_index: usize) -> Option<Self>;

    fn encode(self: &Self) -> Code;

    fn get_result(self: &Self) -> GameResult;
}

// battle field, Outward(1..BOARD_SIZE) && Homeward(BOARD_SIZE..1)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Position {
    Outward(u8),
    Homeward(u8),
    Finished,
}

impl std::fmt::Display for Position {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Position::Outward(n) => write!(f, "o{}", n),
            Position::Homeward(n) => write!(f, "h{}", n),
            Position::Finished => write!(f, "f"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameResult {
    Unknown,
    RedWins,
    YellowWins,
    Invalid,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Turn {
    Red,
    Yellow,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Code(pub String);