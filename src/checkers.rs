// RED: -1
// RED KING: -3
// BLACK: 1
// BLACK KING: 3

use std::thread::JoinHandle;

use thiserror::Error;

pub struct Checkers {
    board_state: [[i8; 8]; 8],
    current_turn: i8
}

#[derive(Error, Debug)]
pub enum CheckersError {
    #[error("none of own piece at this square")]
    WrongPiece,
    #[error("cannot move here")]
    ImpossibleMove
}

impl Checkers {
    const RED_PIECE: i8 = -1;
    const RED_KING: i8 = -3;
    const BLACK_PIECE: i8 = 1;
    const BLACK_KING: i8 = 3;

    pub fn new() -> Result<Self, ()> {
        let mut board_state: [[i8; 8]; 8] = [[0; 8]; 8];
        for j in 0..3 {
            for i in 0..8 {
                if j % 2 == 0 {
                    if i % 2 == 1 {
                        board_state[j][i] = Self::RED_PIECE;
                    }
                } else {
                    if i % 2 == 0 {
                        board_state[j][i] = Self::RED_PIECE;
                    }
                }
            }
        }

        for j in 5..8 {
            for i in 0..8 {
                if j % 2 == 0 {
                    if i % 2 == 1 {
                        board_state[j][i] = Self::BLACK_PIECE;
                    }
                } else {
                    if i % 2 == 0 {
                        board_state[j][i] = Self::BLACK_PIECE;
                    }
                }
            }
        }

        for j in 0..8 {
            let mut string = String::new();
            for i in 0..8 {
                string.push_str(board_state[j][i].to_string().as_str());
            }
            println!("{}", string);
        }
        let checkers = Self {
            board_state,
            current_turn: Self::BLACK_PIECE,
        };
        Ok(checkers)
    }

    pub fn is_game_over(&self) -> bool {
        let mut still_red: bool = false;
        let mut still_black: bool = false;
        for j in 0..8 {
            for i in 0..8 {
                if self.board_state[j][i] < 0 {
                    still_red = true;
                } else if self.board_state[j][i] > 0 {
                    still_black = true;
                }
            }
        }
        return !still_red || !still_black
    }

    pub fn get_current_player(&self) -> i8 {
        return self.current_turn;
    }

    fn can_move_up_right (&self, j_coord: usize, i_coord: usize) -> bool {
        if j_coord == 0 || i_coord == 7 {
            return false
        }
        match self.board_state[j_coord][i_coord] {
            -1 => return false,
            0 => return false, 
            _ => return self.board_state[j_coord - 1][i_coord + 1] == 0
        }
    }

    fn can_jump_up_right (&self, j_coord: usize, i_coord: usize) -> bool {
        if j_coord < 2 || i_coord > 5 {
            return false
        }
        match self.board_state[j_coord][i_coord] {
            -1 => return false,
            0 => return false, 
            -3 => return (self.board_state[j_coord - 1][i_coord + 1] > 0 && self.board_state[j_coord - 2][i_coord + 2] == 0),
            _ => return (self.board_state[j_coord - 1][i_coord + 1] < 0 && self.board_state[j_coord - 2][i_coord + 2] == 0),
        }
    }

    fn can_move_up_left (&self, j_coord: usize, i_coord: usize) -> bool {
        if j_coord == 0 || i_coord == 0 {
            return false
        }
        match self.board_state[j_coord][i_coord] {
            -1 => return false,
            0 => return false, 
            _ => return self.board_state[j_coord - 1][i_coord - 1] == 0
        }
    }

    fn can_jump_up_left (&self, j_coord: usize, i_coord: usize) -> bool {
        if j_coord < 2 || i_coord < 2 {
            return false
        }
        match self.board_state[j_coord][i_coord] {
            -1 => return false,
            0 => return false, 
            -3 => return (self.board_state[j_coord - 1][i_coord - 1] > 0 && self.board_state[j_coord - 2][i_coord - 2] == 0),
            _ => return (self.board_state[j_coord - 1][i_coord - 1] < 0 && self.board_state[j_coord - 2][i_coord - 2] == 0),
        }
    }

    fn can_move_down_left (&self, j_coord: usize, i_coord: usize) -> bool {
        if j_coord == 7 || i_coord == 0 {
            return false
        }
        match self.board_state[j_coord][i_coord] {
            1 => return false,
            0 => return false, 
            _ => return self.board_state[j_coord + 1][i_coord - 1] == 0
        }
    }

    fn can_jump_down_left (&self, j_coord: usize, i_coord: usize) -> bool {
        if j_coord > 5 || i_coord < 2 {
            return false
        }
        match self.board_state[j_coord][i_coord] {
            1 => return false,
            0 => return false, 
            3 => return (self.board_state[j_coord + 1][i_coord - 1] < 0 && self.board_state[j_coord + 2][i_coord - 2] == 0),
            _ => return (self.board_state[j_coord + 1][i_coord - 1] > 0 && self.board_state[j_coord + 2][i_coord - 2] == 0),
        }
    }

    fn can_move_down_right (&self, j_coord: usize, i_coord: usize) -> bool {
        if j_coord == 7 || i_coord == 7 {
            return false
        }
        match self.board_state[j_coord][i_coord] {
            1 => return false,
            0 => return false, 
            _ => return self.board_state[j_coord + 1][i_coord + 1] == 0
        }
    }

    fn can_jump_down_right (&self, j_coord: usize, i_coord: usize) -> bool {
        if j_coord > 5 || i_coord > 5 {
            return false
        }
        match self.board_state[j_coord][i_coord] {
            1 => return false,
            0 => return false, 
            3 => return (self.board_state[j_coord + 1][i_coord + 1] < 0 && self.board_state[j_coord + 2][i_coord + 2] == 0),
            _ => return (self.board_state[j_coord + 1][i_coord + 1] > 0 && self.board_state[j_coord + 2][i_coord + 2] == 0),
        }
    }

    pub fn get_possible_moves(&self, j_coord: usize, i_coord: usize, team: i8) -> Result<Option<Vec<(usize, usize, bool)>>, CheckersError> {
        if self.board_state[j_coord][i_coord] < 0 && team > 0 || self.board_state[j_coord][i_coord] > 0 && team < 0 || self.board_state[j_coord][i_coord] == 0 {
            return Err(CheckersError::WrongPiece)
        }
        let mut return_vec: Vec<(usize, usize, bool)> = Vec::new();
        if self.can_move_up_right(j_coord, i_coord) {
            return_vec.push((j_coord - 1, i_coord + 1, false));
        } else if self.can_jump_up_right(j_coord, i_coord) {
            return_vec.push((j_coord - 2, i_coord + 2, true));
        }

        if self.can_move_up_left(j_coord, i_coord) {
            return_vec.push((j_coord - 1, i_coord - 1, false));
        } else if self.can_jump_up_left(j_coord, i_coord) {
            return_vec.push((j_coord - 2, i_coord - 2, true));
        }

        if self.can_move_down_right(j_coord, i_coord) {
            return_vec.push((j_coord + 1, i_coord + 1, false));
        } else if self.can_jump_down_right(j_coord, i_coord) {
            return_vec.push((j_coord + 2, i_coord + 2, true));
        }

        if self.can_move_down_left(j_coord, i_coord) {
            return_vec.push((j_coord + 1, i_coord - 1, false));
        } else if self.can_jump_down_left(j_coord, i_coord) {
            return_vec.push((j_coord + 2, i_coord - 2, true));
        }
        
        if return_vec.is_empty() {
            return Ok(None);
        } else {
            Ok(Some(return_vec))
        }

    }

    pub fn set_square(&mut self, square_j: usize, square_i: usize, value: i8) {
        self.board_state[square_j][square_i] = value;
    }

    pub fn make_move (&mut self, move_from_j: usize, move_from_i: usize, move_to_j: usize, move_to_i: usize) -> Result<(), CheckersError> {
        if (move_from_j > 7 || move_to_j > 7 || move_from_j < 0 || move_to_j < 0) {
            return Err(CheckersError::ImpossibleMove)
        }
        if self.board_state[move_from_j][move_from_i] == 0 || self.board_state[move_to_j][move_to_i] != 0 {
            return Err(CheckersError::ImpossibleMove)
        }
        self.set_square(move_to_j, move_to_i, self.board_state[move_from_j][move_from_i]);
        self.set_square(move_from_j, move_from_i, 0);
        if self.current_turn < 0 {
            self.current_turn = 1;
        } else if self.current_turn > 0 {
            self.current_turn = -1;
        }
        Ok(())
    }
}