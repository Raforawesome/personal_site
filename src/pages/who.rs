use leptos::prelude::*;

use crate::components::Page;
use crate::content::CONTACT;

#[component]
pub fn Who() -> impl IntoView {
    view! {
        <Page
            current="who"
            title="Who am I?".into()
            description="Tahir Chaudhry: computer science student at Wilfrid Laurier, looking for a software internship."
        >
            <section class="page-head page-head--about">
                <h1 class="h-page">"Tahir Chaudhry"</h1>
                <p class="lede">
                    "I’m a computer science student at Wilfrid Laurier, and I’m looking for a software internship."
                </p>
                <p class="lede">
                    "I’m interested in systems, embedded and graphics work. Rust is the language I reach for first, but I’m comfortable in many."
                </p>
            </section>
            <section aria-label="Contact" class="contact">
                {CONTACT
                    .iter()
                    .map(|l| {
                        view! {
                            <div class="contact-row">
                                <span class="contact-label mono muted">{l.label}</span>
                                <a class="contact-link" href=l.href>{l.text}</a>
                            </div>
                        }
                    })
                    .collect_view()}
            </section>
        </Page>
    }
}
