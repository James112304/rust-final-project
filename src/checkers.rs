// RED: -1
// RED KING: -3
// BLACK: 1
// BLACK KING: 3


use thiserror::Error;
use std::sync::Mutex;
use std::sync::Arc;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub enum Move {
    UpLeft(usize),
    DownLeft(usize), 
    UpRight(usize), 
    DownRight(usize),
    Sequence(Vec<Move>)
}


#[derive(Error, Debug, Clone, Copy)]
pub enum CheckersError {
    #[error("none of own piece at this square")]
    WrongPiece,
    #[error("cannot move here")]
    ImpossibleMove,
    #[error("no moves left to make")] 
    GameOver
}

#[derive(Clone, Copy)]
pub struct Checkers {
    pub board_state: [[i32; 8]; 8],
    pub current_turn: i32,
    pub required_square: Option<(usize, usize)>,
    pub game_over: bool,
    pub calculating: bool
}


impl Checkers {
    const RED_PIECE: i32 = -1;
    const RED_KING: i32 = -3;
    const BLACK_PIECE: i32 = 1;
    const BLACK_KING: i32 = 3;

    pub fn new() -> Result<Self, ()> {
        let mut board_state: [[i32; 8]; 8] = [[0; 8]; 8];
        for j in 0..3 {
            for i in 0..8 {
                if j % 2 == 0 {
                    if i % 2 == 1 {
                        board_state[j][i] = Self::RED_PIECE;
                    }
                } else if i % 2 == 0 {
                    board_state[j][i] = Self::RED_PIECE;
                }
            }
        }

        for j in 5..8 {
            for i in 0..8 {
                if j % 2 == 0 {
                    if i % 2 == 1 {
                        board_state[j][i] = Self::BLACK_PIECE;
                    }
                } else if i % 2 == 0 {
                    board_state[j][i] = Self::BLACK_PIECE;
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
            required_square: None,
            game_over: false, 
            calculating: false
        };
        Ok(checkers)
    }

    pub fn is_game_over(&self) -> bool {
        return self.black_won() || self.red_won()
    }

    pub fn red_won(&self) -> bool {
        if self.current_turn < 0 {
            return false;
        }
        for j in 0..8 {
            for i in 0..8 {
                if self.board_state[j][i] > 0 {
                    match self.get_possible_moves(j, i, self.board_state[j][i]) {
                        Ok(option) => {
                            match option {
                                Some(_vec) => return false,
                                None => ()
                            }
                        },
                        Err(e) => eprintln!("error {}", e)
                    }
                }
            }
        }
        return true
    }

    pub fn black_won(&self) -> bool {
        if self.current_turn < 0 {
            return false;
        }
        for j in 0..8 {
            for i in 0..8 {
                if self.board_state[j][i] < 0 {
                    match self.get_possible_moves(j, i, self.board_state[j][i]) {
                        Ok(option) => {
                            match option {
                                Some(_vec) => return false,
                                None => ()
                            }
                        },
                        Err(e) => eprintln!("error {}", e)
                    }
                }
            }
        }
        return true
    }

    fn can_move_up_right (&self, j_coord: usize, i_coord: usize) -> bool {
        if j_coord == 0 || i_coord == 7 {
            return false
        }
        match self.board_state[j_coord][i_coord] {
            -1 => false,
            0 => false, 
            _ => self.board_state[j_coord - 1][i_coord + 1] == 0
        }
    }

    fn can_jump_up_right (&self, j_coord: usize, i_coord: usize) -> bool {
        if j_coord < 2 || i_coord > 5 {
            return false
        }
        match self.board_state[j_coord][i_coord] {
            -1 => false,
            0 => false, 
            -3 => self.board_state[j_coord - 1][i_coord + 1] > 0 && self.board_state[j_coord - 2][i_coord + 2] == 0,
            _ => self.board_state[j_coord - 1][i_coord + 1] < 0 && self.board_state[j_coord - 2][i_coord + 2] == 0,
        }
    }

    fn can_move_up_left (&self, j_coord: usize, i_coord: usize) -> bool {
        if j_coord == 0 || i_coord == 0 {
            return false
        }
        match self.board_state[j_coord][i_coord] {
            -1 => false,
            0 => false, 
            _ => self.board_state[j_coord - 1][i_coord - 1] == 0
        }
    }

    fn can_jump_up_left (&self, j_coord: usize, i_coord: usize) -> bool {
        if j_coord < 2 || i_coord < 2 {
            return false
        }
        match self.board_state[j_coord][i_coord] {
            -1 => false,
            0 => false, 
            -3 => self.board_state[j_coord - 1][i_coord - 1] > 0 && self.board_state[j_coord - 2][i_coord - 2] == 0,
            _ => self.board_state[j_coord - 1][i_coord - 1] < 0 && self.board_state[j_coord - 2][i_coord - 2] == 0,
        }
    }

    fn can_move_down_left (&self, j_coord: usize, i_coord: usize) -> bool {
        if j_coord == 7 || i_coord == 0 {
            return false
        }
        match self.board_state[j_coord][i_coord] {
            1 => false,
            0 => false, 
            _ => self.board_state[j_coord + 1][i_coord - 1] == 0
        }
    }

    fn can_jump_down_left (&self, j_coord: usize, i_coord: usize) -> bool {
        if j_coord > 5 || i_coord < 2 {
            return false
        }
        match self.board_state[j_coord][i_coord] {
            1 => false,
            0 => false, 
            3 => self.board_state[j_coord + 1][i_coord - 1] < 0 && self.board_state[j_coord + 2][i_coord - 2] == 0,
            _ => self.board_state[j_coord + 1][i_coord - 1] > 0 && self.board_state[j_coord + 2][i_coord - 2] == 0,
        }
    }

    fn can_move_down_right (&self, j_coord: usize, i_coord: usize) -> bool {
        if j_coord == 7 || i_coord == 7 {
            return false
        }
        match self.board_state[j_coord][i_coord] {
            1 => false,
            0 => false, 
            _ => self.board_state[j_coord + 1][i_coord + 1] == 0
        }
    }

    fn can_jump_down_right (&self, j_coord: usize, i_coord: usize) -> bool {
        if j_coord > 5 || i_coord > 5 {
            return false
        }
        match self.board_state[j_coord][i_coord] {
            1 => false,
            0 => false, 
            3 => self.board_state[j_coord + 1][i_coord + 1] < 0 && self.board_state[j_coord + 2][i_coord + 2] == 0,
            _ => self.board_state[j_coord + 1][i_coord + 1] > 0 && self.board_state[j_coord + 2][i_coord + 2] == 0,
        }
    }

    pub fn get_possible_moves(&self, j_coord: usize, i_coord: usize, team: i32) -> Result<Option<Vec<(usize, usize, bool)>>, CheckersError> {
        if self.board_state[j_coord][i_coord] < 0 && team > 0 || self.board_state[j_coord][i_coord] > 0 && team < 0 || self.board_state[j_coord][i_coord] == 0 {
            return Err(CheckersError::WrongPiece)
        }
        let mut return_vec: Vec<(usize, usize, bool)> = Vec::new();
        if let Some((_j, _i)) = self.required_square {
            if self.can_jump_up_right(j_coord, i_coord) {
                return_vec.push((j_coord - 2, i_coord + 2, true));
            }
            if self.can_jump_up_left(j_coord, i_coord) {
                return_vec.push((j_coord - 2, i_coord - 2, true));
            }
            if self.can_jump_down_right(j_coord, i_coord) {
                return_vec.push((j_coord + 2, i_coord + 2, true));
            }
            if self.can_jump_down_left(j_coord, i_coord) {
                return_vec.push((j_coord + 2, i_coord - 2, true));
            }
            if return_vec.is_empty() {
                return Ok(None)
            } else {
                return Ok(Some(return_vec))
            }
        }
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
            Ok(None)
        } else {
            Ok(Some(return_vec))
        }

    }

    pub fn get_all_possible_moves(&self, j_coord: usize, i_coord: usize) -> Option<Vec<Move>> {
        let mut clone = *self;
        let mut return_vec: Vec<Move> = Vec::new();

        if self.board_state[j_coord][i_coord] > 0 && self.current_turn < 0  || self.board_state[j_coord][i_coord] < 0 && self.current_turn > 0{
            return None;
        }

        if self.can_move_up_right(j_coord, i_coord) {
            return_vec.push(Move::UpRight(1));
        } 
        if self.can_move_up_left(j_coord, i_coord) {
            return_vec.push(Move::UpLeft(1));
        } 

        if self.can_move_down_right(j_coord, i_coord) {
            return_vec.push(Move::DownRight(1));
        } 

        if self.can_move_down_left(j_coord, i_coord) {
            return_vec.push(Move::DownLeft(1));
        } 
        let (_num, jumped_from_here) = clone.how_many_jumped_from_here(j_coord, i_coord, self.board_state[j_coord][i_coord]);
        
        match jumped_from_here {
            Some(vec) => {
                for s in vec {
                    return_vec.push(s);
                }
            },
            None => ()
        }

        if return_vec.is_empty() {
            return None
        } else {
            return Some(return_vec)
        }
    }

    pub fn can_make_move (&self, move_from_j: usize, move_from_i: usize, move_to_j: usize, move_to_i: usize) -> bool{
        if move_from_j > 7 || move_to_j > 7 {
            return false;
        }
        if self.board_state[move_from_j][move_from_i] == 0 || self.board_state[move_to_j][move_to_i] != 0 {
            return false;
        }
        match self.get_possible_moves(move_from_j, move_from_i, self.board_state[move_from_j][move_from_i]) {
            Ok(possible_moves_from_square) => {
                if possible_moves_from_square.is_none() {
                    return false;
                } else {
                    for (j, i, _b) in possible_moves_from_square.unwrap() {
                        if j == move_to_j && i == move_to_i {
                            return true
                        }
                    }
                    return false
                }
            },
            Err(e) => {
                eprintln!("error: {}", e); 
                return false;
            }
        }
        
    }

    pub fn make_move (&mut self, move_from_j: usize, move_from_i: usize, move_to_j: usize, move_to_i: usize) -> Result<(), CheckersError> {
        //println!("before move {:?}", self.board_state);
        if move_from_j > 7 || move_to_j > 7 {
            return Err(CheckersError::ImpossibleMove)
        }
        if self.board_state[move_from_j][move_from_i] == 0 || self.board_state[move_to_j][move_to_i] != 0 {
            return Err(CheckersError::ImpossibleMove)
        }
        if self.board_state[move_from_j][move_from_i] < 0 && self.current_turn > 0 {
            return Err(CheckersError::ImpossibleMove)
        }
        if self.board_state[move_from_j][move_from_i] > 0 && self.current_turn < 0 {
            return Err(CheckersError::ImpossibleMove)
        }
        if self.board_state[move_from_j][move_from_i] == -1 && move_to_j == 7 {
            self.board_state[move_from_j][move_from_i] = -3;
        } else if self.board_state[move_from_j][move_from_i] == 1 && move_to_j == 0 {
            self.board_state[move_from_j][move_from_i] = 3;
        }
        self.board_state[move_to_j][move_to_i] = self.board_state[move_from_j][move_from_i];
        self.board_state[move_from_j][move_from_i] = 0;

        if std::cmp::max(move_from_i, move_to_i) - std::cmp::min(move_from_i, move_to_i) > 1 {
            self.board_state[(move_from_j + move_to_j) / 2][(move_from_i + move_to_i) / 2] = 0;
            if let Some(vec) = self.get_possible_moves(move_to_j, move_to_i, self.board_state[move_to_j][move_to_i]).unwrap() {
                for (_j, _i, b) in vec {
                    if b {
                        //println!("yes");
                        self.required_square = Some((move_to_j, move_to_i));
                        return Ok(())
                    }
                }
                self.required_square = None;
            }
        }
        if self.current_turn < 0 {
            self.current_turn = 1;
            if self.is_game_over() {
                self.game_over = true
            }
        } else if self.current_turn > 0 {
            self.current_turn = -1;
        }
        //println!("after move {:?}", self.board_state);
        self.required_square = None;
        Ok(())
    }

    pub fn make_move_then_ai(&mut self, move_from_j: usize, move_from_i: usize, move_to_j: usize, move_to_i: usize) -> Result<(), CheckersError> {
        let stored_state = self.current_turn;
        self.make_move(move_from_j, move_from_i, move_to_j, move_to_i);
        if stored_state > 0 && self.current_turn > 0 || stored_state < 0 && self.current_turn < 0 {
            return Ok(())
        } else {
            match self.get_best_move(7, self.current_turn) {
                Ok((j, i, m)) => {
                    println!("best move for {} is {}, {}, {:?}", self.current_turn, j, i, m);
                    if self.current_turn < 0 {
                        println!("got here");
                        self.make_move_from_enum(j, i, &m);
                    }
                    return Ok(())
                },
                Err(e) => return Err(e)
            } 
        }

    }
    pub fn make_move_from_enum (&mut self, move_from_j: usize, move_from_i: usize, move_to_make: &Move) -> Result<((usize, usize)), CheckersError> {
        let mut current_j = move_from_j;
        let mut current_i = move_from_i;
        match move_to_make {
            Move::Sequence(sequence) => {
                for m in sequence {
                    match m {
                        Move::UpLeft(size) => {
                            self.make_move(current_j, current_i, current_j - size, current_i - size);
                            current_i -= size;
                            current_j -= size;
                        }, 
                        Move::UpRight(size) => {
                            self.make_move(current_j, current_i, current_j - size, current_i + size);
                            current_i += size;
                            current_j -= size;
                        }, 
                        Move::DownLeft(size) => {
                            self.make_move(current_j, current_i, current_j + size, current_i - size);
                            current_i -= size;
                            current_j += size;
                        },
                        Move::DownRight(size) => {
                            self.make_move(current_j, current_i, current_j + size, current_i + size);
                            current_i += size;
                            current_j += size;
                        }, 
                        _ => {
                            println!("{:?}", move_to_make);
                            panic!("how got composite in composite")
                        }
                    }
                }
            }, 
            Move::UpLeft(size) => {
                if *size == 1 && !self.can_move_up_left(current_j, current_i){
                    println!("{:?}", self.board_state);
                    println!("{:?}, ({}, {})", move_to_make, current_j, current_i);
                    panic!("how");
                } else if *size == 2 && !self.can_jump_up_left(current_j, current_i) {
                    println!("{:?}", self.board_state);
                    println!("{:?}, ({}, {})", move_to_make, current_j, current_i);
                    panic!("how");
                }
                match self.make_move(current_j, current_i, current_j - size, current_i - size) {
                    Ok(_) => (),
                    Err(e) => panic!("error: {}", e)
                };
                current_i -= size;
                current_j -= size;
            }, 
            Move::UpRight(size) => {
                if *size == 1 && !self.can_move_up_right(current_j, current_i){
                    println!("{:?}", self.board_state);
                    println!("{:?}, ({}, {})", move_to_make, current_j, current_i);
                    panic!("how");
                } else if *size == 2 && !self.can_jump_up_right(current_j, current_i) {
                    println!("{:?}", self.board_state);
                    println!("{:?}, ({}, {})", move_to_make, current_j, current_i);
                    panic!("how");
                }
                
                match self.make_move(current_j, current_i, current_j - size, current_i + size) {
                    Ok(_) => (),
                    Err(e) => panic!("error: {}", e)
                };
                current_i += size;
                current_j -= size;
            }, 
            Move::DownLeft(size) => {
                if *size == 1 && !self.can_move_down_left(current_j, current_i){
                    println!("{:?}", self.board_state);
                    println!("{:?}, ({}, {})", move_to_make, current_j, current_i);
                    panic!("how");
                } else if *size == 2 && !self.can_jump_down_left(current_j, current_i) {
                    println!("{:?}", self.board_state);
                    println!("{:?}, ({}, {})", move_to_make, current_j, current_i);
                    panic!("how");
                }
                match self.make_move(current_j, current_i, current_j + size, current_i - size) {
                    Ok(_) => (),
                    Err(e) => panic!("error: {}", e)
                }
                current_i -= size;
                current_j += size;
            },
            Move::DownRight(size) => {
                if *size == 1 && !self.can_move_down_right(current_j, current_i){
                    println!("{:?}", self.board_state);
                    println!("{:?}, ({}, {})", move_to_make, current_j, current_i);
                    panic!("how");
                } else if *size == 2 && !self.can_jump_down_right(current_j, current_i) {
                    println!("{:?}", self.board_state);
                    println!("{:?}, ({}, {})", move_to_make, current_j, current_i);
                    panic!("how");
                }
                match self.make_move(current_j, current_i, current_j + size, current_i + size) {
                    Ok(_) => (),
                    Err(e) => panic!("error: {}", e)
                }

                current_i += size;
                current_j += size;
            }
        }
        Ok((current_j, current_i))
    }

    pub fn get_best_move(&mut self, max_depth: usize, _team: i32) -> Result<((usize, usize, Move)), CheckersError> {
        let default: i32;
        if self.current_turn < 0 {
            default = std::i32::MAX;
        } else {
            default = std::i32::MIN;
        }
        let best_move = Arc::new(Mutex::new(None));
        let best_move_from = Arc::new(Mutex::new(None));
        let best_move_score = Arc::new(Mutex::new(default));
        
        (0..8).into_par_iter().for_each(|j| {
            (0..8).into_par_iter().for_each(|i| {
                if self.board_state[j][i] > 0 && self.current_turn > 0 {
                    if let Some(moves_to_make) = self.get_all_possible_moves(j, i) {
                        //println!("3 {}, {}", j, i);
                        //println!("{:?}", moves_to_make);
                        for m in moves_to_make {
                            let mut new_board = *self;
                            new_board.make_move_from_enum(j, i, &m);
                            let score = new_board.minimax(1, max_depth, false, std::i32::MIN, std::i32::MAX);
                            println!("score from {:?} from ({}, {}) is {}", m, j, i, score);
                            let mut best_score = best_move_score.lock().unwrap();
                            if score > *best_score {
                                println!("updating");
                                *best_score = score;
                                *best_move_from.lock().unwrap() = Some((j, i));
                                *best_move.lock().unwrap() = Some(m);
                            }
                        }
                    }
                    
                } else if self.board_state[j][i] < 0 && self.current_turn < 0 {
                    if let Some(moves_to_make) = self.get_all_possible_moves(j, i) {
                        //println!("3 {}, {}", j, i);
                        //println!("{:?}", moves_to_make);
                        for m in moves_to_make {
                            let mut new_board = *self;
                            new_board.make_move_from_enum(j, i, &m);
                            //println!("hello");
                            let score = new_board.minimax(1, max_depth, true, std::i32::MIN, std::i32::MAX);
                            println!("score from {:?} from ({}, {}) is {}", m, j, i, score);
                            let mut best_score = best_move_score.lock().unwrap();
                            if score < *best_score {
                                println!("updating");
                                *best_score = score;
                                *best_move_from.lock().unwrap() = Some((j, i));
                                *best_move.lock().unwrap() = Some(m);
                            }
                        }
                    }
                }
            });
        });
        
        let move_option = best_move.lock().unwrap();
        match move_option.as_ref() {
            Some(mov) => {
                match best_move_from.lock().unwrap().as_ref() {
                    Some((j, i)) => {
                        return Ok((*j, *i, mov.clone()))
                    }, 
                    None => return Err(CheckersError::GameOver)
                }
            },
            None => Err(CheckersError::GameOver)
        }

    }

    pub fn minimax(&mut self, current_depth: usize, max_depth: usize, is_maximizing_player: bool, mut alpha: i32, mut beta: i32) -> i32 {
        //println!("received");
        if current_depth == max_depth {
            let current = self.evaluate_board();
            //println!("evaluating... {}", current);
            return current
        } else {
            if is_maximizing_player {
                //println!("maximizing depth {}", current_depth);
                let mut bestVal: i32 = std::i32::MIN;
                for j in 0..8 {
                    for i in 0..8 {
                        if self.board_state[j][i] > 0 {
                            if let Some(vec) = self.get_all_possible_moves(j, i) {
                                for m in vec {
                                    let mut copy = *self;
                                    copy.make_move_from_enum(j, i, &m);
                                    let value = copy.minimax(current_depth + 1, max_depth, false, alpha, beta);
                                    bestVal = std::cmp::max(bestVal, value);
                                    alpha = std::cmp::max(alpha, bestVal);
                                    if beta <= alpha {
                                        break;
                                    }
                                }
                            } else {

                            }
                        }
                    }
                }
                return bestVal;
            } else {
                //println!("not maximizing depth {}", current_depth);
                let mut bestVal: i32 = std::i32::MAX;
                for j in 0..8 {
                    for i in 0..8 {
                        if self.board_state[j][i] < 0 {
                            if let Some(vec) = self.get_all_possible_moves(j, i) {
                                for m in vec {
                                    let mut copy = *self;
                                    copy.make_move_from_enum(j, i, &m);
                                    let value = copy.minimax(current_depth + 1, max_depth, true, alpha, beta);
                                    bestVal = std::cmp::min(bestVal, value);
                                    beta = std::cmp::min(beta, bestVal);
                                    if beta <= alpha {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                return bestVal;
            }
        }


    }

    pub fn evaluate_board(&mut self) -> i32 {
        let mut score = 0;
        if self.red_won() {
            println!("red won");
            return std::i32::MIN;
        } else if self.black_won() {
            println!("black won");
            return std::i32::MAX;
        }
        score += self.count_material() * 5;
        score += self.count_vulnerable_red() ;
        //println!("vulnerable red: {}", self.count_vulnerable_red());
        score -= self.count_vulnerable_black();
        //println!("vulnerable black: {}", self.count_vulnerable_black());
        return score
    }

    pub fn count_material(&self) -> i32 {
        let mut total: i32 = 0;
        let mut black_piece = 0;
        let mut red_piece = 0;
        for j in 0..8 {
            for i in 0..8 {
                total += self.board_state[j][i];
                if self.board_state[j][i] < 0 {
                    red_piece += 1;
                } else if self.board_state[j][i] > 0 {
                    black_piece +=1;
                }
            }
        }
        //println!("total black: {}", black_piece);
        //println!("total red: {}", red_piece);
        return total
    }

    pub fn how_many_jumped_from_here(&mut self, j_coord: usize, i_coord:usize, team: i32) -> (i32, Option<Vec<Move>>) {
        let mut count = 0;
        let mut moves_vec: Vec<Move> = Vec::new();
        if self.can_jump_down_left(j_coord, i_coord) {
            count += 1;
            let current_store  = self.board_state[j_coord][i_coord];
            self.board_state[j_coord][i_coord] = 0;
            let store = self.board_state[j_coord + 1][i_coord - 1];
            self.board_state[j_coord + 1][i_coord - 1] = 0;
            if (j_coord + 2 == 7) && team == -1 {
                self.board_state[j_coord + 2][i_coord - 2] = -3; 
            } else {
                self.board_state[j_coord + 2][i_coord - 2] = current_store;
            }    
            let (add_to_count, moves) = self.how_many_jumped_from_here(j_coord + 2, i_coord - 2, team);
            count += add_to_count;
            match moves {
                Some(vec) => {
                    for m in vec {
                        match m {
                            Move::Sequence(v) => {
                                let mut anothavec: Vec<Move> = Vec::new();     
                                anothavec.push(Move::DownLeft(2));   
                                for moo in v {
                                    anothavec.push(moo);
                                }
                                moves_vec.push(Move::Sequence(anothavec));
                            }
                            _ => {
                                let mut new_move_vec: Vec<Move> = Vec::new();     
                                new_move_vec.push(Move::DownLeft(2));   
                                new_move_vec.push(m);
                                moves_vec.push(Move::Sequence(new_move_vec));
                            }
                        }
                    }
                },
                None => moves_vec.push(Move::DownLeft(2))
            };
            self.board_state[j_coord][i_coord] = current_store;
            self.board_state[j_coord + 1][i_coord - 1] = store;
            self.board_state[j_coord + 2][i_coord - 2] = 0;
        }

        if self.can_jump_down_right(j_coord, i_coord) {
            count += 1;
            let current_store = self.board_state[j_coord][i_coord];
            self.board_state[j_coord][i_coord] = 0;
            let store = self.board_state[j_coord + 1][i_coord + 1];
            self.board_state[j_coord + 1][i_coord + 1] = 0;
            if (j_coord + 2 == 7) && team == -1 {
                self.board_state[j_coord + 2][i_coord + 2] = -3; 
            } else {
                self.board_state[j_coord + 2][i_coord + 2] = team;
            }
            let (add_to_count, moves) = self.how_many_jumped_from_here(j_coord + 2, i_coord + 2, team);
            count += add_to_count;
            match moves {
                Some(vec) => {
                    for m in vec {
                        match m {
                            Move::Sequence(v) => {
                                let mut anothavec: Vec<Move> = Vec::new();     
                                anothavec.push(Move::DownRight(2));   
                                for moo in v {
                                    anothavec.push(moo);
                                }
                                moves_vec.push(Move::Sequence(anothavec));
                            }
                            _ => {
                                let mut new_move_vec: Vec<Move> = Vec::new();     
                                new_move_vec.push(Move::DownRight(2));   
                                new_move_vec.push(m);
                                moves_vec.push(Move::Sequence(new_move_vec));
                            }
                        }
                    }
                },
                None => moves_vec.push(Move::DownRight(2))
            };
            self.board_state[j_coord][i_coord] = current_store;
            self.board_state[j_coord + 1][i_coord + 1] = store;
            self.board_state[j_coord + 2][i_coord + 2] = 0;
        }

        if self.can_jump_up_left(j_coord, i_coord) {
            count += 1;
            let current_store = self.board_state[j_coord][i_coord];
            self.board_state[j_coord][i_coord] = 0;
            let store = self.board_state[j_coord - 1][i_coord - 1];
            self.board_state[j_coord - 1][i_coord - 1] = 0;
            if (j_coord - 2 == 0) && team == 1 {
                self.board_state[j_coord - 2][i_coord - 2] = 3;
            } else {
                self.board_state[j_coord - 2][i_coord - 2] = team;
            }
            let (add_to_count, moves) = self.how_many_jumped_from_here(j_coord - 2, i_coord - 2, team);
            count += add_to_count;
            match moves {
                Some(vec) => {
                    for m in vec {
                        match m {
                            Move::Sequence(v) => {
                                let mut anothavec: Vec<Move> = Vec::new();     
                                anothavec.push(Move::UpLeft(2));   
                                for moo in v {
                                    anothavec.push(moo);
                                }
                                moves_vec.push(Move::Sequence(anothavec));
                            }
                            _ => {
                                let mut new_move_vec: Vec<Move> = Vec::new();     
                                new_move_vec.push(Move::UpLeft(2));   
                                new_move_vec.push(m);
                                moves_vec.push(Move::Sequence(new_move_vec));
                            }
                        }
                    }
                },
                None => moves_vec.push(Move::UpLeft(2))
            };
            self.board_state[j_coord][i_coord] = current_store;
            self.board_state[j_coord - 1][i_coord - 1] = store;
            self.board_state[j_coord - 2][i_coord - 2] = 0;
        }

        if self.can_jump_up_right(j_coord, i_coord) {
            count += 1;
            let current_store = self.board_state[j_coord][i_coord];
            self.board_state[j_coord][i_coord] = 0;
            let store = self.board_state[j_coord - 1][i_coord + 1];
            self.board_state[j_coord - 1][i_coord + 1] = 0;
            if (j_coord - 2 == 0) && team == 1 {
                self.board_state[j_coord - 2][i_coord + 2] = 3;
            } else {
                self.board_state[j_coord - 2][i_coord + 2] = team;
            }
            let (add_to_count, moves) = self.how_many_jumped_from_here(j_coord - 2, i_coord + 2, team);
            count += add_to_count;
            match moves {
                Some(vec) => {
                    for m in vec {
                        match m {
                            Move::Sequence(v) => {
                                let mut anothavec: Vec<Move> = Vec::new();     
                                anothavec.push(Move::UpRight(2));   
                                for moo in v {
                                    anothavec.push(moo);
                                }
                                moves_vec.push(Move::Sequence(anothavec));
                            }
                            _ => {
                                let mut new_move_vec: Vec<Move> = Vec::new();     
                                new_move_vec.push(Move::UpRight(2));   
                                new_move_vec.push(m);
                                moves_vec.push(Move::Sequence(new_move_vec));
                            }
                        }
                    }
                },
                None => moves_vec.push(Move::UpRight(2))
            };
            self.board_state[j_coord][i_coord] = current_store;
            self.board_state[j_coord - 1][i_coord + 1] = store;
            self.board_state[j_coord - 2][i_coord + 2] = 0;
        }
        if count == 0 {
            return (count, None)
        } else {
            return (count, Some(moves_vec))
        }
    }

    pub fn count_vulnerable_black(&mut self) -> i32 {
        let mut vulnerable_black = 0;
        for j in 0..8 {
            for i in 0..8 {
                if self.board_state[j][i] < 0 {
                    let _count = self.how_many_jumped_from_here(j, i, self.board_state[j][i]).0;
                    vulnerable_black += self.how_many_jumped_from_here(j, i, self.board_state[j][i]).0;
                }
            }
        }
        //println!("vulnerable black: {}", vulnerable_black);
        vulnerable_black
    }

    pub fn count_vulnerable_red(&mut self) -> i32 {
        let mut vulnerable_red = 0;
        for j in 0..8 {
            for i in 0..8 {
                if self.board_state[j][i] > 0 {
                    let count = self.how_many_jumped_from_here(j, i, self.board_state[j][i]).0;
                    vulnerable_red += count;
                }
            }
        }
        //println!("vulnerable red: {}", vulnerable_red);
        vulnerable_red
    }
}