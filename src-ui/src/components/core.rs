use leptos::{component, view, IntoView};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div id="header" class="otc-header">
            <i class="fa fa-user" aria-hidden="true"></i>
            <h1 class="ml-4" >"One Time Chat"</h1>
            <i class="fa fa-pencil" aria-hidden="true"></i>
        </div>
    }
}
