pub mod components;
pub mod routes;

use crate::routes::home::Home;
use leptos::{component, view, IntoView};
use leptos_router::{Route, Router, Routes};
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
        <main>
            <div id="route-view">
            <Routes>
            <Route path="/" view=Home/>
            </Routes>
            </div>
            </main>
        </Router>
    }
}
