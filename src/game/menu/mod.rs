use lazy_static::__Deref;
use log::info;
use yew::prelude::*;
use super::Game;

pub fn render(game: &UseStateHandle<Game>) -> Html {
    //let state = game.clone();
    html! {
        <div class="menu--wrapper p-6 shadow-2xl rounded-xl">
            {game.spells().iter().map(|spell| {
                let state = game.clone();
                let spell = spell.clone();
                let classes = spell.class_list();
                let name = spell.name();
                html! {
                    <div id={format!("spell-{:?}", spell.identifier())} draggable="true" class="spell--wrapper p-6 shadow-xl rounded-lg" ondragend={
                        Callback::from(move |e| {
                            let game = state.clone();
                            let game_ = super::moves::cast_spell(&game, spell.clone(), e);
                            state.set(game_)
                        })
                    }>
                        <div class="spell--icon">
                            <div class={classes}>
                        </div>
                        </div>
                        <span class="text-base">{name}</span>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}