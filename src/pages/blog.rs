//! The blog index and post pages. The posts themselves live in `src/posts/`.

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::{Breadcrumb, ListRow, Page, PageHead};
use crate::pages::not_found::NotFound;
use crate::posts::{self, Post};

#[component]
pub fn BlogIndex() -> impl IntoView {
    view! {
        <Page current="blog" title="Blog".into()>
            <PageHead title="Blog"/>
            <section aria-label="Posts" class="rows">
                {posts::ALL
                    .iter()
                    .map(|p| {
                        view! {
                            <ListRow
                                href=Some(format!("/blog/{}", p.slug))
                                meta_a=p.date
                                meta_b=p.read
                                title=p.title
                                blurb=p.blurb
                            />
                        }
                    })
                    .collect_view()}
            </section>
        </Page>
    }
}

#[component]
pub fn BlogPost() -> impl IntoView {
    let slug = use_params_map()
        .read_untracked()
        .get("slug")
        .unwrap_or_default();
    let Some((idx, post)) = posts::find(&slug) else {
        return view! { <NotFound/> }.into_any();
    };
    // posts::ALL is newest first: "previous" is older (later in the list).
    let prev = posts::ALL.get(idx + 1);
    let next = idx.checked_sub(1).and_then(|i| posts::ALL.get(i));

    view! {
        <Page current="blog" title=post.title.to_string() description=post.blurb>
            <Breadcrumb parent="Blog" href="/blog" here=post.title/>
            <header class="post-head">
                <h1 class="h-post">{post.title}</h1>
                <p class="mono muted small">
                    {post.date} " · " {post.read} " · " {post.tags}
                </p>
            </header>
            {(post.body)()}
            <PostNav prev next/>
        </Page>
    }
    .into_any()
}

#[component]
fn PostNav(prev: Option<&'static Post>, next: Option<&'static Post>) -> impl IntoView {
    if prev.is_none() && next.is_none() {
        return ().into_any();
    }
    view! {
        <nav aria-label="More posts" class="post-nav">
            {prev.map(|p| view! { <a href=format!("/blog/{}", p.slug)>"← " {p.title}</a> })}
            <span></span>
            {next.map(|p| view! { <a href=format!("/blog/{}", p.slug)>{p.title} " →"</a> })}
        </nav>
    }
    .into_any()
}
