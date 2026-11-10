use leptos::prelude::*;

use crate::components::{Page, PageHead};

#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    if let Some(resp) = use_context::<leptos_axum::ResponseOptions>() {
        resp.set_status(axum::http::StatusCode::NOT_FOUND);
    }

    view! {
        <Page current="" title="Not found".into()>
            <PageHead title="Not found" lede="Nothing lives at this address."/>
            <div class="back">
                <a href="/">"← Home"</a>
            </div>
        </Page>
    }
}
