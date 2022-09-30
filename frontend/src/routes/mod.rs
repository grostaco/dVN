use yew::{html, Html};
use yew_router::Routable;

pub mod home;
pub mod play;

use home::Home;
use play::Play;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/play")]
    Play,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Play => html! { <Play /> },
    }
}
