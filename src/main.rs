mod checkers;

use std::thread::current;

use checkers::{Checkers};


use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    pub board_position: (usize, usize),
    pub ui_position: (f32, f32),
    pub piecekind: i8,
    pub is_dragging: bool,
}
impl Piece {
    pub fn new(jpos: usize, ipos: usize, kind: i8) -> Self {
        Self {
            board_position: (jpos, ipos),
            ui_position: (0., 0.),
            piecekind: kind, 
            is_dragging: false
        }
    }
}

#[macroquad::main("Checkers")]
async fn main() {
    let mut checkers: Checkers = Checkers::new().unwrap();
    let mut current_dragged: Option<(usize, usize)> = None;
    let mut piece_board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

    for j in 0..3 {
        for i in 0..8 {
            if j % 2 == 0 {
                if i % 2 == 1 {
                    piece_board[j][i] = Some(Piece::new(j, i, -1));
                }
            } else if i % 2 == 0 {
                piece_board[j][i] = Some(Piece::new(j, i, -1));
            }
        }
    }

    for j in 5..8 {
        for i in 0..8 {
            if j % 2 == 0 {
                if i % 2 == 1 {
                    piece_board[j][i] = Some(Piece::new(j, i, 1));
                }
            } else if i % 2 == 0 {
                piece_board[j][i] = Some(Piece::new(j, i, 1));
            }
        }
    }


    loop {

        clear_background(LIGHTGRAY);
        let black_turn = checkers.current_turn > 0;
        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let sq_size = (screen_height() - offset_y * 2.) / 8 as f32;
        let circle_radius = 2. * sq_size / 5.;

        draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

        paint_board();

        if let Some((j, i)) = current_dragged {
            //println!("moving here");
            if let Some(team) = piece_board[j][i] {
                let possible_places_vec: Option<Vec<(usize, usize, bool)>> = checkers.get_possible_moves(j, i, team.piecekind).unwrap();
                match possible_places_vec {
                    Some(vec) => {
                        for (j, i, b) in vec {
                            draw_circle(
                                offset_x + (i as f32 + 0.5) * sq_size,
                                offset_y + (j as f32 + 0.5) * sq_size,
                                sq_size / 4.,
                                DARKGRAY
                            )
                        }
                    },
                    None => (),
                }
            } else {
                panic!("how did we get here");
            }
        }
            
        let board_x = ((mouse_position().0 - offset_x) / sq_size).floor() as usize;
        let board_y = ((mouse_position().1 - offset_y) / sq_size).floor() as usize;

        if is_mouse_button_down(MouseButton::Left) {

            if let Some((dragged_j, dragged_i)) = current_dragged {
                if let Some(piece) = &mut piece_board[dragged_j][dragged_i] {
                    if piece.piecekind > 0 && black_turn || piece.piecekind < 0 && !black_turn {
                        // if circle_radius.powi(2) < ((mouse_position().0 - piece.ui_position.1).powi(2) + (mouse_position().1 - piece.ui_position.0).powi(2)) {
                        //     println!("here");
                        //     current_dragged = None;
                        //     piece.is_dragging = false;
                        // }
                    }
                }
            } else {
                if let Some(mut piece) = &mut piece_board[board_y][board_x] {
                    //println!("routed");
                    if piece.piecekind > 0 && black_turn || piece.piecekind < 0 && !black_turn {
                        if circle_radius.powi(2) > ((mouse_position().0 - piece.ui_position.1).powi(2) + (mouse_position().1 - piece.ui_position.0).powi(2)) {
                            piece.is_dragging = true;
                            current_dragged = Some((board_y, board_x));
                        } else {
                            current_dragged = None;
                            piece.is_dragging = false;
                        }
                        
                    }
                }
            }
        } else {
            if let Some(dragged_piece) = current_dragged {
                if checkers.can_make_move(
                    dragged_piece.0,
                        dragged_piece.1, 
                        board_y, 
                        board_x
                    ) {
                        match checkers.make_move(dragged_piece.0,
                            dragged_piece.1, 
                            board_y, 
                            board_x
                        ) {
                            Ok(_) => {
                                //black_turn = !black_turn;
                                piece_board[board_y][board_x] = piece_board[dragged_piece.0][dragged_piece.1];
                                piece_board[dragged_piece.0][dragged_piece.1] = None;
                                if std::cmp::max(board_x, dragged_piece.1) - std::cmp::min(board_x, dragged_piece.1) > 1 {
                                    piece_board[(board_y + dragged_piece.0) / 2][(board_x + dragged_piece.1) / 2] = None
                                }
                                println!("got here, y should become {}, x {}", board_y, board_x);
                            },
                            Err(e) => eprintln!("error: {}", e),
                        };
                    }
            }
            current_dragged = None;
            for j in 0..8 {
                for i in 0..8 {
                    if let Some(mut piece) = &mut piece_board[j][i] {
                        piece.is_dragging = false;
                    }
                }
            }
        }
        if let Some(required) = checkers.required_square {
            if let Some(dragging) = current_dragged {
                if dragging != required {
                    piece_board[dragging.0][dragging.1].unwrap().is_dragging = false;
                    current_dragged = None;
                }
            }
        }
        for j in 0..8 {
            for i in 0..8 {
                if let Some(piece) = &mut piece_board[j][i] {
                    if piece.is_dragging {
                        piece.ui_position.0 = mouse_position().1;
                        piece.ui_position.1 = mouse_position().0;
                        println!("am dragging");
                    } else {
                        piece.ui_position.1 =  offset_x + (i as f32 + 0.5) * sq_size;
                        piece.ui_position.0 =  offset_y + (j as f32 + 0.5) * sq_size;
                    }
                    let color: Color;
                    if piece.piecekind < 0 {
                        color = RED;
                    } else {
                        color = BLACK;
                    }
                    draw_circle(piece.ui_position.1, piece.ui_position.0, circle_radius, color);
                    
                }
            }
        }

        next_frame().await;
    }

    fn paint_board () {
        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let sq_size = (screen_height() - offset_y * 2.) / 8 as f32;

        draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

        for j in 0..8 {
            for i in 0..8 {
                if (i + j) % 2 == 0 {
                    draw_rectangle(
                        offset_x + i as f32 * sq_size,
                        offset_y + j as f32 * sq_size,
                        sq_size,
                        sq_size,
                        BEIGE,
                    );
                } else {
                    draw_rectangle(
                        offset_x + i as f32 * sq_size,
                        offset_y + j as f32 * sq_size,
                        sq_size,
                        sq_size,
                        BROWN,
                    );
                }
            }
        }

        for i in 1..8 {
            draw_line(
                offset_x,
                offset_y + sq_size * i as f32,
                screen_width() - offset_x,
                offset_y + sq_size * i as f32,
                2.,
                BLACK,
            );
        }

        for i in 1..8 {
            draw_line(
                offset_x + sq_size * i as f32,
                offset_y,
                offset_x + sq_size * i as f32,
                screen_height() - offset_y,
                2.,
                BLACK,
            );
        }
    }
}