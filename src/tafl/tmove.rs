use std::str::FromStr;



#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TMove {
    pub start: (u8, u8),
    pub end: (u8, u8)
}

#[derive(Debug)]
pub enum TMoveError {
    InvalidArgumentError
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

impl FromStr for TMove {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<char>>();
        if s.len() < 5 {
            return Err(TMoveError::InvalidArgumentError);
        }

        let x_from = chars[0];
        let y_from = chars[1];
        let x_to = chars[3];
        let y_to = chars[4];

        let x_char_to_number = |x: char| {
            match  x {
                'a' => Some(0u8),
                'b' => Some(1u8),
                'c' => Some(2u8),
                'd' => Some(3u8),
                'e' => Some(4u8),
                _ => None
            }
        };

        let y_char_to_numer = |x: char| {
            match x {
                '1' => Some(4u8),
                '2' => Some(3u8),
                '3' => Some(2u8),
                '4' => Some(1u8),
                '5' => Some(0u8),
                _ => None
            }
        };

        let start = (x_char_to_number(x_from), y_char_to_numer(y_from));
        let end = (x_char_to_number(x_to), y_char_to_numer(y_to));

        match (start, end) {
            ((Some(x_start), Some(y_start)), (Some(x_end), Some(y_end))) => { Ok(TMove { start: (x_start, y_start), end: (x_end, y_end) }) },
            _ => { return Err(TMoveError::InvalidArgumentError); }
        }
    }

    type Err = TMoveError;
}