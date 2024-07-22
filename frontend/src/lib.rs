use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Switch};

use crate::routes::{Route, switch};

mod routes;
mod views;
mod models;

#[function_component(App)]
pub fn app() -> Html {
    html!{
        <BrowserRouter>
            <Switch<Route> render={ switch } />
        </BrowserRouter>
    }
}