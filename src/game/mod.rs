pub mod board;
pub mod moves;
pub mod spell;
pub mod menu;
pub mod inventory;

use std::{collections::HashMap, sync::Arc};

pub use moves::make_move;
pub use spell::*;
pub use menu::*;
pub use inventory::*;

#[derive(Clone)]
pub struct Game {
    board: board::UIBoard,
    moves: Vec<chess::ChessMove>,
    inventory: Inventory,
    casted_mine: HashMap<(usize, usize), Arc<dyn Spell>>,
    casted_other: HashMap<(usize, usize), Arc<dyn Spell>>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: board::UIBoard::default(),
            moves: Vec::new(),
            inventory: Inventory::default(),
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
        self.board.board()
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

    pub fn spells(&self) -> Vec<Arc<dyn Spell>> {
        self.inventory.spells()
    }

    pub fn new_board(&self) -> board::UIBoard {
        self.board.clone()
    }

    pub fn set_board(self, board: board::UIBoard) -> Self {
        Self {
            board,
            moves: self.moves,
            inventory: self.inventory,
            casted_mine: self.casted_mine,
            casted_other: self.casted_other,
        }
    }

    pub fn cast_spell(&self, spell: Arc<dyn Spell>, square: chess::Square) -> Self {
        let spell = spell.clone();
        let i_ = self.inventory.to_owned().remove_spell(spell.identifier());//self.remove_spell(spell.identifier());
        let mut casted_ = self.casted_mine.to_owned();
        casted_.insert((square.get_rank().to_index(), square.get_file().to_index()), spell);
        Self {
            board: self.board.clone(),
            moves: self.moves.clone(),
            inventory: i_.clone(),
            casted_mine: casted_,
            casted_other: self.casted_other.clone(),
        }
        
    }

    pub fn collect_spell(&self, square: chess::Square, spell: Arc<dyn Spell>) -> Game {
        let i_ = self.inventory.clone().collect_spell(spell);
        Self {
            board: self.board.clone(),
            moves: self.moves.clone(),
            inventory: i_,
            casted_mine: self.casted_mine.clone(),
            casted_other: self.casted_other.clone(),
        }
    }

    pub fn remove_spell(self, id: u32) -> Self {
        let i_ = self.inventory.remove_spell(id);
        // let mut spells = self.spells().clone();
        // let rm_index = spells.iter().position(|s| s.identifier() == id).unwrap();
        // spells.remove(rm_index);
        Self {
            board: self.board.clone(),
            moves: self.moves.clone(),
            inventory: i_,
            casted_mine: self.casted_mine.clone(),
            casted_other: self.casted_other.clone(),
        }
    }
}