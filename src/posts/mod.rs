//! Blog posts: one file per post in this folder.
//!
//! To add a post:
//! 1. Copy an existing post file (e.g. `voxel_engine_v0_1.rs`) and edit its
//!    `POST` metadata and `body()`.
//! 2. Add a `mod` line for it below.
//! 3. Put its `POST` at the top of [`ALL`] (newest first).
//!
//! Images and videos for a post go in `public/blog/<slug>/`.

use leptos::prelude::*;

mod voxel_engine_v0_1;

/// Every published post, newest first. The blog index lists them in this
/// order, and "previous"/"next" links on a post follow it.
pub const ALL: &[Post] = &[voxel_engine_v0_1::POST];

pub struct Post {
    /// URL: `/blog/{slug}`.
    pub slug: &'static str,
    pub date: &'static str,
    pub read: &'static str,
    pub tags: &'static str,
    pub title: &'static str,
    /// One sentence for the blog index and the page description.
    pub blurb: &'static str,
    pub body: fn() -> AnyView,
}

/// Looks up a post by slug, with its position in [`ALL`].
pub fn find(slug: &str) -> Option<(usize, &'static Post)> {
    ALL.iter().enumerate().find(|(_, p)| p.slug == slug)
}

// ---------------------------------------------------------------------------
// Building blocks for post bodies
// ---------------------------------------------------------------------------

/// Two-column layout: "On this page" contents beside the article. `toc` is
/// a list of (heading id, label).
#[component]
pub fn PostLayout(
    toc: &'static [(&'static str, &'static str)],
    children: Children,
) -> impl IntoView {
    view! {
        <div class="post-layout">
            <aside aria-label="On this page" class="toc">
                <p class="toc-label">"On this page"</p>
                <ol>
                    {toc
                        .iter()
                        .map(|&(id, label)| {
                            view! {
                                <li>
                                    <a href=format!("#{id}")>{label}</a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ol>
            </aside>
            <article class="article">{children()}</article>
        </div>
    }
}

/// A short looping animation (muted, so browsers allow autoplay), with a
/// poster frame for before it loads and a caption.
#[component]
pub fn Animation(
    src: &'static str,
    poster: &'static str,
    /// Describes what the animation shows, for screen readers.
    label: &'static str,
    caption: &'static str,
) -> impl IntoView {
    view! {
        <figure class="fig">
            <video
                class="post-video"
                src=src
                poster=poster
                aria-label=label
                autoplay=true
                muted=true
                loop=true
                playsinline=true
                controls=true
                preload="metadata"
            ></video>
            <figcaption class="caption">{caption}</figcaption>
        </figure>
    }
}

/// A code block, highlighted on the server. `lang` is a syntect language
/// token such as `"rust"`; unknown languages are shown as plain text.
#[component]
pub fn Code(lang: &'static str, code: &'static str) -> impl IntoView {
    view! {
        <pre>
            <code inner_html=highlight(lang, code)></code>
        </pre>
    }
}

/// Wraps tokens in `<span class="hl-…">`; the colours live in main.css.
#[cfg(feature = "ssr")]
fn highlight(lang: &str, code: &str) -> String {
    use std::sync::LazyLock;
    use syntect::html::{ClassStyle, ClassedHTMLGenerator};
    use syntect::parsing::SyntaxSet;
    use syntect::util::LinesWithEndings;

    static SYNTAXES: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
    let syntax = SYNTAXES
        .find_syntax_by_token(lang)
        .unwrap_or_else(|| SYNTAXES.find_syntax_plain_text());
    let mut html = ClassedHTMLGenerator::new_with_class_style(
        syntax,
        &SYNTAXES,
        ClassStyle::SpacedPrefixed { prefix: "hl-" },
    );
    for line in LinesWithEndings::from(code) {
        // Only fails on a broken grammar, which the bundled ones aren't.
        let _ = html.parse_html_for_line_which_includes_newline(line);
    }
    html.finalize()
}

/// Posts only render on the server, so the client never needs this.
#[cfg(not(feature = "ssr"))]
fn highlight(_lang: &str, _code: &str) -> String {
    String::new()
}
