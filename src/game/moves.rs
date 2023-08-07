use chess::{Board, Square, ChessMove};
use web_sys::{Document, HtmlElement};
use web_sys::console::info;
use yew::{DragEvent, MouseEvent, TargetCast, HtmlResult, UseStateHandle};
use std::sync::Arc;
use std::{result::Result, ops::Div};
use log::info;
use crate::document;

use super::board::render::UIBoard;
use super::{Game, Spell, spell};

/// Error Type for Illagl Move
pub struct IllegalMoveError;

impl std::fmt::Display for IllegalMoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Illegal Move")
    }
}

pub fn move_piece(game: &Game, target_square: Square, source_id: String) -> Result<Game, IllegalMoveError> {
    let r = source_id.chars().nth(0).unwrap().to_digit(10).unwrap();
    let f = source_id.chars().nth(1).unwrap().to_digit(10).unwrap();
    info!("from -> file: {}, rank: {}", f, r);
    let source_square = Square::make_square(chess::Rank::from_index(f as usize), chess::File::from_index(r as usize));
    let m = ChessMove::new(source_square, target_square, None);
    info!("move: {:?}", m);
    let mut legal = chess::MoveGen::new_legal(game.raw_board());

    match legal.find(|l| l == &m) {
        Some(m) => Ok(game.make_move_new(m)),
        None => Err(IllegalMoveError),
    }
}

pub fn collect_spell(game: &Game, target_square: Square) -> Game {
    let piece = game.board().piece_on(target_square);
    let color = game.board().color_on(target_square);
    let spell = game.board().spell_on(target_square);
    match spell {
        Some(s) => game.collect_spell(target_square, s),
        None => game.clone()
    }
}

pub fn make_move(game: &Game, event: DragEvent) -> Game {
    info!("make_move called");
    let point = (event.client_x() as f64, event.client_y() as f64);
    let e: Option<web_sys::HtmlElement> = event.target_dyn_into();
    let board_el = document().get_element_by_id(&"board").unwrap();

    let id = match e {
        Some(e) => e.id(),
        None => return game.clone(),
    };

    match id.len() {
        0 => return {
            info!{"no piece target"};
            game.clone()
        },
        _ => {
            info!{"target id: {}", id};
            let target_square = map_to_square(point, &board_el, game.board().dims());
            let game = match move_piece(game, target_square, id) {
                Ok(game) => game,
                Err(e) => {
                    return game.clone();
                }
            };
            let game = collect_spell(&game, target_square);
            return game.clone();
        }
    };
    //board.make_move(from, to);
}


pub fn cast_spell(game: &Game, spell: Arc<dyn Spell>, event: DragEvent) -> Game {
    info!("cast_spell called");
    let point = (event.client_x() as f64, event.client_y() as f64);
    let e: Option<web_sys::HtmlElement> = event.target_dyn_into();
    let board_el = document().get_element_by_id(&"board").unwrap();

    let square = map_to_square(point, &board_el, game.board().dims());
    //let valid = game.board().piece_on(square).unwrap_or(false).is_mine();
    info!("casted {:?} to square: {:?}", spell.name(), square);

    let game = spell.execute(game.clone(), Some(square));

    let id = match e {
        Some(e) => e.id(),
        None => return game.clone(),
    };

    match id.len() {
        0 => return {
            info!{"no piece target"};
            game.clone()
        },
        _ => {
            info!{"target id: {}", id};
        }
    };

    game.clone()
}

fn map_to_square(point: (f64, f64), board_el: &web_sys::Element, dims: (usize, usize)) -> Square {
    let board_bounds = board_el.get_bounding_client_rect();
    let board_pos = (board_bounds.x(), board_bounds.y());
    let board_size = (board_el.client_width() as f64, board_el.client_height() as f64);
    map_to_square_(point, board_size, board_pos, dims)
}

fn map_to_square_(point: (f64, f64), board_size: (f64, f64), board_pos: (f64, f64), board_dims: (usize, usize)) -> Square {
    let square_dims = (board_size.0 / board_dims.0 as f64, board_size.1 / board_dims.1 as f64);
    info!("square_dims: {:?}", square_dims);
    info!("board_pos: {:?}", board_pos);
    info!("point: {:?}", point);

    let normalized_point = (point.0 - board_pos.0, point.1 - board_pos.1);

    info!("normalized_point: {:?}", normalized_point);
    info!("rel_pos: {:?}", ((normalized_point.0 / board_size.0), (normalized_point.1 / board_size.1)));

    let inv_file = (((normalized_point.0 / board_size.0) * board_dims.0 as f64).floor() as u8);
    let inv_rank = (((normalized_point.1 / board_size.1) * board_dims.1 as f64).floor() as u8);

    info!("inv_file: {}, inv_rank: {}", inv_file, inv_rank);

    let file = (7 as u8) - inv_file;
    let rank = (7 as u8) - inv_rank;

    info!("to -> file: {}, rank: {}", file, rank);
    Square::make_square(chess::Rank::from_index(rank as usize), chess::File::from_index(file as usize))
}
// ideas: invisibility spell, teleportation spell, lightning spell, jihadi warrior spell,
//        train spell, floor is lava spell, add row spell, add column spell, poison spell, steel spell
// add powerups to specific squares that maybe are suboptimal to get to
