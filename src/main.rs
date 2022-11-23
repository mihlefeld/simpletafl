mod tafl;
use tafl::{board::Board, tmove::TMove};
use tafl::negamax::Negamax;
use text_io::read;
use std::str::FromStr;
use std::time::Instant;
use argparse::{ArgumentParser, Store};

fn search(board: &Board, start_depth: i32, depth: i32, step: usize, verbose: bool) -> Option<TMove> {
    let mut negamax = Negamax::new();
    let total = Instant::now();
    let mut best_move = None;
    for d in (start_depth..=depth).step_by(step) {
        let now = Instant::now();
        if verbose { println!("Searching depth {d}...") }
        let (v, m) = negamax.solve(&board, d);
        if verbose { println!("Time: {:.2}s[{:.2}s]\t#MAP: {}\teval: {v}", now.elapsed().as_millis() as f32/1000., total.elapsed().as_millis() as f32/1000., negamax.map.len()); }
        match m {
            Some(tmove) => { 
                best_move = Some(tmove);
                if verbose { println!("Next: {}", tmove.to_string()) }; 
            }
            None => { }
        }
        if verbose { println!("----------"); }
    }
    return best_move;
}

fn solve(board: &Board, depth: i32) {
    board.print_board();
    search(board, 2, depth, 2, true);
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

fn play(start_board: &Board, computer_starts: bool, depth: i32, with_computer: bool) {
    let mut board = *start_board;
    let mut computers_turn = computer_starts & with_computer;
    while true {
        board.print_board();
        match board.get_winner() {
            Some(0) => { println!("White Won!"); return; }
            Some(1) => { println!("Black Won!"); return; }
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

        let tmove_option = { match computers_turn {
            true => search(&board, 1, depth, 1, false),
            false => get_human_move(&possible_moves)
        }};
        match tmove_option {
            Some(tmove) => { 
                println!("Executing move {}", tmove.to_string());
                board = board.make_move(&tmove); 
                if with_computer {
                    computers_turn = !computers_turn;
                }
            },
            None => { return; }
        }
    }
}

fn main() {
    let mut mode = "solve".to_string();
    let mut base_board = "benchmark".to_string();
    let mut depth = 12;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("What do you want to do?");
        ap.refer(&mut mode).add_option(&["-a", "--action"], Store, "Action: solve, play, playhc, playch [playch: play vs computer, computer starts]");
        ap.refer(&mut base_board).add_option(&["-b", "--board"], Store, "Staring postion. One of: start, benchmark, 18move");
        ap.refer(&mut depth).add_option(&["-d", "--depth"], Store, "Search depth for computer generation.");
        ap.parse_args_or_exit();
    }

    let board = Board { board: 
        match base_board.as_str() {
            "benchmark" => 0b0_0000010000_0100000001_0110101001_0111000010_0001000001,
            "start" => 0b0_0100010001_0000100000_0110111001_0000100000_0100010001,
            "18move" => 0b0_0100010000_0000000100_0110101001_0111000010_0001000001,
            _ => { println!("Defaulting to start position"); 0b0_0100010001_0000100000_0110111001_0000100000_0100010001 }
        }
    };

    match mode.as_str() {
        "solve" => { solve(&board, depth); },
        "play" => { play(&board, false, 0, false); },
        "playhc" => { play(&board, false, 8, true); },
        "playch" => { play(&board, true, 8, true); },
        _ => { println!("Defaulting to solve!"); solve(&board, depth)}
    }
}   
