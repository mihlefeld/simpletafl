mod tafl;
use tafl::{board::Board, tmove::TMove};
use tafl::negamax::Negamax;
use text_io::read;
use std::str::FromStr;
use std::time::{Instant};
use argparse::{ArgumentParser, Store, StoreTrue};



fn search_in_time(negamax: &mut Negamax, board: &Board, start_depth: i32, depth: i32, step: usize, t: f32, pvs: bool) -> (i32, Option<TMove>) {
    let t0 = Instant::now();
    let mut last_result = negamax.solve(board, start_depth, pvs);
    println!("Depth\tTime\tTotal\tPts\tLogLen\tMove\tNorm\tTran\tZerW\tPVS-\tTotl");
    for d in (start_depth+1..=depth).step_by(step) {
        let t1 = Instant::now();
        let negamax_result = negamax.solve(board, d, pvs);
        let elapsed_d = t1.elapsed().as_secs_f32();
        let elapsed = t0.elapsed().as_secs_f32();
        let log_len = (negamax.map.len() as f32).log10();
        print!("{d}\t{elapsed_d:.2}s\t{elapsed:.2}s\t{}\t{log_len:.1}", negamax_result.0);
        match negamax_result {
            (_, Some(tmove)) => { print!("\t{}\t", tmove.to_string()) }
            (_, None) => { print!("\t") }
        }
        let normal = negamax.normal_calls;
        let transpo = negamax.transpo_calls;
        let zerow = negamax.zero_window_calls;
        let pvs_fail = negamax.pvs_failed_calls;
        let total = normal + transpo + zerow + pvs_fail;
        println!("{normal:.1e}\t{transpo:.1e}\t{zerow}\t{pvs_fail}\t{total:.1e}");
        if elapsed < t/2.0 {
            last_result = negamax_result;
        } else if elapsed <= t {
            return negamax_result;
        } else {
            return last_result;
        }
    }
    return last_result;
}


fn search(board: &Board, start_depth: i32, depth: i32, step: usize, pvs: bool) -> Option<TMove> {
    let mut negamax = Negamax::new();
    return search_in_time(&mut negamax, board, start_depth, depth, step, 60.0*60.0*24.0, pvs).1;
}

fn solve(board: &Board, depth: i32, pvs: bool) {
    board.print_board();
    search(board, 1, depth, 2, pvs);
}

fn get_human_move(possible_moves: &Vec<TMove>) -> Option<TMove> {
    let mut input = "".to_string();
    while input.to_lowercase().as_str() != "exit" {
        println!("Enter move: ");
        input = read!();
        match TMove::from_str(&input) {
            Ok(tmove) => { 
                if possible_moves.contains(&tmove) {
                    return Some(tmove); 
                }
                println!("Not a valid move!");
            }
            Err(_) => { println!("Syntax Error!")}
        } 
    }
    return None;
}

fn is_next_player_human() -> bool {
    let mut input = "".to_string();
    while input.to_lowercase().as_str() != "exit" {
        println!("Enter next player h/c: ");
        input = read!();
        match input.as_str() {
            "c" => { return false; }
            "h" => { return true; }
            _ => {}
        } 
    }
    return true;
}

fn sandbox(start_board: &Board, pvs: bool) {
    let mut negamax = Negamax::new();
    let mut board = *start_board;
    loop {
        board.print_board();
        match board.get_winner() {
            Some(0) => { 
                println!("White Won!"); 
                return; 
            }
            Some(1) => { 
                println!("Black Won!");
                return; 
            }
            _ => {}
        }

        let possible_moves = board.get_possible_moves();
        if possible_moves.is_empty() {
            match board.get_player() {
                0 => println!("Black Won!"),
                _ => println!("White Won!")
            }
            return;
        }
        
        let tmove_option = match is_next_player_human() {
            true => { get_human_move(&possible_moves) }
            false => { search_in_time(&mut negamax, &board, 4, 20, 2, 10.0, pvs).1 }
        };
        
        match tmove_option {
            Some(tmove) => { 
                println!("Executing move {}", tmove.to_string());
                board = board.make_move(&tmove); 
            },
            None => { return; }
        }
    }
}


fn main() {
    let mut mode = "solve".to_string();
    let mut base_board = "start".to_string();
    let mut depth = 12;
    let mut pvs = false;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("What do you want to do?");
        ap.refer(&mut mode).add_option(&["-a", "--action"], Store, "Action: solve, sanbox");
        ap.refer(&mut base_board).add_option(&["-b", "--board"], Store, "Staring postion. One of: start, benchmark, 18move");
        ap.refer(&mut depth).add_option(&["-d", "--depth"], Store, "Search depth for computer generation.");
        ap.refer(&mut pvs).add_option(&["-p", "--pvs"], StoreTrue, "Search depth for computer generation.");
        ap.parse_args_or_exit();
    }

    println!("Mode\tBoard    \tDepth\tPVS");
    println!("{mode}\t{base_board}\t{depth}\t{pvs}");

    let board = Board { board: 
        match base_board.as_str() {
            "benchmark" => 0b0_0000010000_0100000001_0110101001_0111000010_0001000001,
            "start" => 0b0_0100010001_0000100000_0110111001_0000100000_0100010001,
            "18move" => 0b0_0100010000_0000000100_0110101001_0111000010_0001000001,
            _ => { println!("Defaulting to start position"); 0b0_0100010001_0000100000_0110111001_0000100000_0100010001 }
        }
    };

    match mode.as_str() {
        "solve" => { solve(&board, depth, pvs); },
        "sandbox" => { sandbox(&board, pvs); },
        _ => { println!("Defaulting to solve!"); solve(&board, depth, pvs)}
    }
}   
