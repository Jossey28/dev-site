use leptos::{IntoView, component, mount::mount_to_body};
use leptos::prelude::{view, ElementChild, ClassAttribute};
use rand::{Rng, RngExt};

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
    let quotes = vec!["Accept everything just the way it is", "ts so chopped", "I'm employed bro 🫰", "i aint got none"];
    let mut rng = rand::rng();
    
    let index: usize  = { // TODO! Fix Fix this horrible implementation
        let num = rng.random::<u8>();
        let i = num % quotes.len() as u8;
        i as usize
    };

    let quote_opt = quotes.get(index);
    let quote = match quote_opt {
        Some(_) =>  { *quote_opt.unwrap() },
        _ => "Hello there",
    };

    view! {
        <footer>
            <div>
                <p>
                    { format!("© 2026 Jossey28 | {:#?}™", quote) }
                </p>
            </div>
        </footer>
    }
}

fn remove_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    match s.strip_prefix(prefix) {
        Some(s) => s,
        None => s
    }
}