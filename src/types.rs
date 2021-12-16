use std::fmt;

pub const WIDTH: usize = 8 + 8;
pub const HEIGHT: usize = 8;

pub const WHITE: u8 = 1 << 0;
pub const BLACK: u8 = 1 << 1;
pub const KING: u8 = 1 << 2;
pub const QUEEN: u8 = 1 << 3;
pub const ROOK: u8 = 1 << 4;
pub const BISHOP: u8 = 1 << 5;
pub const KNIGHT: u8 = 1 << 6;
pub const PAWN: u8 = 1 << 7;

#[derive(Copy, Clone, Debug, Default)]
pub struct Piece(u8);

impl Piece {
    fn value(self) -> usize {
        match self {
            _ if self.0 & KING > 0 => 0,
            _ if self.0 & QUEEN > 0 => 9,
            _ if self.0 & ROOK > 0 => 5,
            _ if self.0 & BISHOP > 0 => 3,
            _ if self.0 & KNIGHT > 0 => 3,
            _ if self.0 & PAWN > 0 => 1,
            _ if self.0 == 0 => 0,
            _ => unreachable!(),
        }
    }

    fn char(self) -> char {
        match self {
            _ if self.0 == 0 => '.',
            _ if self.0 & BLACK > 0 && self.0 & BISHOP > 0 => '♝',
            _ if self.0 & BLACK > 0 && self.0 & KING > 0 => '♚',
            _ if self.0 & BLACK > 0 && self.0 & KNIGHT > 0 => '♞',
            _ if self.0 & BLACK > 0 && self.0 & PAWN > 0 => '♟',
            _ if self.0 & BLACK > 0 && self.0 & QUEEN > 0 => '♛',
            _ if self.0 & BLACK > 0 && self.0 & ROOK > 0 => '♜',
            _ if self.0 & WHITE > 0 && self.0 & BISHOP > 0 => '♗',
            _ if self.0 & WHITE > 0 && self.0 & KING > 0 => '♔',
            _ if self.0 & WHITE > 0 && self.0 & KNIGHT > 0 => '♘',
            _ if self.0 & WHITE > 0 && self.0 & PAWN > 0 => '♙',
            _ if self.0 & WHITE > 0 && self.0 & QUEEN > 0 => '♕',
            _ if self.0 & WHITE > 0 && self.0 & ROOK > 0 => '♖',
            _ => unreachable!(),
        }
    }
}

impl From<u8> for Piece {
    fn from(p: u8) -> Piece {
        Piece(p)
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.char())
    }
}

pub struct Board {
    pub board: [[Piece; WIDTH]; HEIGHT],
}

impl Default for Board {
    fn default() -> Self {
        let mut board: [[Piece; WIDTH]; HEIGHT] = Default::default();

        for i in 0..HEIGHT {
            board[i][i] = Piece::from(WHITE | ROOK);
        }

        Self { board }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, rank) in self.board.iter().enumerate() {
            write!(f, " {} ", HEIGHT - i)?;
            for cell in rank {
                write!(f, "{} ", cell)?;
            }
            writeln!(f)?;
        }

        write!(f, "   ")?;
        for i in 0..WIDTH {
            write!(f, "{} ", (b'a' + i as u8) as char)?;
        }

        Ok(())
    }
}
