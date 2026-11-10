use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::{Breadcrumb, ListRow, Page, PageHead};
use crate::content::PROJECTS;
use crate::islands::run_demo::RunDemo;
use crate::pages::not_found::NotFound;

#[component]
pub fn ProjectsIndex() -> impl IntoView {
    view! {
        <Page current="projects" title="Projects".into()>
            <PageHead title="Projects"/>
            <section aria-label="All projects" class="rows">
                {PROJECTS
                    .iter()
                    .map(|p| {
                        view! {
                            <ListRow
                                href=p.slug.map(|s| format!("/projects/{s}"))
                                meta_a=p.stack
                                meta_b=p.status
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
pub fn ProjectDetail() -> impl IntoView {
    let slug = use_params_map().read_untracked().get("slug").unwrap_or_default();
    match slug.as_str() {
        "voxel-engine" => view! { <VoxelEngine/> }.into_any(),
        _ => view! { <NotFound/> }.into_any(),
    }
}

const STAGES: &[(&str, &str, &str)] = &[
    ("01", "Scheduler", "Main thread. Loads and unloads chunks by view distance."),
    ("02", "Generation", "Worker pool."),
    ("03", "Meshing", "Worker pool."),
    ("04", "Upload", "Main thread."),
    ("05", "Render", "GPU."),
];

const MEASUREMENTS: &[(&str, &str, &str)] = &[
    ("Generation", "[__] µs / chunk", "[Note]"),
    ("Meshing", "[__] µs / chunk", "[Note]"),
    ("GPU upload", "[__] MB / frame", "[Note]"),
    ("Render", "[__] ms / frame", "[Note]"),
];

#[component]
fn VoxelEngine() -> impl IntoView {
    view! {
        <Page
            current="projects"
            title="Voxel engine".into()
            description="A multithreaded voxel renderer and engine in Rust and wgpu."
        >
            <Breadcrumb parent="Projects" href="/projects" here="Voxel engine"/>

            <section class="page-head page-head--tight">
                <h1 class="h-page">"Voxel engine"</h1>
                <p class="lede">
                    "A multithreaded voxel renderer and engine in Rust and wgpu. This page covers how chunks are generated, meshed and streamed, and what each stage costs."
                </p>
            </section>

            <dl class="facts">
                <div>
                    <dt>"Stack"</dt>
                    <dd>"Rust, wgpu"</dd>
                </div>
                <div>
                    <dt>"Status"</dt>
                    <dd>"v0.1 shipped"</dd>
                </div>
                <div>
                    <dt>"Write-up"</dt>
                    <dd>
                        <a class="fact-link" href="/blog/voxel-engine-v0-1">"Blog post"</a>
                    </dd>
                </div>
                <div>
                    <dt>"Source"</dt>
                    <dd>"[Link, or on request]"</dd>
                </div>
            </dl>

            <figure class="demo">
                <RunDemo src="/demo/voxel/index.html".to_string()/>
                <figcaption class="caption">
                    "Loads WASM only when you press the button. Needs WebGPU. [size] KB."
                </figcaption>
            </figure>

            <section aria-labelledby="pipeline" class="sec sec--wide">
                <h2 id="pipeline" class="h-sec">"How a chunk gets to the screen"</h2>
                <ol class="stages">
                    {STAGES
                        .iter()
                        .map(|&(no, name, place)| {
                            view! {
                                <li>
                                    <span class="mono muted">{no}</span>
                                    <span class="stage-name">{name}</span>
                                    <span class="stage-where">{place}</span>
                                </li>
                            }
                        })
                        .collect_view()}
                </ol>
                <p class="caption">"Figure 1. Chunk pipeline. [Confirm stages.]"</p>
            </section>

            <section aria-labelledby="problem" class="sec prose-col">
                <h2 id="problem" class="h-sec">"Problem"</h2>
                <p class="prose">
                    "[The constraint that drove the design, for example view distance against a 16.6 ms frame budget. Two or three concrete sentences.]"
                </p>
            </section>

            <section aria-labelledby="approach" class="sec prose-col">
                <h2 id="approach" class="h-sec">"Approach"</h2>
                <p class="prose">
                    "[Why occupancy masks, why packed vertices, how work moves between threads, and the alternative you rejected.]"
                </p>
            </section>

            <section aria-labelledby="measurements" class="sec">
                <h2 id="measurements" class="h-sec">"Measurements"</h2>
                <p class="prose prose-col">
                    "[Method first: hardware, build flags, warm-up, number of runs. Then the numbers.]"
                </p>
                <div class="table-scroll">
                    <table class="data">
                        <thead>
                            <tr>
                                <th scope="col">"Stage"</th>
                                <th scope="col">"Cost"</th>
                                <th scope="col">"Notes"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {MEASUREMENTS
                                .iter()
                                .map(|&(stage, cost, note)| {
                                    view! {
                                        <tr>
                                            <th scope="row">{stage}</th>
                                            <td class="mono">{cost}</td>
                                            <td class="muted">{note}</td>
                                        </tr>
                                    }
                                })
                                .collect_view()}
                        </tbody>
                    </table>
                </div>
                <figure class="fig">
                    <div class="placeholder placeholder--tall mono">
                        "Chart: frame time against view distance, rendered at build time from benches/*.json"
                    </div>
                    <figcaption class="caption">"Figure 2. [Caption.]"</figcaption>
                </figure>
            </section>

            <section aria-labelledby="change" class="sec prose-col">
                <h2 id="change" class="h-sec">"What I’d change"</h2>
                <p class="prose">
                    "[The bottleneck you know about and haven’t fixed. Interviewers read this section first.]"
                </p>
            </section>

            <div class="back">
                <a href="/projects">"← All projects"</a>
            </div>
        </Page>
    }
}
