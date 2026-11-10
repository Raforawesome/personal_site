#![recursion_limit = "256"]

pub mod app;
pub mod components;
pub mod content;
pub mod islands;
pub mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    // Only `#[island]` components hydrate; everything else stays static HTML.
    leptos::mount::hydrate_islands();
}
