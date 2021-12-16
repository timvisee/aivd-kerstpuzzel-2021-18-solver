use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};

mod input;
mod types;

use took::Timer;

use input::*;
use types::*;

/// Whether to output color.
pub const COLOR: bool = true;

/// Tracks total number of tested positions.
static TOTAL: AtomicUsize = AtomicUsize::new(0);

fn main() {
    // Configure colors
    colored::control::set_override(COLOR);

    // Load board and pieces
    let board = Board::from_fen(INPUT);
    let mut pieces = PieceSet::from_fen(PIECES);
    pieces.remove_board(&board);
    pieces.sort();

    println!("# Initial state");
    println!();
    println!("{}", board);
    println!();
    println!("To place: {}", pieces);
    println!();
    println!();
    println!("# Start brute force");

    let timer = Timer::new();
    let mut positions = HashSet::new();
    brute(board, pieces, &mut positions);
    let took = timer.took();

    // Show test count
    let count = TOTAL.load(Ordering::Relaxed);
    println!();
    took.describe("# Brute force");
    println!("# Possible positions: {}", positions.len());
    println!("# Tested positions: {}", count);
    println!();
    println!("Done.");
}

/// Brute force given board with given set of pieces.
fn brute(board: Board, mut pieces: PieceSet, found: &mut HashSet<Board>) {
    if let Some(same_pieces) = pieces.pop_off_same() {
        brute_same_type(board, same_pieces, (0, 0), pieces, found);
    }
}

/// Brute force only with given `pieces` which are of the same type, and place them only after the
/// given `after` coordinate.
///
/// Once all pieces are consumed `rest_pieces` are used.
fn brute_same_type(
    board: Board,
    same_pieces: PieceSet,
    start_cell: (usize, usize),
    mut rest_pieces: PieceSet,
    found: &mut HashSet<Board>,
) {
    // If same piece set is emtpy, get new pieces or check complete
    if same_pieces.is_empty() {
        // Try to get new same pieces
        if let Some(same_pieces) = rest_pieces.pop_off_same() {
            return brute_same_type(board, same_pieces, (0, 0), rest_pieces, found);
        }

        assert!(rest_pieces.is_empty(), "Rest pieces should be empty");
        return handle_complete(board, found);
    }

    // Get board diff
    let (rank_diff, file_diff) = board.target_diff(RANK_TARET, FILE_TARGET);

    let piece = same_pieces.next().unwrap();
    let val = piece.value();

    (start_cell.0..WIDTH).for_each(|x| {
        let file_diff = file_diff[x];

        // Ensure file sum is <= target
        if piece.0 & WHITE > 0 {
            if file_diff.0 < val {
                return;
            }
        } else if piece.0 & BLACK > 0 {
            if file_diff.1 < val {
                return;
            }
        } else {
            unreachable!();
        }

        (0..HEIGHT).for_each(|y| {
            // Improve this, start loop at proper position first time
            if start_cell.0 == x && start_cell.1 > y {
                return;
            }

            let rank_diff = rank_diff[y];

            // Cell must be empty
            if board.board[y][x].0 != 0 {
                return;
            }

            // Ensure rank sum is <= target
            if piece.0 & WHITE > 0 {
                if rank_diff.0 < val {
                    return;
                }
            } else if piece.0 & BLACK > 0 {
                if rank_diff.1 < val {
                    return;
                }
            } else {
                unreachable!();
            }

            // Increment position check counter
            incr_total_count();

            let mut board = board;
            let mut pieces = same_pieces.clone();

            board.board[y][x] = piece;
            pieces.pop();

            // TODO: do not clone here
            brute_same_type(board, pieces, (x, y), rest_pieces.clone(), found);
        });
    });
}

/// Handle complete board.
fn handle_complete(board: Board, found: &mut HashSet<Board>) {
    assert!(
        board.matches_target(RANK_TARET, FILE_TARGET),
        "Complete board should match target here"
    );

    // White cannot be in check
    if is_white_check(&board) {
        return;
    }

    // Print current board state if complete
    if !found.insert(board) {
        unreachable!("We should not find same position twice");
    }

    println!();
    println!("# Position: {}", found.len());
    println!();
    println!("{}", board);
}

/// Check whether white is currently checked by black.
fn is_white_check(board: &Board) -> bool {
    // Find white kings
    let kings: Vec<(usize, usize)> = board
        .board
        .iter()
        .enumerate()
        .flat_map(|(y, rank)| {
            rank.iter()
                .enumerate()
                .filter(|(_, p)| p.0 == WHITE | KING)
                .map(move |(x, _)| (x, y))
        })
        .collect();

    assert_eq!(kings.len(), 2, "White should have two kings");

    // For each black piece, ensure it doesn't check white's kings
    board.board.iter().enumerate().any(|(y, rank)| {
        rank.iter()
            .enumerate()
            .filter(|(_, p)| p.0 & BLACK > 0)
            .any(|(x, p)| {
                p.attacked_pieces(board, (x, y))
                    .iter()
                    .any(|attack| kings.contains(attack))
            })
    })
}

#[inline(always)]
fn incr_total_count() {
    // Show test count
    let count = TOTAL.fetch_add(1, Ordering::Relaxed) + 1;
    if count % 100_000_000 == 0 {
        println!("# Tested: {}", count);
    }
}
