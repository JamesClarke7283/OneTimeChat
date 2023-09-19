pub mod components;
pub mod routes;
pub mod utilities;
use crate::routes::home::Home;
use crate::utilities::set_displayed_theme;
use leptos::{component, create_action, view, IntoView};
use leptos_router::{Route, Router, Routes};
use log::warn;
use tauri_sys::tauri;

#[component]
pub fn App() -> impl IntoView {
    // Create action to query theme
    let init_theme = create_action(move |_| async move {
        match tauri::invoke::<(), String>("query_settings_theme", &()).await {
            Ok(retrieved_theme) => {
                set_displayed_theme(&retrieved_theme); // Replace with your actual function to set theme
            }
            Err(e) => {
                warn!("Failed to call query_theme: {}", e);
            }
        }
    });

    // Dispatch the action to query and set theme at startup
    init_theme.dispatch(&());
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
