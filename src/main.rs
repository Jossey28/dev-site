#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use dev_site::app::{App, shell};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};

    #[allow(clippy::expect_used)]
    let conf = get_configuration("./Cargo.toml".into()).expect("Failed to pull configuration");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    #[cfg(debug_assertions)]
    {
        use tower_livereload::LiveReloadLayer;
        let app = app.clone().layer(LiveReloadLayer::new());
        log!("LIVE RELOAD ON ; listening on http://{}", &addr);
        #[allow(clippy::expect_used)]
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .expect("Unable to bind to port");
        #[allow(clippy::expect_used)]
        // Since theres no point in continuing if either of these fail
        axum::serve(listener, app.into_make_service())
            .await
            .expect("Failed to serve app");
    }

    #[cfg(not(debug_assertions))]
    {
        log!("listening on http://{}", &addr);
        #[allow(clippy::expect_used)]
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .expect("Unable to bind to port");
        #[allow(clippy::expect_used)]
        axum::serve(listener, app.into_make_service())
            .await
            .expect("Failed to serve app");
    }
}

#[cfg(not(feature = "ssr"))]
fn main() {
    panic!(r#"Non-SSR mode hasn't been made yet. Run "cargo run --features ssr" to execute"#)
}
