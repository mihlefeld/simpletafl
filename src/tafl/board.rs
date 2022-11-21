
use colored::Colorize;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TMove {
    pub start: (u8, u8),
    pub end: (u8, u8)
}

impl ToString for TMove {
    fn to_string(&self) -> String {
        let (sx, sy) = self.start;
        let (ex, ey) = self.end;
        let letters = ["A", "B", "C", "D", "E"];
        let numbers = (1..=5).rev().collect::<Vec<i32>>();
        format!("{}{}->{}{}", letters[sx as usize], numbers[sy as usize], letters[ex as usize], numbers[ey as usize])
    }
}

#[derive(Clone, Copy)]
pub struct Board {pub board: u64}

impl Board {
    pub fn new(board: u64) -> Self{
        return Board{board};
    }

    pub fn get_winner(&self) -> Option<u8> {
        let mut king_pos: Option<(u8, u8)> = None;
        for i in 0..5 {
            for j in 0..5 {
                if self.get(i, j) == 3 {
                    king_pos = Some((i, j));
                }
            }
        }
        match king_pos {
            None => { return Some(1); }
            Some((x, y)) => {
                if x == 0 || x == 4 || y == 0 || y == 4 {
                    return Some(0);
                }
                return None;
            }
        }
    }

    pub fn get(&self, x: u8, y: u8) -> u8 {
        let i = y * 5 + x;
        let shift = 48 - i * 2;
        ((self.board >> shift) & 0b11) as u8
    }

    fn set(&self, x: u8, y: u8, v: u8) -> Self {
        let i = y * 5 + x;
        let shift = 48 - i * 2;
        let mask = 0b11u64 << shift;
        let board = (self.board & !mask) | ((v as u64) << shift);
        Board {board}
    }

    pub fn get_player(&self) -> u8 {
        ((self.board >> 50) & 1) as u8
    }

    fn set_player(&self, p: u8) -> Self {
        let mask = 1u64 << 50;
        let board = self.board & !mask | ((p as u64) << 50);
        Board { board }
    }

    fn next_player(&self) -> Self {
        Board { board: self.board ^ (1u64 << 50) }
    }

    fn check_beaten(&self, player: u8, middle_piece: u8, outer_piece: u8) -> bool {
        let p1_cond = (player == 0) && (outer_piece > 1) && (middle_piece == 1);
        let p2_cond = (player == 1) && (outer_piece == 1) && (middle_piece > 1);
        return p1_cond | p2_cond;
    }

    pub fn make_move(&self, tmove: &TMove) -> Board {
        let (sx, sy) = tmove.start;
        let (ex, ey) = tmove.end;
        let player = self.get_player();
        let piece = self.get(sx, sy);
        let mut board = self.set(sx, sy, 0).set(ex, ey, piece);
        
        // check for pieces to remove, needs space for 2 pieces 0, 1, 2, 3, 4
        // check left
        if ex > 1 {
            let middle_piece = board.get(ex - 1, ey);
            let outer_piece = board.get(ex - 2, ey);
            if board.check_beaten(player, middle_piece, outer_piece) {
                board = board.set(ex-1, ey, 0);
            }
        }
        // check right
        if ex < 3 {
            let middle_piece = board.get(ex + 1, ey);
            let outer_piece = board.get(ex + 2, ey);
            if board.check_beaten(player, middle_piece, outer_piece) {
                board = board.set(ex + 1, ey, 0);
            }
        }
        // check top
        if ey > 1 {
            let middle_piece = board.get(ex, ey - 1);
            let outer_piece = board.get(ex, ey - 2);
            if board.check_beaten(player, middle_piece, outer_piece) {
                board = board.set(ex, ey - 1, 0);
            }
        }
        // check bottom
        if ey < 3 {
            let middle_piece = board.get(ex, ey + 1);
            let outer_piece = board.get(ex, ey + 2);
            if board.check_beaten(player, middle_piece, outer_piece) {
                board = board.set(ex, ey + 1, 0);
            }
        }
        board.next_player()
    }

    pub fn get_max_moves_piece(&self, x: u8, y: u8) -> (u8, u8, u8, u8){
        // check free moves in top direction, from y to y=0 y excluded
        let mut min_y_move = y;
        for j in (0..y).rev() {
            if self.get(x, j) != 0 {break} 
            else {
                min_y_move = j;
            }
        }

        // check free moves in bottom direction, from y+1 to y=4
        let mut max_y_move = y;
        for j in y+1..5 {
            if self.get(x, j) != 0 {break}
            else {
                max_y_move = j;
            }
        }

        // check free moves in left direction, from x to x=0, x excluded
        let mut min_x_move = x;
        for i in (0..x).rev() {
            if self.get(i, y) != 0 {break}
            else {
                min_x_move = i;
            }
        }

        let mut max_x_move = x;
        for i in x+1..5 {
            if self.get(i, y) != 0 {break}
            else {
                max_x_move = i;
            }
        }

        return (min_x_move, max_x_move, min_y_move, max_y_move);
    }

    fn get_possible_moves_player<const PLAYER: u8>(&self) -> Vec<TMove> {
        let mut moves = Vec::new();
        for i in 0..5u8 {
            for j in 0..5u8 {
                if ((PLAYER == 0) & (self.get(i, j) > 1)) | ((PLAYER == 1) & (self.get(i, j) == 1)){
                    let (min_x_move, max_x_move, min_y_move, max_y_move) = self.get_max_moves_piece(i, j);

                    // add moves in x direction to possible moves
                    for m_i in min_x_move..i {moves.push(TMove{start: (i, j), end: (m_i, j)})}
                    for m_i in i+1..=max_x_move {moves.push(TMove{start: (i, j), end: (m_i, j)})}
                    
                    // add moves in y direction to possible moves
                    for m_j in min_y_move..j {moves.push(TMove{start: (i, j), end: (i, m_j)})}
                    for m_j in j+1..=max_y_move {moves.push(TMove{start: (i, j), end: (i, m_j)})}
                }
            }
        }
        moves
    }

    pub fn get_possible_moves(&self) -> Vec<TMove>{
        if self.get_player() == 0 {
            return self.get_possible_moves_player::<0>();
        } else {
            return self.get_possible_moves_player::<1>();
        }
    }

    fn get_only_black_board(&self) -> u64 {
        let bm1 = 0b1010101010_1010101010_1010101010_1010101010_1010101010u64;
        let bm2 = bm1 >> 1;
        ((self.board & bm2)) & (!(self.board & bm1) >> 1)
    }

    fn count_white_blocked(&self, b_only_black: u64, i: u8, j: u8) -> i32 {
        // row mask shifted to correct row
        let row_mask = 0b1111111111u64;
        // we only want to consider everything to the left or right of i
        let row_mask_left = row_mask & (row_mask << (5 - i) * 2);
        let row_mask_right = row_mask >> (i + 1) * 2;

        let blocking_left = (b_only_black & (row_mask_left << (4 - j) * 10)) > 0;
        let blocking_right = (b_only_black & (row_mask_right << (4 - j) * 10)) > 0;
        let blocking_x = blocking_left as i32 + blocking_right as i32;

        let col_mask = 0b0000000011_0000000011_0000000011_0000000011_0000000011u64;
        let col_mask_left = col_mask & (col_mask << (5 - j) * 10);
        let col_mask_right = col_mask >> (j + 1) * 10;

        let blocking_top = (b_only_black & (col_mask_left << (4 - i)*2)) > 0;
        let blocking_down = (b_only_black & (col_mask_right << (4 - i)*2)) > 0;
        let blocking_y = blocking_top as i32 + blocking_down as i32;
        
        return blocking_x + blocking_y;
    }


    pub fn eval(&self) -> i32 {
        let b_board = Board { board: self.get_only_black_board() };
        let mut score = 0i32;

        for i in 0..5 {
            for j in 0..5 {
                let piece = self.get(i, j);
                match piece {
                    1 => { score -= 7 },
                    2..=3 => {
                        if i == 0 || i == 4 || j == 0 || j == 4 { score += 12 }
                        else if i == 1 || i == 3 || j == 1 || j == 3 { score += 6 }

                        score += 2 * (4 - self.count_white_blocked(b_board.board, i, j));


                        if piece == 3 && (
                            (i > 0) && (b_board.get(i - 1, j) != 0) ||
                            (i < 4) && (b_board.get(i + 1, j) != 0) ||
                            (j > 0) && (b_board.get(i, j - 1) != 0) ||
                            (j < 4) && (b_board.get(i, j + 1) != 0)
                        ) { 
                            score -= 5;
                        }
                    },
                    _ => {}
                }
            }
        }
        return score * (-2 * self.get_player() as i32 + 1);
    }

    pub fn print_board(&self) {
        let player_string = format!("P{}", self.get_player() + 1);
        println!("{} ABCDE", match self.get_player() {
            0 => player_string.blue(),
            1 => player_string.black(),
            _ => player_string.white()
        });
        println!("  +-----+ ");
        for j in 0..5 {
            print!("{} |", 5 - j);
            for i in 0..5 {
                print!("{}", match self.get(i, j) {
                    1 => "+".black(),
                    2 => "+".blue(),
                    3 => "K".blue(),
                    _ => " ".white()
                });
            }
            println!("|");
        }
        println!("  +-----+ ");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let mut board = Board { board: 0b0_0100010001_0000100000_0110111001_0000100000_0100010001 };
        let moves = vec![
            TMove { start: (3, 2), end: (3, 0) }, // d
            TMove { start: (4, 2), end: (3, 2) }, // d
            TMove { start: (2, 3), end: (3, 3) }, // d
            TMove { start: (2, 4), end: (2, 3) }, // d
            TMove { start: (3, 0), end: (3, 1) }, // d
            TMove { start: (4, 4), end: (4, 2) }, //
            TMove { start: (3, 1), end: (4, 1) }, //
            TMove { start: (4, 2), end: (3, 2) }, //
            TMove { start: (4, 1), end: (3, 1) }, //
            TMove { start: (4, 0), end: (4, 2) }  //
        ];
        board.print_board();
        for tmove in moves {
            println!("{}", match board.get_player() {
                0 => tmove.to_string().blue(),
                1 => tmove.to_string().black(),
                _ => tmove.to_string().white()
            });
            println!("");

            board = board.make_move(&tmove);
            board.print_board();
            println!("Eval: {}", board.eval());
        }
        match board.get_winner() {
            Some(x) => println!("Player {} won", x+1),
            None => println!("Game is undecided!")
        }
    }

    #[test]
    fn test_move() {
        /* Test configuration:
            Player: 2
              0 1 2 3 4
            0 X _ _ _ _
            1 X _ O X _
            2 X O _ O X
            3 X _ K _ X
            4 _ _ _ X X
         */
        let board = Board { board: 0b1_0100000000_0100100100_0110001001_0100110001_0000000101 };
        let b_only_black = board.get_only_black_board();
        let block_1 = board.count_white_blocked(b_only_black, 2, 3);
        let block_2 = board.count_white_blocked(b_only_black, 3, 0);
        assert_eq!(block_1, 2);
        assert_eq!(block_2, 2);
    }
}