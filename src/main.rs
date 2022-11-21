mod tafl;
use tafl::board::Board;
use tafl::alphabeta::Negamax;
use std::time::Instant;

fn main() {
    // 0b1_01_00_01_00_00_00_00_00_00_01_01_10_10_10_01_01_11_00_00_10_00_01_00_00_01
    // old benchmark 0b0_0000010000_0100000001_0110101001_0100001110_0000000101
    // new benchmark 0b0_0000010000_0100000001_0110101001_0111000010_0001000001
    // start stellung 0b0_0100010001_0000100000_0110111001_0000100000_0100010001
    // debug stellung 0b0_0100010000_0000000001_0110101001_0111000010_0001010000
    let board = Board::new(0b1_0010010000_0100000001_0100001001_0011000010_0101100001);
    println!("Start evaluation {}", board.eval());

    for _ in 0..1 {
        board.print_board();
        let mut negamax = Negamax::new();
        let total = Instant::now();
        for d in (2..7).step_by(1) {
            let now = Instant::now();
            println!("Searching depth {d}...");
            let (v, m) = negamax.solve(&board, d);
            println!("Depth {d} took {:.2}ms and total {:.2}ms {} elements in hashmap", now.elapsed().as_millis(), total.elapsed().as_millis(), negamax.map.len());
            match m {
                Some(tmove) => { println!("Next Move: {} with eval {v} .make_move(&{tmove:?})", tmove.to_string()); }
                None => { }
            }
            
        }
    }
}   
