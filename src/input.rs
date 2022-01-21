use crate::types::*;

pub const INPUT: &str = "r6k7r/pp4ppp3ppk//14K///PP6PP5P/2B1K3R6R";
pub const PIECES: &str = "KKQQBNRRRRPPPPPPPPPPPkkqqbbnrrppppppppppp";

/// Rank scores from 8 to 1, `(sum white, sum black)`.
pub const RANK_TARET: RankTable = [
    (0, 10),
    (0, 7),
    (1, 4),
    (1, 2),
    (10, 6),
    (20, 9),
    (5, 10),
    (18, 0),
];

/// File scores from a to p, `(sum white, sum black)`.
pub const FILE_TARGET: FileTable = [
    (1, 6),
    (6, 7),
    (6, 9),
    (1, 0),
    (5, 0),
    (1, 0),
    (9, 2),
    (0, 1),
    (6, 1),
    (1, 0),
    (1, 1),
    (1, 4),
    (1, 1),
    (9, 1),
    (1, 0),
    (6, 15),
];
