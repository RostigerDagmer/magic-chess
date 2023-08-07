use std::sync::Arc;

use super::{Spell, JihadiWarrior};

#[derive(Clone)]
pub struct Inventory {
    spells: Vec<Arc<dyn Spell>>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self { spells: Vec::new() }
    }
}

impl Inventory {
    pub fn test() -> Self {
        Self {
            spells: vec![Arc::new(JihadiWarrior::default())],
        }
    }
    pub fn collect_spell(self, spell: Arc<dyn Spell>) -> Self {
        let mut spells = self.spells;
        spells.push(spell);
        Self {
            spells
        }
    }
    pub fn remove_spell(self, id: u32) -> Self {
        let mut spells = self.spells;
        let spells = spells.iter().filter_map(|s| {
            match s.identifier() {
                id => {
                    Some(s.clone())
                }
                _ => None
            }
        }).collect::<Vec<_>>();
        Self {
            spells
        }
    }

    pub fn spells(&self) -> Vec<Arc<dyn Spell>> {
        self.spells.to_vec()
    } 
}