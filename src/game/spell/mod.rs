pub mod render;
pub use render::*;
mod warrior;
pub use warrior::*;
use super::Game;

pub trait Spell {
    fn class_list(&self) -> String;
    fn execute(&self, game: Game, square: Option<chess::Square>) -> Game;
    fn update(&mut self, m: &chess::ChessMove);
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn identifier(&self) -> u32;
}

#[derive(Clone)]
pub enum Kind<T: Spell + ?Sized> {
    Transparent(Box<T>),
    Opaque(Box<T>),
    None,
}

impl<T: Spell + PartialEq> PartialEq for Kind<T> {
    fn eq(&self, other: &Self) -> bool {
        // TODO: incomplete
        match (self, other) {
            (Kind::Transparent(_), Kind::Transparent(_)) => true,
            (Kind::Opaque(_), Kind::Opaque(_)) => true,
            _ => false,
        }
    }
}