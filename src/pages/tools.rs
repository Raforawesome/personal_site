use leptos::prelude::*;

use crate::components::{ListRow, Page, PageHead};
use crate::content::TOOLS;

#[component]
pub fn Tools() -> impl IntoView {
    view! {
        <Page current="tools" title="Tools".into()>
            <PageHead title="Tools"/>
            <section aria-label="Tools" class="rows">
                {TOOLS
                    .iter()
                    .map(|t| {
                        view! {
                            <ListRow
                                href=t.href.map(String::from)
                                meta_a=t.kind
                                meta_b=t.size
                                title=t.title
                                blurb=t.blurb
                            />
                        }
                    })
                    .collect_view()}
            </section>
        </Page>
    }
}
