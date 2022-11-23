use metrohash::MetroHashMap;
use super::board::Board;
use super::tmove::TMove;

#[derive(Clone, Copy)]
pub struct TTEntry {
    pub tmove: TMove
}

pub struct  Negamax {
    pub map: MetroHashMap<u64, TTEntry>
}

impl Negamax {

    pub fn negamax(&mut self, board: &Board, d: i32, max_d: i32, alpha: i32, beta: i32) -> (i32, Option<TMove>) {
        match board.get_winner() {
            Some(0) => { return ((1000 - d) * (1 - 2 * board.get_player() as i32), None); },
            Some(_) => { return ((1000 - d) * (2 * board.get_player() as i32 - 1), None); },
            None => { }
        }

        if d == max_d {
            return (board.eval(), None);
        }

        let entry = self.map.get(&board.board);
        let mut first_attempt = None;
        let mut max = alpha;
        let mut best_move = None;

        match entry {
            Some(tentry) => {
                first_attempt = Some(tentry.tmove);
                best_move = first_attempt;
                max = -self.negamax(&board.make_move(&tentry.tmove), d + 1, max_d, -beta, -alpha).0;
            },
            None => {}
        }

        if max < beta {
            // get possible moves
            let possible_moves = board.get_possible_moves();

            // loose condition
            if possible_moves.is_empty() { return (-1000 + d, first_attempt); }

            // sort possible moves
            let mut moved_boards = possible_moves.into_iter().map(|tmove| (board.make_move(&tmove), tmove)).collect::<Vec<(Board, TMove)>>();
            moved_boards.sort_by_key(|b| {
                b.0.eval()
            });

            for (moved_board, tmove) in &moved_boards {
                if first_attempt.is_some() && first_attempt.unwrap() == *tmove { continue; }
                let value = -self.negamax(moved_board, d + 1, max_d, -beta, -max).0;
                if value > max {
                    max = value;
                    best_move = Some(*tmove);

                    if max >= beta { break; }
                }
            }
            match best_move {
                Some(tmove) => { self.map.insert(board.board, TTEntry { tmove }); }
                None => {}
            }
        } 
        return (max, best_move);
    }

    pub fn solve(&mut self, board: &Board, depth: i32) -> (i32, Option<TMove>) {
        return self.negamax(board, -1, depth, -10_000, 10_000);
    }

    pub fn new() -> Negamax {
        Negamax { map: MetroHashMap::default() }
    }
}

