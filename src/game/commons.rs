use std::fmt::Debug;

pub trait Board where Self: Sized + Clone + Debug {
    fn get_board_size() -> usize;

    fn move_at(self: &Self, piece_index: usize) -> Option<Self>;

    fn encode(self: &Self) -> Code;

    fn get_turn_from_code(code: &Code) -> Turn;

    fn get_result(self: &Self) -> GameResult;

    fn draw_ascii_art(self: &Self) -> String;
}

// battle field, Outward(1..BOARD_SIZE) && Homeward(1..BOARD_SIZE)
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
            Position::Finished => write!(f, "f_"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameResult {
    Unknown,
    RedWins,
    YellowWins,
    Drawn,
    Invalid,
}

impl std::fmt::Display for GameResult {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self) // cheat
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Turn {
    Red,
    Yellow,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Code(pub String);

impl Code {
    pub fn get_turn<B>(self: &Self) -> Turn where B: Board {
        B::get_turn_from_code(self)
    }
}