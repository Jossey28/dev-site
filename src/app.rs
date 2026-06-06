use std::{process::{Command, Output}, time::{SystemTime, UNIX_EPOCH}};

use leptos::prelude::*;
use leptos_use::use_favicon;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
		<!DOCTYPE html>
		<html>
			<head>
				<meta charset="utf-8" />
				<meta name="viewport" content="width=device-width, initial-scale=1.0" />
				<meta name="dcterms.rightsHolder" content="Jossey Corp." />
				<meta name="dcterms.rights" content="Copyright 2026, All Rights Reserved." />

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
			<table id="88x31 Table" class="marquee-container">
				<tbody>
					<tr>
						<td class="88x31s marquee-content">
							<Eightyeight
								link="https://github.com/Jossey28/dev-site"
								image="/assets/88x31s/opensource-88x31.gif"
							/>
							<Eightyeight
								link="https://lumiverse.dev/"
								image="/assets/88x31s/lumi-88x31.gif"
							/>
							<Eightyeight
								link="https://rust-lang.org"
								image="/assets/88x31s/rust-88x31.gif"
							/>
						</td>
					</tr>
					<tr>
						<td class="88x31s marquee-content">
							<Eightyeight
								link="https://github.com/Jossey28/dev-site"
								image="/assets/88x31s/opensource-88x31.gif"
							/>
							<Eightyeight
								link="https://lumiverse.dev/"
								image="/assets/88x31s/lumi-88x31.gif"
							/>
							<Eightyeight
								link="https://rust-lang.org"
								image="/assets/88x31s/rust-88x31.gif"
							/>
						</td>
					</tr>
				</tbody>
			</table>
		</section>
	}
}

#[component]
fn Eightyeight(
	image: &'static str,
	link: &'static str
) -> impl IntoView {
	view! {
		<a href=link target="_blank">
			<img src=image width=88 height=31 />
		</a>
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
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
	let quote = quotes[(now % quotes.len() as u64) as usize]; // T-T

	let commit = 'get_commit: { 
		let prim = option_env!("GIT_COMMIT_SHA_RUST");
		let secnd = option_env!("GIT_COMMIT_SHA_DOCKER"); 

		if let Some(sha) = prim {
			if sha.len() > 3 {
				break 'get_commit sha
			}
		} 
		
		if let Some(sha) = secnd {
			if sha.len() > 3 {
				break 'get_commit sha
			}
		} 
		
		"unknown"
	};

    view! {
		<footer>
			<small>
				<span class="copyright">"\u{00A9} "</span>
				<span>
					<a href=format!(
						"https://github.com/Jossey28/dev-site/commit/{}",
						commit,
					)>{format!(" 2026 Jossey28 @ {} ", commit.chars().take(7).collect::<String>())}</a>
				</span>
				<span>{format!(" | {} ™ — Aristotle", quote)}</span>
			</small>
		</footer>
	}
}