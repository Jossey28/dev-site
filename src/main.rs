use leptos::{IntoView, component, mount::mount_to_body};
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {

    view! {
        <NavBar/>

        <Footer/>
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

#[component]
fn NavBar() -> impl IntoView {
    let links = vec!["/about", "/home", "/ctfs"];
    view! {
        <div class="topnav">            
            {
                links.into_iter()
                    .map(|link| view! { <a href={link}> {remove_prefix({link}, "/")} </a> })
                    .collect::<Vec<_>>()
            }

            <a class="left" href="/home"> "irene" </a>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <p>"Bye Bye"</p>
    }
}

fn remove_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    match s.strip_prefix(prefix) {
        Some(s) => s,
        None => s
    }
}