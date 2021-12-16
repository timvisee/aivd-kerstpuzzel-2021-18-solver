use std::fmt;
use std::iter;
use std::mem;
use std::ops::Index;

use colored::Colorize;

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

pub type RankTable = [(usize, usize); HEIGHT];
pub type FileTable = [(usize, usize); WIDTH];

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Piece(pub u8);

impl Piece {
    pub fn from_fen(p: char) -> Self {
        Piece(match p {
            'K' => WHITE | KING,
            'Q' => WHITE | QUEEN,
            'R' => WHITE | ROOK,
            'B' => WHITE | BISHOP,
            'N' => WHITE | KNIGHT,
            'P' => WHITE | PAWN,
            'k' => BLACK | KING,
            'q' => BLACK | QUEEN,
            'r' => BLACK | ROOK,
            'b' => BLACK | BISHOP,
            'n' => BLACK | KNIGHT,
            'p' => BLACK | PAWN,
            _ => unreachable!(),
        })
    }

    pub fn to_fen(self) -> Option<char> {
        match self {
            _ if self.0 == 0 => None,
            _ if self.0 == WHITE | KING => Some('K'),
            _ if self.0 == WHITE | QUEEN => Some('Q'),
            _ if self.0 == WHITE | ROOK => Some('R'),
            _ if self.0 == WHITE | BISHOP => Some('B'),
            _ if self.0 == WHITE | KNIGHT => Some('N'),
            _ if self.0 == WHITE | PAWN => Some('P'),
            _ if self.0 == BLACK | KING => Some('k'),
            _ if self.0 == BLACK | QUEEN => Some('q'),
            _ if self.0 == BLACK | ROOK => Some('r'),
            _ if self.0 == BLACK | BISHOP => Some('b'),
            _ if self.0 == BLACK | KNIGHT => Some('n'),
            _ if self.0 == BLACK | PAWN => Some('p'),
            _ => unreachable!(),
        }
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn value(self) -> usize {
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

    pub fn char(self) -> char {
        match self {
            _ if self.0 == 0 => '·',
            _ if self.0 == WHITE | KING => '♚',
            _ if self.0 == WHITE | QUEEN => '♛',
            _ if self.0 == WHITE | ROOK => '♜',
            _ if self.0 == WHITE | BISHOP => '♝',
            _ if self.0 == WHITE | KNIGHT => '♞',
            _ if self.0 == WHITE | PAWN => '♟',
            _ if self.0 == BLACK | KING => '♔',
            _ if self.0 == BLACK | QUEEN => '♕',
            _ if self.0 == BLACK | ROOK => '♖',
            _ if self.0 == BLACK | BISHOP => '♗',
            _ if self.0 == BLACK | KNIGHT => '♘',
            _ if self.0 == BLACK | PAWN => '♙',
            _ => unreachable!(),
        }
    }

    pub fn format(self) -> String {
        let c = self.char().to_string();
        if self.0 == 0 {
            c.bright_black().to_string()
        } else if self.0 & WHITE > 0 {
            c.yellow().to_string()
        } else if self.0 & BLACK > 0 {
            c.blue().to_string()
        } else {
            c
        }
    }

    /// Build a list of relative cell attack iterators.
    ///
    /// These may yield infinitely.
    pub fn attacked_iters(self) -> Vec<Box<dyn Iterator<Item = (isize, isize)>>> {
        match self {
            _ if self.0 & KING > 0 => vec![
                Box::new(iter::once((-1, -1))),
                Box::new(iter::once((-1, 0))),
                Box::new(iter::once((-1, 1))),
                Box::new(iter::once((0, -1))),
                Box::new(iter::once((0, 1))),
                Box::new(iter::once((1, -1))),
                Box::new(iter::once((1, 0))),
                Box::new(iter::once((1, 1))),
            ],
            _ if self.0 & QUEEN > 0 => vec![
                Box::new((1..).map(|n| (-n, 0))),
                Box::new((1..).map(|n| (n, 0))),
                Box::new((1..).map(|n| (0, -n))),
                Box::new((1..).map(|n| (0, n))),
                Box::new((1..).map(|n| (n, n))),
                Box::new((1..).map(|n| (-n, n))),
                Box::new((1..).map(|n| (n, -n))),
                Box::new((1..).map(|n| (-n, -n))),
            ],
            _ if self.0 & ROOK > 0 => vec![
                Box::new((1..).map(|n| (-n, 0))),
                Box::new((1..).map(|n| (n, 0))),
                Box::new((1..).map(|n| (0, -n))),
                Box::new((1..).map(|n| (0, n))),
            ],
            _ if self.0 & BISHOP > 0 => vec![
                Box::new((1..).map(|n| (n, n))),
                Box::new((1..).map(|n| (-n, n))),
                Box::new((1..).map(|n| (n, -n))),
                Box::new((1..).map(|n| (-n, -n))),
            ],
            _ if self.0 & KNIGHT > 0 => vec![
                Box::new(iter::once((-1, -2))),
                Box::new(iter::once((1, -2))),
                Box::new(iter::once((-2, 1))),
                Box::new(iter::once((-2, -1))),
                Box::new(iter::once((2, 1))),
                Box::new(iter::once((2, -1))),
                Box::new(iter::once((-1, 2))),
                Box::new(iter::once((1, 2))),
            ],
            _ if self.0 & PAWN > 0 => vec![
                Box::new(iter::once((-1, if self.0 & WHITE > 0 { -1 } else { 1 }))),
                Box::new(iter::once((1, if self.0 & WHITE > 0 { -1 } else { 1 }))),
            ],
            _ if self.0 == 0 => vec![],
            _ => unreachable!(),
        }
    }

    /// Find positions of pieces that are attacked on the board by this piece.
    ///
    /// The board and the position of the current piece must be given.
    pub fn attacked_pieces(self, board: &Board, pos: (usize, usize)) -> Vec<(usize, usize)> {
        // Assert we are on the board
        assert!(board[pos] == self);

        // Go through each attack iter, find collision positions
        self.attacked_iters()
            .into_iter()
            .map(|iter| {
                iter.map(|p| {
                    (
                        (pos.0 as isize + p.0) as usize,
                        (pos.1 as isize + p.1) as usize,
                    )
                })
                .map(|p| board.get(p).map(|piece| (p, piece)))
                .take_while(|p| p.is_some())
                .flatten()
                .find(|(_, piece)| !piece.is_empty())
                .map(|(pos, _)| pos)
            })
            .flatten()
            .collect()
    }
}

impl From<u8> for Piece {
    fn from(p: u8) -> Piece {
        Piece(p)
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[derive(Copy, Clone, Default, Hash, Eq, PartialEq)]
pub struct Board {
    pub board: [[Piece; WIDTH]; HEIGHT],
}

impl Board {
    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board::default();

        for (y, rank) in fen.split('/').enumerate() {
            let mut i = 0;
            let mut x = 0;

            while i < rank.len() {
                let c = rank.as_bytes()[i];
                if c.is_ascii_alphabetic() {
                    board.board[y][x] = Piece::from_fen(c as char);
                    x += 1;
                    i += 1;
                    continue;
                }

                let num = rank
                    .get(i..=i + 1)
                    .filter(|n| n.bytes().all(|b| b.is_ascii_digit()))
                    .or_else(|| rank.get(i..=i))
                    .unwrap();

                x += num.parse::<usize>().unwrap();
                i += num.len();
            }
        }

        board
    }

    pub fn to_fen(self) -> String {
        self.board
            .iter()
            .map(|rank| rank_fen(rank))
            .collect::<Vec<_>>()
            .join("/")
    }

    pub fn to_fen_left_right(self) -> (String, String) {
        (
            self.board
                .iter()
                .map(|rank| rank_fen(&rank[0..WIDTH / 2]))
                .collect::<Vec<_>>()
                .join("/"),
            self.board
                .iter()
                .map(|rank| rank_fen(&rank[WIDTH / 2..]))
                .collect::<Vec<_>>()
                .join("/"),
        )
    }

    pub fn get(&self, pos: (usize, usize)) -> Option<&Piece> {
        self.board.get(pos.1).and_then(|rank| rank.get(pos.0))
    }

    /// Iterate over all board cells.
    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), Piece)> + '_ {
        self.board
            .iter()
            .enumerate()
            .flat_map(|(y, rank)| rank.iter().enumerate().map(move |(x, p)| ((x, y), *p)))
    }

    /// Iterate over all board pieces.
    pub fn iter_pieces(&self) -> impl Iterator<Item = ((usize, usize), Piece)> + '_ {
        self.iter().filter(|(_, p)| !p.is_empty())
    }

    /// Get sum of rank values, `(white, black)`.
    pub fn rank_score(&self, rank: usize) -> (usize, usize) {
        (
            self.board[rank]
                .iter()
                .filter(|p| p.0 & WHITE > 0)
                .map(|p| p.value())
                .sum(),
            self.board[rank]
                .iter()
                .filter(|p| p.0 & BLACK > 0)
                .map(|p| p.value())
                .sum(),
        )
    }

    /// Get sum of file values, `(white, black)`.
    pub fn file_score(&self, file: usize) -> (usize, usize) {
        (
            self.board
                .iter()
                .map(|rank| rank[file])
                .filter(|p| p.0 & WHITE > 0)
                .map(|p| p.value())
                .sum(),
            self.board
                .iter()
                .map(|rank| rank[file])
                .filter(|p| p.0 & BLACK > 0)
                .map(|p| p.value())
                .sum(),
        )
    }

    pub fn matches_target(&self, ranks: RankTable, files: FileTable) -> bool {
        ranks.iter().enumerate().all(|(i, rank)| {
            let (white, black) = self.rank_score(i);
            white == rank.0 && black == rank.1
        }) && files.iter().enumerate().all(|(i, file)| {
            let (white, black) = self.file_score(i);
            white == file.0 && black == file.1
        })
    }

    pub fn target_diff(&self, ranks: RankTable, files: FileTable) -> (RankTable, FileTable) {
        let (mut ranks_diff, mut files_diff) = (ranks, files);

        ranks_diff
            .iter_mut()
            .zip(ranks)
            .enumerate()
            .for_each(|(i, (new, target))| {
                let (white, black) = self.rank_score(i);
                *new = (target.0 - white, target.1 - black);
            });
        files_diff
            .iter_mut()
            .zip(files)
            .enumerate()
            .for_each(|(i, (new, target))| {
                let (white, black) = self.file_score(i);
                *new = (target.0 - white, target.1 - black);
            });

        (ranks_diff, files_diff)
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Piece;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        self.get(pos).unwrap()
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

        writeln!(f)?;
        let fen = self.to_fen();
        let (fen_left, fen_right) = self.to_fen_left_right();
        write!(f, "\nFEN: {} ", fen)?;
        write!(f, "\nFEN L: {} ", fen_left)?;
        write!(f, "\nFEN R: {} ", fen_right)?;

        Ok(())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PieceSet {
    pub pieces: Vec<Piece>,
}

impl PieceSet {
    pub fn from(pieces: Vec<Piece>) -> Self {
        Self { pieces }
    }

    pub fn from_fen(fen: &str) -> Self {
        Self {
            pieces: fen.chars().map(Piece::from_fen).collect(),
        }
    }

    pub fn sort(&mut self) {
        self.pieces.sort_by_key(|p| p.value());
    }

    pub fn remove_piece(&mut self, piece: Piece) {
        let i = self.pieces.iter().position(|q| *q == piece).unwrap();
        self.pieces.remove(i);
    }

    pub fn remove_board(&mut self, board: &Board) {
        board.board.iter().flatten().for_each(|p| {
            if !p.is_empty() {
                self.remove_piece(*p);
            }
        });
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.pieces.is_empty()
    }

    #[inline]
    pub fn next(&self) -> Option<Piece> {
        self.pieces.last().copied()
    }

    #[inline]
    pub fn pop(&mut self) -> Option<Piece> {
        self.pieces.pop()
    }

    /// Pop off a set of all the same piece types from the end.
    #[inline]
    pub fn pop_off_same(&mut self) -> Option<PieceSet> {
        if self.pieces.is_empty() {
            return None;
        }

        // let piece = self.pieces.last().unwrap();
        // let i = self.pieces.iter().rposition(|p| p != piece).unwrap_or(0);
        // Some(PieceSet::from(self.pieces.split_off(i)))

        let piece = self.pieces.last().unwrap();

        match self.pieces.iter().rposition(|p| p != piece) {
            // Pop off pieces until different piece
            Some(i) => Some(PieceSet::from(self.pieces.split_off(i + 1))),

            // All pieces the same, create new pieset set, swap pieces and return
            None => {
                let mut same_pieces = PieceSet::default();
                mem::swap(&mut self.pieces, &mut same_pieces.pieces);
                Some(same_pieces)
            }
        }
    }
}

impl fmt::Display for PieceSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for piece in &self.pieces {
            write!(f, "{} ", piece)?;
        }
        Ok(())
    }
}

/// Generate FEN for single rank.
fn rank_fen(rank: &[Piece]) -> String {
    let mut rank_fen = String::new();
    let mut i = 0;
    while i < rank.len() {
        // If empty, check how many empy, add count
        let p = rank[i];
        if !p.is_empty() {
            rank_fen.push(p.to_fen().unwrap());
            i += 1;
        } else {
            // Find how many empty
            let empty = rank[i..rank.len()]
                .iter()
                .take_while(|p| p.is_empty())
                .count();
            rank_fen += &format!("{}", empty);
            i += empty;
        }
    }

    rank_fen
}
