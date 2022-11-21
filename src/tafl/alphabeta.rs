use std::collections::HashMap;

use super::board::Board;
use super::board::TMove;

#[derive(Clone, Copy)]
pub struct TTEntry {
    pub tmove: TMove
}

pub struct  Negamax {
    pub map: HashMap<u64, TTEntry>
}

impl Negamax {

    pub fn negamax(&mut self, board: Board, depth: i32, max_depth: i32, alpha: i32, beta: i32) -> i32 {
        let winner = board.get_winner();
        match winner {
            Some(x) => { return (2 * (x == board.get_player()) as i32 - 1) * (1000 - depth) },
            _ => {}
        }
        if depth == max_depth {
            return board.eval();
        }

        let mut max = -10_000;
        let mut alpha = alpha;

        // get move from hastable and try to skip everything
        let mut best_move = None;
        match self.map.get(&board.board) {
            Some(x) => {
                let tmove = x.clone().tmove;
                let nv = -self.negamax(board, depth + 1, max_depth, -beta, -alpha);
                if nv > max {
                    max = nv;
                    best_move = Some(tmove);
                    alpha = std::cmp::max(alpha, max);
                }
                if alpha > beta {
                    return max;
                }
            },
            None => {}
        }


        // get possible moves
        let possible_moves = board.get_possible_moves();

        // loose condition
        if possible_moves.is_empty() { return -1000 - depth; }

        // sort possible moves
        let mut moved_boards = possible_moves.into_iter().map(|tmove| (board.make_move(&tmove), tmove)).collect::<Vec<(Board, TMove)>>();
        moved_boards.sort_by_key(|b| b.0.eval());

        for (board, tmove) in moved_boards {
            let nv = -self.negamax(board, depth + 1, max_depth, -beta, -alpha);
            if nv > max {
                max = nv;
                best_move = Some(tmove);
                alpha = std::cmp::max(alpha, max);
            }
            if alpha > beta {
                break;
            }
        }

        // save best move in hashtable for future skip
        match best_move {
            Some(tmove) => { self.map.insert(board.board, TTEntry { tmove }); },
            None => {}
        }

        return max;
    }
    
    pub fn solve(&mut self, board: &Board, depth: i32) -> (i32, Option<TMove>) {
        let mut alpha = -10_000;
        let beta = 10_000;
        let winner = board.get_winner();
        match winner {
            Some(x) => { return ((2 * (x == board.get_player()) as i32 - 1) * (1000 - depth), None) },
            _ => {}
        }
        if depth == 0 {
            return (board.eval(), None);
        }

        let mut max = -10_000;
        let mut alpha = alpha;

        // get move from hastable and try to skip everything
        let mut best_move = None;
        match self.map.get(&board.board) {
            Some(x) => {
                let tmove = x.clone().tmove;
                let nv = -self.negamax(*board, 0, depth, -beta, -alpha);
                if nv > max {
                    max = nv;
                    best_move = Some(tmove);
                    alpha = std::cmp::max(alpha, max);
                }
                if alpha > beta {
                    return (max, best_move);
                }
            },
            None => {}
        }


        // get possible moves
        let possible_moves = board.get_possible_moves();

        // loose condition
        if possible_moves.is_empty() { return (-1000 - depth, None); }

        // sort possible moves
        let mut moved_boards = possible_moves.into_iter().map(|tmove| (board.make_move(&tmove), tmove)).collect::<Vec<(Board, TMove)>>();
        moved_boards.sort_by_key(|b| b.0.eval());

        for (board, tmove) in moved_boards {
            let nv = -self.negamax(board, 0, depth, -beta, -alpha);
            if nv > max {
                max = nv;
                best_move = Some(tmove);
                alpha = std::cmp::max(alpha, max);
            }
            if alpha > beta {
                break;
            }
        }

        // save best move in hashtable for future skip
        match best_move {
            Some(tmove) => { self.map.insert(board.board, TTEntry { tmove }); },
            None => {}
        }

        return (max, best_move);
    }

    pub fn new() -> Negamax {
        Negamax { map: HashMap::new() }
    }
}

