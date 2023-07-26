use yew::prelude::*;
use crate::game;



#[function_component(App)]
pub fn app() -> Html {
    let g = use_state(|| game::Game::new());

    let board = game::board::render(g.clone());
    let menu = game::menu::render(&g);
    html! {
        <div class="app">
            {board}
            {menu}
        </div>
    }
    // html! {
    //     <main>
    //         <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
    //         <h1>{ "Hello World!" }</h1>
    //         <button {onclick}>{ "Click me!" }</button>
    //         <p>{ "You have clicked " }{ *counter }{ " times." }</p>
    //         <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
    //     </main>
    // }
}
