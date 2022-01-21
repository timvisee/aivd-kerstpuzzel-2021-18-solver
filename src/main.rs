use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};

use rayon::prelude::*;
use regex::Regex;
use uci::Engine;

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
    println!("# Start position search...");

    let timer = Timer::new();
    let mut positions = HashSet::new();
    brute(board, pieces, &mut positions);
    let took = timer.took();

    // Show test count
    let count = TOTAL.load(Ordering::Relaxed);
    println!();
    took.describe("# Search");
    println!("# Possible positions: {}", positions.len());
    println!("# Tested positions: {}", count);
    println!("# Done placing pieces");

    println!();
    println!("# Filtering positions to mate in 2...");
    println!();

    // Filter all games to mate in 2
    filter_positions_mate_2(positions.into_iter().collect());
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
        .iter_pieces()
        .filter(|(_, piece)| piece.0 == WHITE | KING)
        .map(|(pos, _)| pos)
        .collect();

    assert_eq!(kings.len(), 2, "White should have two kings");

    // For each black piece, ensure it doesn't check white's kings
    board
        .iter_pieces()
        .filter(|(_, piece)| piece.0 & BLACK > 0)
        .any(|(pos, piece)| {
            piece
                .attacked_pieces(board, pos)
                .iter()
                .any(|attack| kings.contains(attack))
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

fn is_board_mate_2(board: &Board) -> Option<(Vec<String>, Vec<String>)> {
    let (mut fen_left, mut fen_right) = board.to_fen_left_right();
    fen_left += " b";
    fen_right += " b";

    let left = is_stockfish_mate_in(&fen_left, 2);
    let right = is_stockfish_mate_in(&fen_right, 2);

    if left.is_none() || right.is_none() {
        return None;
    }

    left.zip(right)
}

fn is_stockfish_mate_in(fen: &str, moves: usize) -> Option<Vec<String>> {
    let re = Regex::new(r" mate (\d+) ").unwrap();

    let engine = Engine::new("stockfish")
        .expect("failed to initialise stockfish, make sure it is installed");
    engine.set_position(fen).unwrap();

    let options = engine
        .command("go depth 2")
        .unwrap()
        .lines()
        .skip(1)
        .take(2)
        .map(|info| match re.captures(info) {
            // TODO: map here instead
            Some(captures) => {
                let val = captures.get(1).unwrap().as_str();
                let val: usize = val.parse().unwrap();

                if val == moves {
                    let plys = info
                        .split_once(" pv ")
                        .unwrap()
                        .1
                        .split(' ')
                        .map(|m| m.to_string())
                        .collect::<Vec<String>>();

                    // TODO: do not hardcode this here
                    if plys.len() != 3 {
                        return None;
                    }

                    Some(plys)
                } else {
                    None
                }
            }
            None => None,
        })
        .collect::<Vec<Option<Vec<String>>>>();

    // All options must be some
    if options.is_empty() || options.iter().any(|o| o.is_none()) {
        return None;
    }

    // TODO: do not clone here?
    options[0].clone()
}

fn filter_positions_mate_2(positions: Vec<Board>) {
    let counter = AtomicUsize::new(0);

    let timer = Timer::new();
    let valid: Vec<(Board, (Vec<String>, Vec<String>))> = positions
        .par_iter()
        .filter_map(|board| {
            let count = counter.fetch_add(1, Ordering::Relaxed) + 1;
            if count % 100 == 0 {
                println!(
                    "Checking {} ({}%)",
                    count,
                    ((count as f32 / positions.len() as f32) * 100.0).round()
                );
            }

            is_board_mate_2(board).map(|moves| (*board, moves))
        })
        .collect();
    let took = timer.took();

    println!();
    println!();
    took.describe("# Testing");
    println!("# Valid positions:");
    println!();

    for (i, (board, (left, right))) in valid.iter().enumerate() {
        println!();
        println!("# Solution {}", i + 1);
        println!();
        println!("{}", board);
        println!();
        println!("Moves left: 1. ‥ {}+ 2. {} {}#", left[0], left[1], left[2]);
        println!(
            "Moves right: 1. ‥ {}+ 2. {} {}#",
            right[0], right[1], right[2]
        );
    }

    println!();
    println!("# Done testing positions for mate in 2");
    println!("# Valid solutions: {}", valid.len());
    println!();
    println!("Done.");
}
