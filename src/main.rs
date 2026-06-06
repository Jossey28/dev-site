#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use dev_site::app::{App, shell};
    use tower_livereload::LiveReloadLayer;

    let conf = get_configuration(None).unwrap();
    let addr = "0.0.0.0:300";
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone()) 
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(LiveReloadLayer::new())
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {
    panic!(r#"Non-SSR mode hasn't been made yet. Run "cargo run --features ssr" to execute"#)
}