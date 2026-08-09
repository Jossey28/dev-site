use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};
use leptos_use::use_favicon;
use rand::{
    rng,
    seq::{IndexedRandom, SliceRandom},
};
use serde::Deserialize;

#[must_use]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <meta name="dcterms.rightsHolder" content="Jossey Corp." />
                <meta name="dcterms.rights" content="Copyright 2026, All Rights Reserved." />

                // Fonts
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                <link
                    href="https://fonts.googleapis.com/css2?family=Exo+2:ital,wght@0,100..900;1,100..900&display=swap"
                    rel="stylesheet"
                />

                <AutoReload options=options.clone() />

                <HydrationScripts options />
                <MetaTags />

                <script type="text/javascript" src="/assets/js/gh-profile-card.min.js"></script>
                <script
                    type="text/javascript"
                    src="https://cdn.jsdelivr.net/gh/lumilovesyou/Gleebus-Webring@main/webring.js"
                ></script>

            </head>
            <body class="dark centered">
                <App />
            </body>
        </html>
    }
}

#[must_use]
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
        <Projects />
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
fn AboutMe() -> impl IntoView {
    // let mat_88 = EightyEightData::new(
    //     "mat_does_dev-88x31.gif",
    //     Some("https://matdoes.dev/"),
    //     Some("Great collecter of 88x31s"),
    // );

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
fn Create88x31Row() -> impl IntoView {
    let eighty_eight_data = include_str!("../data/buttons.json");
    #[allow(clippy::expect_used)]
    let mut eights: Vec<EightyEightData> =
        serde_json::from_str(eighty_eight_data).expect("Failed to properly parse buttons.json");

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

#[component]
fn EightyEight(info: EightyEightData) -> impl IntoView {
    view! {
        <a href=info.link target="_blank">
            <img
                title=info.title.unwrap_or("")
                src=format!("/assets/88x31s/{}", info.image)
                alt=format!(
                    "eighty eight by thirty one linking to {}",
                    info.link.unwrap_or("nowhere :)"),
                )
                width=88
                height=31
            />
        </a>
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct EightyEightData {
    image: &'static str,
    link: Option<&'static str>,
    title: Option<&'static str>,
}

impl EightyEightData {
    const fn new(
        image: &'static str,
        link: Option<&'static str>,
        title: Option<&'static str>,
    ) -> Self {
        Self { image, link, title }
    }
}

#[component]
pub fn Projects() -> impl IntoView {}

#[component]
fn Footer() -> impl IntoView {
    const QUOTES: [&str; 6] = [
        "Accept everything just the way it is",
        "ts so chopped",
        "I'm employed bro 🫰",
        "i aint got none",
        "Thankfully I'm immortal as I've never died before",
        "And when Jossey28 saw the breadth of his domain, he wept, for there were no more worlds left to conquer.",
    ];

    let mut rng = rand::rng();
    let quote = QUOTES
        .choose(&mut rng)
        .map_or("something smart", |quote| *quote);

    let commit = option_env!("GIT_COMMIT_SHA_RUST").map_or_else(
        || option_env!("GIT_COMMIT_SHA_DOCKER").map_or("unknown", |commit_docker| commit_docker),
        |commit_rust| commit_rust,
    );

    let next = EightyEightData::new("continue_the_ring-88x31.gif", None, None);

    let prev = EightyEightData::new("continue_the_ring-2-88x31.gif", None, None);

    view! {
        <footer>
            <span id="ring-prev" onclick="gleebusOpen(-1)">
                <EightyEight info=prev />
            </span>

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
                <span>{format!(" | {quote} ™ — Aristotle")}</span>
            </small>

            <span id="ring-next" onclick="gleebusOpen(1)">
                <EightyEight info=next />
            </span>
        </footer>
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
