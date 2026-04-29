use leptos::{IntoView, component, mount::mount_to_body};
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    set_count.set(0);

    view! {
        <button
            on:click=move |_| { *set_count.write() += 1 }
            >
            "Click me: "
            {count}
            </button>
        <p>
            "Double count: "
            {move || count.get() * 2}
        </p>
    }
}