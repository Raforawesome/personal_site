use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::{Breadcrumb, ListRow, Page, PageHead};
use crate::content::{POSTS, Post};
use crate::pages::not_found::NotFound;

#[component]
pub fn BlogIndex() -> impl IntoView {
    view! {
        <Page current="blog" title="Blog".into()>
            <PageHead title="Blog"/>
            <section aria-label="Posts" class="rows">
                {POSTS
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
    let slug = use_params_map().read_untracked().get("slug").unwrap_or_default();
    let Some(idx) = POSTS.iter().position(|p| p.slug == slug) else {
        return view! { <NotFound/> }.into_any();
    };
    let post = &POSTS[idx];
    // POSTS is newest first: "previous" is older (later in the list).
    let prev = POSTS.get(idx + 1);
    let next = idx.checked_sub(1).and_then(|i| POSTS.get(i));

    let body = match post.slug {
        "voxel-engine-v0-1" => view! { <VoxelWriteup/> }.into_any(),
        _ => ().into_any(),
    };

    view! {
        <Page current="blog" title=post.title.to_string() description=post.blurb>
            <Breadcrumb parent="Blog" href="/blog" here=post.title/>
            <header class="post-head">
                <h1 class="h-post">{post.title}</h1>
                <p class="mono muted small">
                    {post.date} " · " {post.read} " · " {post.tags}
                </p>
            </header>
            {body}
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

/// Two-column layout: table of contents beside the article.
#[component]
fn PostLayout(toc: &'static [(&'static str, &'static str)], children: Children) -> impl IntoView {
    view! {
        <div class="post-layout">
            <aside aria-label="On this page" class="toc">
                <p class="toc-label">"On this page"</p>
                <ol>
                    {toc
                        .iter()
                        .map(|&(id, label)| view! { <li><a href=format!("#{id}")>{label}</a></li> })
                        .collect_view()}
                </ol>
            </aside>
            <article class="article">{children()}</article>
        </div>
    }
}

#[component]
fn VoxelWriteup() -> impl IntoView {
    view! {
        <PostLayout toc=&[("s1", "[Section]"), ("s2", "[Section]"), ("s3", "[Section]")]>
            <p class="intro">
                "[Opening paragraph. State the problem and what you found in two or three sentences.]"
            </p>

            <h2 id="s1">"[Section heading]"</h2>
            <p>
                "[Body paragraph. Prose is set in a serif at about 19px with a 66-character line, so long posts stay readable.]"
            </p>
            <p>
                "[Another paragraph, with an " <a href="#">"inline link"</a> " and a footnote marker"
                <sup>
                    <a href="#fn1" class="fn-ref">"1"</a>
                </sup> ".]"
            </p>
            <pre>
                <code>
                    "// [code from the post]\nfn mesh_chunk(chunk: &Chunk) -> Mesh {\n    todo!()\n}"
                </code>
            </pre>

            <h2 id="s2">"[Section heading]"</h2>
            <p>"[Body paragraph.]"</p>
            <figure class="fig">
                <div class="placeholder mono">"Figure or chart"</div>
                <figcaption class="caption">"Figure 1. [Caption.]"</figcaption>
            </figure>

            <h2 id="s3">"[Section heading]"</h2>
            <p>"[Closing paragraph.]"</p>

            <p id="fn1" class="footnote">"1. [Footnote text.]"</p>
        </PostLayout>
    }
}
