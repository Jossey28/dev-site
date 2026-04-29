use leptos::tachys::html::property;
use leptos::{IntoView, component, mount::mount_to_body};
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    view! {
        <button on:click=move |_| { *set_count.write() += 1}>
        "Click me"
        </button>

        <ProgressBar progress=count/>
        <ProgressBar progress=Signal::derive(double_count)/>
    }

}

// Below is an example of a doc component. These can be placed above the function, and above declared function parameters.
/// This component shows progress toward a goal
#[component]
fn ProgressBar(
    #[prop(default = 100)]
    max: u16,
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}