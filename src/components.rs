//! Shared server-rendered components. None of these hydrate.

use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::content::{GITHUB, HANDLE};

const NAV: &[(&str, &str, &str)] = &[
    ("projects", "Projects", "/projects"),
    ("blog", "Blog", "/blog"),
    // Tools is hidden until the page is reworked; see pages/tools.rs.
    ("who", "Who?", "/who"),
];

#[component]
pub fn SiteHeader(current: &'static str) -> impl IntoView {
    view! {
        <header class="site-header wrap">
            <a href="/" class="brand">{HANDLE}</a>
            <nav aria-label="Primary" class="primary-nav">
                {NAV
                    .iter()
                    .map(|&(id, label, href)| {
                        let active = id == current;
                        view! {
                            <a href=href aria-current=active.then_some("page")>
                                {label}
                            </a>
                        }
                    })
                    .collect_view()}
            </nav>
        </header>
    }
}

#[component]
pub fn SiteFooter() -> impl IntoView {
    view! {
        <footer class="site-footer wrap">
            <div class="site-footer-inner">
                <span class="muted">"© 2026 " {HANDLE} " · Built with Leptos"</span>
                <nav aria-label="Elsewhere" class="footer-nav">
                    <a href=GITHUB>"GitHub"</a>
                </nav>
            </div>
        </footer>
    }
}

/// Standard inner page: header, main column, footer.
#[component]
pub fn Page(
    current: &'static str,
    title: String,
    #[prop(optional)] description: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        <Title text=format!("{title} · {HANDLE}")/>
        {description.map(|d| view! { <Meta name="description" content=d/> })}
        <div class="page">
            <SiteHeader current/>
            <main class="wrap">{children()}</main>
            <SiteFooter/>
        </div>
    }
}

#[component]
pub fn PageHead(
    title: &'static str,
    #[prop(optional)] lede: Option<&'static str>,
) -> impl IntoView {
    view! {
        <section class="page-head">
            <h1 class="h-page">{title}</h1>
            {lede.map(|l| view! { <p class="lede">{l}</p> })}
        </section>
    }
}

#[component]
pub fn Breadcrumb(parent: &'static str, href: &'static str, here: &'static str) -> impl IntoView {
    view! {
        <nav aria-label="Breadcrumb" class="breadcrumb">
            <a href=href>{parent}</a>
            <span aria-hidden="true" class="muted">"/"</span>
            <span class="muted">{here}</span>
        </nav>
    }
}

/// One row of an index list (projects, posts, tools). Rows without a
/// destination render as plain blocks so there are no dead links.
#[component]
pub fn ListRow(
    href: Option<String>,
    meta_a: &'static str,
    meta_b: &'static str,
    title: &'static str,
    blurb: &'static str,
) -> impl IntoView {
    let inner = view! {
        <span class="row-meta mono">
            <span>{meta_a}</span>
            <span>{meta_b}</span>
        </span>
        <span class="row-body">
            <span class="row-title">{title}</span>
            <span class="row-blurb">{blurb}</span>
        </span>
    };
    match href {
        Some(href) => view! { <a class="row" href=href>{inner}</a> }.into_any(),
        None => view! { <div class="row">{inner}</div> }.into_any(),
    }
}
