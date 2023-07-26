pub mod board;
pub mod moves;
pub mod spell;
pub mod menu;

use std::{collections::HashMap, sync::Arc};

pub use moves::make_move;
pub use spell::*;
pub use menu::*;

#[derive(Clone)]
pub struct Game {
    board: board::UIBoard,
    moves: Vec<chess::ChessMove>,
    spells: Vec<Arc<dyn Spell>>,
    casted_mine: HashMap<(usize, usize), Arc<dyn Spell>>,
    casted_other: HashMap<(usize, usize), Arc<dyn Spell>>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: board::UIBoard::default(),
            moves: Vec::new(),
            spells: vec![
                Arc::new(JihadiWarrior::default())],
            casted_mine: HashMap::new(),
            casted_other: HashMap::new(),
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn board(&self) -> &board::UIBoard {
        &self.board
    }

    pub fn raw_board(&self) -> &chess::Board {
        self.board.game_state()
    }

    pub fn make_move(&mut self, m: chess::ChessMove) {
        self.board = self.board.make_move_new(m);
        self.moves.push(m);
    }

    pub fn make_move_new(&self, m: chess::ChessMove) -> Self {
        let mut new_game = self.clone();
        new_game.make_move(m);
        new_game
    }

    pub fn spells(&self) -> &Vec<Arc<dyn Spell>> {
        &self.spells
    }

    pub fn new_board(&self) -> board::UIBoard {
        self.board.clone()
    }

    pub fn set_board(self, board: board::UIBoard) -> Self {
        Self {
            board,
            moves: self.moves,
            spells: self.spells,
            casted_mine: self.casted_mine,
            casted_other: self.casted_other,
        }
    }

    pub fn cast_spell(&mut self, spell: Arc<dyn Spell>, square: chess::Square) {
        let mut spell = spell.clone();
        self.remove_spell(spell.identifier());
        self.casted_mine.insert((square.get_rank().to_index(), square.get_file().to_index()), spell);
    }

    pub fn remove_spell(&self, id: u32) -> Self {
        let mut spells = self.spells().clone();
        let rm_index = spells.iter().position(|s| s.identifier() == id).unwrap();
        spells.remove(rm_index);
        Self {
            board: self.board.clone(),
            moves: self.moves.clone(),
            spells,
            casted_mine: self.casted_mine.clone(),
            casted_other: self.casted_other.clone(),
        }
    }
}