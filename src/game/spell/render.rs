use yew::prelude::*;
use super::Kind;
use crate::game::spell::Spell;

pub fn render<T: Spell + ?Sized>(spell: &Kind<T>) -> Html {
    match spell {
        Kind::Opaque(_) => {
            html! {
                <div class="spell spell--opaque">
                </div>
            }
        },
        Kind::Transparent(spell) => {
            html! {
                <div class={spell.class_list()}>
                </div>
            }
        },

        Kind::None => {
            html! {}
        },

    }
}