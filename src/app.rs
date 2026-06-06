use std::time::{SystemTime, UNIX_EPOCH};

use leptos::prelude::*;
use leptos_use::use_favicon;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use rand::{rng, seq::SliceRandom};

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
				<Routes fallback=move || {
					view! { <HomePage /> }
				}>
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
		// <NavBar />
		<AboutMe />
		<Footer />
	}
}

#[component]
fn AboutMe() -> impl IntoView {
	let mat_88 = EightyEightData::new("mat_does_dev-88x31.gif", "https://matdoes.dev/", Some("Great collecter of 88x31s"));

    view! {
		<section id="about-me">
			<h2>
				"About Me via " <span style="text-decoration: underline;">
					<a href="https://en.wikipedia.org/wiki/Web_badge">"88x31s"</a>
				</span>
			// "couresty of" <span id="mat-apprecation">
			// <EightyEight info=mat_88 />
			// </span>
			</h2>

			<div id="table-of-88x31s" class="marquee-boss">
				<Create88x31Row />
				<Create88x31Row />
				<Create88x31Row />
			</div>
		</section>
	}
}

#[component]
fn EightyEight(
	info: EightyEightData
) -> impl IntoView {
	view! {
		<a href=info.link target="_blank">
			<img
				title=info.title.unwrap_or_else(|| "")
				src=format!("/assets/88x31s/{}", info.image)
				alt=format!("eighty eight by thirty one linking to {}", info.link)
				width=88
				height=31
			/>
		</a>
	}
}

#[component]
fn Create88x31Row() -> impl IntoView {
	let mut eights: Vec<EightyEightData> = vec![ 
		EightyEightData::new("opensource-88x31.gif", "https://github.com/Jossey28/dev-site", None),
		EightyEightData::new("powered_by_nixos-88x31.gif", "https://nixos.org/", None),
		EightyEightData::new("made_on_linux-88x31.gif", "https://stallman-copypasta.github.io/", None),
		EightyEightData::new("rtr-88x31.gif", "https://www.youtube.com/@rossmanngroup/", None),
		EightyEightData::new("rust-88x31.gif", "https://rust-lang.org/", None),
		EightyEightData::new("lumi-88x31.gif", "https://lumiverse.dev/", Some("Lumi!! qwq")),
		EightyEightData::new("arch_btw-88x31.gif", "https://archlinux.org/", Some("I use arch btw")),
		EightyEightData::new("xbox_live-88x31.gif", "https://xenia-emulator.com/xbox-360-roms/", None),
		EightyEightData::new("xbox_live-88x31.gif", "https://xenia-emulator.com/xbox-360-roms/", None),
		EightyEightData::new("hosted_on_a_pi-88x31.gif", "https://raspberrypi.com", None),
		EightyEightData::new("stop_sign-88x31.gif", "https://matdoes.dev/buttons#d0f3c00fbd84b58dd3ab3cc353fc3ffd", None),
		EightyEightData::new("emulate_now-88x31.gif", "https://matdoes.dev/buttons#d0f3c00fbd84b58dd3ab3cc353fc3ffd", None),
		EightyEightData::new("sleepy-88x31.gif", "https://www.nhlbi.nih.gov/health/sleep/why-sleep-important", None),
		EightyEightData::new("hacker_powered-88x31.gif", "https://iateched.org/", None),
		EightyEightData::new("cookie_free-88x31.gif", "https://improvado.io/blog/what-is-tracking-pixel", Some("It gets to a point")),
		EightyEightData::new("archive-88x31.gif", "https://archive.org/donate", None),
		EightyEightData::new("copy_that_floppy-88x31.gif", "https://archive.org/donate", None),
		EightyEightData::new("neocities-88x31.gif", "https://neocities.org/", None),
		EightyEightData::new("midi_files-88x31.gif", "https://www.nccih.nih.gov/health/music-and-health-what-you-need-to-know", None),
		EightyEightData::new("dr_pepper-88x31.gif", "https://drpepper.com/en", None),
		EightyEightData::new("im_no_diva-88x31.gif", "https://dev-site.homecamp.biz", None),
		EightyEightData::new("steam_powered-88x31.gif", "https://web.archive.org/web/20041115052057/http://steampowered.com/", None),
		EightyEightData::new("kitrinos-88x31.gif", "https://kitrinos.neocities.org/", None),
		EightyEightData::new("cool_graphics-88x31.gif", "https://codepen.io/trending", None),
		EightyEightData::new("hellnet-88x31.gif", "https://hellnet.work/", None),
		EightyEightData::new("wikipedia-88x31.gif", "https://en.wikipedia.org/wiki/Main_Page", None),
	];

	let mut r = rng();
	eights.shuffle(&mut r);

	view! {
		<div class="marquee-row-container">
			<span class="class-88x31s marquee-content">
				// https://docs.rs/leptos/latest/leptos/attr.component.html
				{eights
					.iter()
					.map(|child| view! { <EightyEight info=*child /> })
					.collect::<Vec<_>>()}
			</span>

			<span class="class-88x31s marquee-content">
				{eights
					.iter()
					.map(|child| view! { <EightyEight info=*child /> })
					.collect::<Vec<_>>()}
			</span>
		</div>
	}
}

#[derive(Debug, Clone, Copy)]
struct EightyEightData {
	image: &'static str,
	link: &'static str,

	title: Option<&'static str>
}

impl EightyEightData {
	fn new(image: &'static str, link: &'static str, title: Option<&'static str>) -> Self {
		Self { image, link, title }
	}
}

#[component]
fn PageNotFound() -> impl IntoView {
    view! {
		<NavBar />

		<h1>"Page not Found"</h1>

		<Footer />
	}
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
		<nav id="topnav">
			<ui>
				<li class="left">
					<a href="/home">"home"</a>
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

	let next = EightyEightData::new("continue_the_ring-88x31.gif", "https://schlumbis.dev/", None);

    view! {
		<footer>
			<small>
				<span class="copyright">"\u{00A9} "</span>
				<span>
					<a href=format!(
						"https://github.com/Jossey28/dev-site/commit/{}",
						commit,
					)>
						{format!(
							" 2026 Jossey28 @ {} ",
							commit.chars().take(7).collect::<String>(),
						)}
					</a>
				</span>
				<span>{format!(" | {} ™ — Aristotle", quote)}</span>
			</small>

			<span id="ring-next">
				<EightyEight info=next />
			</span>
		</footer>
	}
}