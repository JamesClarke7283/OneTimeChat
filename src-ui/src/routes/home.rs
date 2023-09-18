use crate::components::core::Header;
use leptos::{component, view, IntoView};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Header/>
        <div>
            <h1>"Hello World!"</h1>
        </div>
    }
}
