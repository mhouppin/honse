use cozy_chess::{Board, Color, Move, Piece, Rank, Square};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::iter::IntoIterator;

#[derive(Copy, Clone)]
pub struct ScoredMove {
    mv: Move,
    score: i16,
}

impl ScoredMove {
    pub fn new(board: &Board, mv: Move) -> Self {
        Self { mv, score: move_score(board, mv) }
    }

    pub fn score(&self) -> i16 {
        self.score
    }

    pub fn mv(&self) -> Move {
        self.mv
    }
}

impl PartialEq for ScoredMove {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for ScoredMove {}

impl PartialOrd for ScoredMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScoredMove {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

// Give a bonus to pieces taken during captures
fn piece_order_score(piece: Piece) -> i16 {
    match piece {
        Piece::Queen => 4096,
        Piece::Rook => 2048,
        Piece::Bishop => 1280,
        Piece::Knight => 1024,
        _ => 0
    }
}

fn move_score(board: &Board, mv: Move) -> i16 {
    let mut score = 0;

    // Give a bonus for captures
    if let Some(piece) = board.piece_on(mv.to) {
        if board.color_on(mv.to).unwrap() != board.side_to_move() {
            score += 8192 + piece_order_score(piece);
        }
    } else if let Some(file) = board.en_passant() {
        // Handle the en passant edge case
        let us = board.side_to_move();
        let ep_square = Square::new(file, Rank::Sixth.relative_to(us));

        if mv.to == ep_square && board.piece_on(mv.from) == Some(Piece::Pawn) {
            score += 8192;
        }
    }

    score
}

pub struct MoveOrdering {
    scored_moves: Vec<ScoredMove>,
}

impl MoveOrdering {
    pub fn new(board: &Board, moves: &Vec<Move>) -> Self {
        let mut ordering = MoveOrdering {
            scored_moves: Vec::with_capacity(moves.len()),
        };

        for &mv in moves {
            ordering.scored_moves.push(ScoredMove::new(board, mv));
        }

        ordering.scored_moves.sort_by(|l, r| r.cmp(l));
        ordering
    }
}

impl IntoIterator for MoveOrdering {
    type Item = ScoredMove;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.scored_moves.into_iter()
    }
}
