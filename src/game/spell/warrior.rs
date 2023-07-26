use std::sync::Arc;

use lazy_static::__Deref;
use log::info;

use super::Spell;
use crate::game::{board::UIBoard, Game};

#[derive(Clone, PartialEq)]
pub struct JihadiWarrior {
    id: u32,
    name: &'static str,
    description: &'static str,
    level: u8,
    on: Option<chess::Square>,
    valid_for: i32,
    valid: bool
}

impl Default for JihadiWarrior {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Jihadi Warrior",
            description: "Remove all pieces in a 3x3 square around the Jihadi Warrior",
            level: 1,
            on: None, //chess::Square::make_square(chess::Rank::Fourth, chess::File::E),
            valid_for: 3,
            valid: true
        }
    }
}

impl Spell for JihadiWarrior {
    fn class_list(&self) -> String {
        "spell spell--jihadi-warrior".to_owned()
    }

    fn execute(&self, game: Game, square: Option<chess::Square>) -> Game {
        // remove all pieces in a 3x3 square around the jihadi warrior
        let mut board = game.board().clone();
        let mut clear_squares = Vec::new();
        let mut on = self.on;
        match square {
            Some(square) => on = Some(square),
            None => (),
        };
        match on {
            Some(square) => {
                for file in square.get_file().to_index() - 1..=square.get_file().to_index() + 1 {
                    for rank in square.get_rank().to_index() - 1..=square.get_rank().to_index() + 1 {
                        if file > 7 || rank > 7 {
                            continue;
                        }
                        let square = chess::Square::make_square(chess::Rank::from_index(rank), chess::File::from_index(file));
                        clear_squares.push(square);
                    }
                }
                for square in clear_squares {
                    board = board.remove_piece(square);
                }
                let game = game.set_board(board);
                let game = game.remove_spell(self.id);
                return game;
            },
            None => {
                info!("Jihadi Warrior not on board (yet! 👳🏾‍♂️)");
                return game;
            },
        }
    }
    fn update(&mut self, _m: &chess::ChessMove) {
        // TODO check if the jihadi warrior is captured
        self.valid = self.valid_for > 0;
        self.valid_for = std::cmp::max(0, self.valid_for - 1);
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn description(&self) -> &'static str {
        self.description
    }

    fn identifier(&self) -> u32 {
        self.id
    }
}