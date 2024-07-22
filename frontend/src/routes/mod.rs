use yew::{html, Html};
use yew_router::Routable;

use crate::views::{
    home::Home,
    not_found::NotFound
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Professional,
    #[not_found]
    #[at("/404")]
    NotFound
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Professional => html!{ <Home /> },
        Route::NotFound => html! { <NotFound /> }
    }
}