#![recursion_limit = "256"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use personal_site::app::{App, shell};
    use tower_http::compression::CompressionLayer;

    // In production, configuration comes from LEPTOS_* env vars
    // (LEPTOS_SITE_ADDR, LEPTOS_SITE_ROOT, LEPTOS_OUTPUT_NAME, ...).
    let conf = get_configuration(None).expect("leptos configuration");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(CompressionLayer::new())
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind site address");
    leptos::logging::log!("listening on http://{addr}");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("server error");
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // The binary is only built with the `ssr` feature; the client is the
    // `hydrate` build of the library.
}
