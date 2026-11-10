use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::pages::{
    blog::{BlogIndex, BlogPost},
    home::Home,
    not_found::NotFound,
    projects::{ProjectDetail, ProjectsIndex},
    tools::Tools,
    who::Who,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="color-scheme" content="dark"/>
                <meta name="theme-color" content="#0F1311"/>
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>
                <link
                    rel="stylesheet"
                    href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400&family=Jost:wght@400;500&family=Source+Serif+4:wght@400;600&display=swap"
                />
                <AutoReload options=options.clone()/>
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/personal_site.css"/>
        <Router>
            <Routes fallback=NotFound>
                <Route path=path!("/") view=Home/>
                <Route path=path!("/projects") view=ProjectsIndex/>
                <Route path=path!("/projects/:slug") view=ProjectDetail/>
                <Route path=path!("/blog") view=BlogIndex/>
                <Route path=path!("/blog/:slug") view=BlogPost/>
                <Route path=path!("/tools") view=Tools/>
                <Route path=path!("/who") view=Who/>
            </Routes>
        </Router>
    }
}
