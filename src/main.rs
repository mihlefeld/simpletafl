mod tafl;
use tafl::board::Board;
use tafl::negamax::Negamax;
use std::time::Instant;

fn main() {
    // 0b1_01_00_01_00_00_00_00_00_00_01_01_10_10_10_01_01_11_00_00_10_00_01_00_00_01
    // old benchmark 0b0_0000010000_0100000001_0110101001_0100001110_0000000101
    // new benchmark 0b0_0000010000_0100000001_0110101001_0111000010_0001000001
    // start stellung 0b0_0100010001_0000100000_0110111001_0000100000_0100010001
    // debug stellung 0b0_0100010000_0000000001_0110101001_0111000010_0001010000
    // 18 move stellu 0b0_0100010000_0000000100_0110101001_0111000010_0001000001
    let board = Board::new(0b0_0000010000_0100000001_0110101001_0111000010_0001000001);
    println!("Start evaluation {}", board.eval());

    for _ in 0..1 {
        board.print_board();
        let mut negamax = Negamax::new();
        let total = Instant::now();
        for d in (2..=12).step_by(2) {
            let now = Instant::now();
            println!("Searching depth {d}...");
            let (v, m) = negamax.solve(&board, d);
            println!("Time: {:.2}s[{:.2}s]\t#MAP: {}\teval: {v}", now.elapsed().as_millis() as f32/1000., total.elapsed().as_millis() as f32/1000., negamax.map.len());
            match m {
                Some(tmove) => { println!("Next: {}", tmove.to_string()); }
                None => { }
            }
            println!("----------");
        }
    }
}   
