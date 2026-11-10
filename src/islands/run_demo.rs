//! "Run it in your browser" for the voxel engine. The engine's own WASM
//! lives at `src` (a static page under `public/`) and only loads after the
//! button is pressed, so the site's bundle never carries it.

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum State {
    Poster,
    Running,
    NoWebGpu,
}

fn has_webgpu() -> bool {
    #[cfg(feature = "hydrate")]
    {
        let nav = window().navigator();
        js_sys::Reflect::get(&nav, &"gpu".into()).is_ok_and(|g| !g.is_undefined() && !g.is_null())
    }
    #[cfg(not(feature = "hydrate"))]
    {
        false
    }
}

#[island]
pub fn RunDemo(src: String) -> impl IntoView {
    let (state, set_state) = signal(State::Poster);
    let start = move |_| {
        set_state.set(if has_webgpu() { State::Running } else { State::NoWebGpu });
    };

    view! {
        <div class="demo-frame">
            {move || match state.get() {
                State::Running => {
                    view! {
                        <iframe
                            class="demo-iframe"
                            src=src.clone()
                            title="Voxel engine running in the browser"
                            allow="fullscreen"
                        ></iframe>
                    }
                        .into_any()
                }
                State::Poster => {
                    view! {
                        <p class="mono demo-note">"Poster frame (10-second loop)"</p>
                        <button type="button" class="btn-accent" on:click=start>
                            "Run it in your browser"
                        </button>
                    }
                        .into_any()
                }
                State::NoWebGpu => {
                    view! {
                        <p class="mono demo-note" role="status">
                            "This browser doesn’t support WebGPU, so the engine can’t run here. Try a recent Chrome, Edge or Safari."
                        </p>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
