use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use rand::{RngExt};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="dark centered">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/dev-site.css"/>

        <Title text="Jossie's Site"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <NavBar/>
        <Footer/>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    let links = vec!["/about", "/home", "/ctfs"];
    view! {
        <nav id="topnav">            
            <ui>
                <li><a class="left" href="/home"> "irene" </a></li>


                <li><a href="/about">about</a></li>
            </ui>
        </nav>
    }
}

#[component]
fn Footer() -> impl IntoView {
    let quotes = vec!["Accept everything just the way it is", "ts so chopped", "I'm employed bro 🫰", "i aint got none", "Thankfully I'm immortal as I've never died before"];
    let mut rng = rand::rng();
    
    let index: usize  = { // TODO! Fix Fix this horrible implementation
        let num: u8 = rng.random();
        let i: u8 = num % quotes.len() as u8;
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
                    { format!("© 2026 Jossey28 | {} ™", quote) }
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