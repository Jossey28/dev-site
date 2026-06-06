use leptos::prelude::*;
use leptos_use::use_favicon;
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
				<meta charset="utf-8" />
				<meta name="viewport" content="width=device-width, initial-scale=1" />
				<AutoReload options=options.clone() />
				<HydrationScripts options />
				<MetaTags />
			</head>
			<body class="dark centered">
				<App />
			</body>
		</html>
	}
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (_, set_icon) = use_favicon();
    set_icon.set(Some("favicon.ico".to_string()));

    view! {
		<Stylesheet id="leptos" href="/pkg/dev-site.css" />

		<Title text="Jossie's Site" />
		// <link rel="icon" type="image/x-icon" href="/favicon.ico" />

		<Router>
			<main>
				<Routes fallback=|| PageNotFound>
					<Route path=StaticSegment("") view=HomePage />
					<Route path=StaticSegment("/home") view=HomePage />
				</Routes>
			</main>
		</Router>
	}
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
		<NavBar />
		<AboutMe />
		<Footer />
	}
}

#[component]
fn AboutMe() -> impl IntoView {
    view! {
		<section id="about-me">
			<h2 style="text-decoration: underline;">"About Me"</h2>
            <table id="88x31" class="marquee-content">
                <tbody>
					<tr>
						<td>
							<a href="https://lumiverse.dev/" target="_blank">
								<img src="/assets/88x31s/lumi-88x31.gif" width=88 height=31 />
							</a>

							<a href="https://lumiverse.dev/" target="_blank">
								<img src="/assets/88x31s/lumi-88x31.gif" width=88 height=31 />
							</a>
						</td>
					</tr>
                </tbody>
            </table>
        </section>
	}
}

#[component]
fn PageNotFound() -> impl IntoView {
    view! {
		<NavBar />

		<h1>"Page not Found"</h1>
	}
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
		<nav id="topnav">
			<ui>
				<li class="left">
					<a href="/home">"irene"</a>
				</li>
				<li>
					<a href="/blog">blog</a>
				</li>
				<li>
					<a href="/comps">accomplishments</a>
				</li>
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
				<p>{format!("© 2026 Jossey28 | {} ™ — Aristotle", quote)}</p>
			</div>
		</footer>
	}
}