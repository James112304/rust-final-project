mod checkers;

use checkers::Checkers;
use image::GenericImageView;
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    pub board_position: (usize, usize),
    pub ui_position: (f32, f32),
    pub piecekind: i32,
    pub is_dragging: bool,
}
impl Piece {
    pub fn new(jpos: usize, ipos: usize, kind: i32) -> Self {
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
    let mut checkers: Checkers = Checkers::new().expect("could not initialize checkers");
    let mut current_dragged: Option<(usize, usize)> = None;
    let mut piece_board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
    let black_piece_image = load_png("media/bp.png").expect("couldn't load");
    let black_king_image = load_png("media/bk.png").expect("couldn't load");
    let red_piece_image = load_png("media/rp.png").expect("couldn't load");
    let red_king_image = load_png("media/rk.png").expect("couldn't load");

    let black_piece_texture = image_to_texture(black_piece_image);
    let black_king_texture = image_to_texture(black_king_image);
    let red_piece_texture = image_to_texture(red_piece_image);
    let red_king_texture = image_to_texture(red_king_image);

    reset_piece_board(&mut piece_board);


    loop {

        clear_background(LIGHTGRAY);
        let black_turn = checkers.current_turn > 0;
        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let sq_size = (screen_height() - offset_y * 2.) / 8_f32;
        let circle_radius = 2. * sq_size / 5.;

        draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

        paint_board();

        if let Some((j, i)) = current_dragged {
            //println!("moving here");
            if let Some(team) = piece_board[j][i] {
                
                let possible_places_vec: Option<Vec<(usize, usize, bool)>> = match checkers.get_possible_moves(j, i, team.piecekind){
                    Ok(r) => r,
                    Err(_e) => panic!("incorrect logic somewhere")
                };
                match possible_places_vec {
                    Some(vec) => {
                        for (j, i, _b) in vec {
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
            
        let calculation_x: f32 = ((mouse_position().0 - offset_x) / sq_size).floor();
        let calculation_y: f32 = ((mouse_position().1 - offset_y) / sq_size).floor();
        if (0. ..8.).contains(&calculation_x) && (0. ..8.).contains(&calculation_y) {
            let board_x = calculation_x as usize;
            let board_y = calculation_y as usize;
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
                } else if let Some(mut piece) = &mut piece_board[board_y][board_x] {
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
            } else {
                if let Some(dragged_piece) = current_dragged {
                    if checkers.can_make_move(
                        dragged_piece.0,
                            dragged_piece.1, 
                            board_y, 
                            board_x
                        ) {
                            match checkers.make_move_then_ai(dragged_piece.0,
                                dragged_piece.1, 
                                board_y, 
                                board_x
                            ) {
                                Ok(_) => {
                                    piece_board[board_y][board_x] = piece_board[dragged_piece.0][dragged_piece.1];
                                    piece_board[dragged_piece.0][dragged_piece.1] = None;
                                    if std::cmp::max(board_x, dragged_piece.1) - std::cmp::min(board_x, dragged_piece.1) > 1 {
                                        piece_board[(board_y + dragged_piece.0) / 2][(board_x + dragged_piece.1) / 2] = None
                                    }
                                    println!("got here, y should become {}, x {}", board_y, board_x);
                                    for j in 0..8 {
                                        for i in 0..8 {
                                            if checkers.board_state[j][i] != 0{
                                                piece_board[j][i] = Some(Piece::new(j, i, checkers.board_state[j][i]));
                                            } else {
                                                piece_board[j][i] = None;
                                            }
                                        }
                                    }
                                    if checkers.is_game_over() {
                                        reset_piece_board(&mut piece_board);
                                        current_dragged = None;
                                        checkers = Checkers::new().expect("could not initialize new checkers");
                                    }
                                },
                                Err(_e) => {
                                    reset_piece_board(&mut piece_board);
                                    current_dragged = None;
                                    checkers = Checkers::new().expect("could not initialize new checkers");
                                },
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
        }
        if let Some(required) = checkers.required_square {
            if let Some(dragging) = current_dragged {
                if dragging != required {
                    piece_board[dragging.0][dragging.1].expect("current_dragged info must be wrong").is_dragging = false;
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
                    let piece_texture;
                    match piece.piecekind {
                        1_i32 => piece_texture = &black_piece_texture,
                        3_i32 => piece_texture = &black_king_texture,
                        -1_i32 => piece_texture = &red_piece_texture,
                        -3_i32 => piece_texture = &red_king_texture,
                        _ => panic!("how")
                    }
                    draw_piece(piece_texture, piece.ui_position.1 - circle_radius, piece.ui_position.0 - circle_radius, circle_radius * 2.)
                }
            }
        }

        next_frame().await;
    }

    fn paint_board () {
        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let sq_size = (screen_height() - offset_y * 2.) / 8_f32;

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

    fn draw_piece(piece_texture: &Texture2D, x: f32, y: f32, across: f32) {
        draw_texture_ex(
            piece_texture, 
            x, 
            y, 
            WHITE, 
            DrawTextureParams { 
                dest_size: Some(Vec2::new(across, across)),
                ..Default::default()
            }
        )
    }

    fn load_png(path: &str) -> Option<image::DynamicImage> {
        match image::open(path) {
            Ok(image) => Some(image),
            Err(e) => {
                eprintln!("Error loading image: {}", e);
                None
            }
        }
    }

    fn image_to_texture(image: image::DynamicImage) -> Texture2D {
        let (width, height) = image.dimensions();
        let rgba_image = image.to_rgba8();
        
        Texture2D::from_rgba8(width as u16, height as u16, &rgba_image)
    }

    fn reset_piece_board(piece_board: &mut [[Option<Piece>; 8]; 8]) {
        for j in 0..8 {
            for i in 0..8 {
                piece_board[j][i] = None;
            }
        }
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
    }

}