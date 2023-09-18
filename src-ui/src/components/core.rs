use leptos::{component, view, IntoView};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div id="header">
            <h2>"One Time Chat"</h2>
        </div>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Header/>
        <div>
            <h1>"Hello World!"</h1>
        </div>
    }
}
