use metrohash::MetroHashMap;
use super::board::Board;
use super::tmove::TMove;

#[derive(Clone, Copy)]
pub struct TTEntry {
    pub tmove: TMove
}

pub struct  Negamax {
    pub map: MetroHashMap<u64, TTEntry>,
    pub zero_window_calls: i32,
    pub pvs_failed_calls: i32,
    pub transpo_calls: i32,
    pub normal_calls: i32,
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
                self.transpo_calls += 1;
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
                self.normal_calls += 1;
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

    pub fn pvs(&mut self, board: &Board, d: i32, max_d: i32, alpha: i32, beta: i32) -> (i32, Option<TMove>) {
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
        let mut first_child_searched = false;

        match entry {
            Some(tentry) => {
                first_attempt = Some(tentry.tmove);
                best_move = first_attempt;
                first_child_searched = true;
                self.transpo_calls += 1;
                max = -self.pvs(&board.make_move(&tentry.tmove), d + 1, max_d, -beta, -alpha).0;
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
                let value = match first_child_searched.clone() {
                    true => {
                        self.zero_window_calls += 1;
                        let score = -self.pvs(moved_board, d + 1, max_d, -max - 1, -max).0;
                        if max < score && score < beta { self.pvs_failed_calls += 1; -self.pvs(moved_board, d + 1, max_d, -beta, -max).0 } 
                        else { score }
                    }
                    false => {
                        first_child_searched = true;
                        self.normal_calls += 1;
                        -self.pvs(moved_board, d + 1, max_d, -beta, -max).0
                    }
                };
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

    pub fn solve(&mut self, board: &Board, depth: i32, pvs: bool) -> (i32, Option<TMove>) {
        if pvs {
            self.pvs(board, -1, depth, -10_000, 10_000)
        } else {
            self.negamax(board, -1, depth, -10_000, 10_000)
        }
    }

    pub fn new() -> Negamax {
        Negamax { map: MetroHashMap::default(), pvs_failed_calls: 0, zero_window_calls: 0, normal_calls: 0, transpo_calls: 0 }
    }
}

