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
    let slug = use_params_map()
        .read_untracked()
        .get("slug")
        .unwrap_or_default();
    match slug.as_str() {
        "voxel-engine" => view! { <VoxelEngine/> }.into_any(),
        _ => view! { <NotFound/> }.into_any(),
    }
}

const REPO: &str = "https://github.com/Raforawesome/between-wgpu";
const THREAD: &str = "https://x.com/raforawesome/status/2114935770566062370";
const PERF: &str = "https://github.com/Raforawesome/between-wgpu/blob/master/docs/performance.md";

const STAGES: &[(&str, &str, &str)] = &[
    (
        "01",
        "Scheduler",
        "Main thread. Picks the chunks inside view distance when the player crosses a chunk boundary.",
    ),
    (
        "02",
        "Generation",
        "Worker pool. Samples layered noise on a 5³ grid and fills the 16³ chunk by trilinear interpolation.",
    ),
    (
        "03",
        "Meshing",
        "Worker pool. Runs once all six neighbours exist, on immutable snapshots, and drops hidden faces.",
    ),
    (
        "04",
        "Upload",
        "Main thread. Packed u32 vertices, indices and a 16³ block-ID texture per chunk.",
    ),
    (
        "05",
        "Render",
        "GPU. Frustum-culled draws with infinite reversed Z; an LRU keeps recent meshes resident.",
    ),
];

/// (stage, cost, notes). All figures are from one machine (MacBook Air, M2).
const MEASUREMENTS: &[(&str, &str, &str)] = &[
    (
        "Generation",
        "18.5 µs / chunk",
        "One thread, 2,048 chunks. 4.2× faster on 4 threads, 5.9× on 8 (see Figure 2).",
    ),
    (
        "Meshing",
        "50.9 µs / mesh",
        "Per non-empty chunk; 12.3 µs averaged over all chunks. 99.5% of faces removed.",
    ),
    (
        "Full view load",
        "78 ms",
        "Generate and mesh all 12,288 chunks on 4 threads. GPU upload not included.",
    ),
    (
        "Culling",
        "59 µs / frame",
        "Tests 6,940 chunk AABBs (8.5 ns each) and culls 78% of them.",
    ),
    (
        "Render",
        "4.0 ms / frame",
        "p50 at 3000×1800, uncapped; 7.6 ms p95. Holds 60 FPS under vsync.",
    ),
];

/// Wall time to generate the default 12,288-chunk region (median of 5).
const SCALING: &[(u32, u32, &str)] = &[
    (1, 199, "1.00×"),
    (2, 99, "2.01×"),
    (4, 48, "4.18×"),
    (8, 34, "5.88×"),
];

/// Thread-scaling bars, rendered on the server as plain SVG.
#[component]
fn ScalingChart() -> impl IntoView {
    const LABEL_W: f64 = 92.0;
    const BAR_MAX: f64 = 420.0;
    const ROW_H: f64 = 44.0;
    let max = f64::from(SCALING[0].1);
    let height = ROW_H * SCALING.len() as f64 + 8.0;
    view! {
        <svg
            class="chart"
            viewBox=format!("0 0 640 {height}")
            role="img"
            aria-label="Generation wall time by thread count: 1 thread 199 ms, 2 threads 99 ms, 4 threads 48 ms, 8 threads 34 ms."
        >
            {SCALING
                .iter()
                .enumerate()
                .map(|(i, &(threads, ms, speedup))| {
                    let y = i as f64 * ROW_H + 8.0;
                    let w = (f64::from(ms) / max * BAR_MAX).max(2.0);
                    let label = if threads == 1 { "1 thread".to_string() } else { format!("{threads} threads") };
                    let class = if threads == 4 { "bar bar--default" } else { "bar" };
                    view! {
                        <g>
                            <text x="0" y=y + 19.0 class="chart-label">{label}</text>
                            <rect x=LABEL_W y=y width=w height="26" class=class></rect>
                            <text x=LABEL_W + w + 10.0 y=y + 19.0 class="chart-value">
                                {format!("{ms} ms · {speedup}")}
                            </text>
                        </g>
                    }
                })
                .collect_view()}
        </svg>
    }
}

#[component]
fn VoxelEngine() -> impl IntoView {
    view! {
        <Page
            current="projects"
            title="Voxel engine".into()
            description="Between: a voxel terrain engine in Rust, wgpu and Slang, with lock-free terrain workers, 4-byte vertices and a WebGPU build you can run in the browser."
        >
            <Breadcrumb parent="Projects" href="/projects" here="Voxel engine"/>

            <section class="page-head page-head--tight">
                <h1 class="h-page">"Voxel engine"</h1>
                <p class="lede">
                    "Between is a voxel terrain engine in Rust, wgpu and Slang shaders. This page covers how a chunk goes from a density field, through worker threads, into compact GPU geometry, and what each stage costs."
                </p>
            </section>

            <dl class="facts">
                <div>
                    <dt>"Stack"</dt>
                    <dd>"Rust, wgpu, Slang"</dd>
                </div>
                <div>
                    <dt>"Status"</dt>
                    <dd>"v0.1 shipped"</dd>
                </div>
                <div>
                    <dt>"Write-up"</dt>
                    <dd>
                        <a class="fact-link" href=THREAD>"Thread on X"</a>
                    </dd>
                </div>
                <div>
                    <dt>"Source"</dt>
                    <dd>
                        <a class="fact-link" href=REPO>"GitHub (GPL-3.0)"</a>
                    </dd>
                </div>
            </dl>

            <figure class="demo">
                <RunDemo
                    src="/demo/voxel/index.html".to_string()
                    poster="/images/between-terrain.jpg".to_string()
                    poster_alt="Seed 42: a sandy coastline with stepped grass terraces beside open water, rendered by Between".to_string()
                />
                <figcaption class="caption">
                    "The real engine compiled to WebAssembly, about 330 KB gzipped, fetched only when you press the button. Needs WebGPU. The browser build runs terrain jobs on the main thread with an 8 ms budget per frame and a smaller view (20×8×20 chunks); "
                    <a href="/demo/voxel/index.html?view=16&height=6">"try a bigger one"</a>
                    "."
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
                <p class="caption">"Figure 1. Chunk pipeline."</p>
            </section>

            <section aria-labelledby="goal" class="sec prose-col">
                <h2 id="goal" class="h-sec">"The Goal"</h2>
                <p class="prose">
                    "Keep a 32 × 12 × 32-chunk view around the camera (12,288 chunks, about 50 million voxels) generated, meshed and on the GPU without the frame rate noticing, including while flying into new terrain at sprint speed."
                </p>
            </section>

            <section aria-labelledby="approach" class="sec prose-col">
                <h2 id="approach" class="h-sec">"Approach"</h2>
                <p class="prose">
                    "The main thread owns the world and hands work to four persistent workers over a crossbeam channel. A meshing job carries immutable snapshots of its six neighbours, so workers never take a lock, and a chunk is only meshed once all six exist. Generation samples noise on a 5³ grid and interpolates the rest, which is 9.4× faster than sampling every voxel and changes 0.11% of voxels, all at the surface."
                </p>
                <p class="prose">
                    "Each vertex is a single u32: 5 bits per axis inside the chunk, 2 bits of UV corner and 3 bits of face. That’s 8× smaller than a typical 32-byte float vertex. There is no normal (the face implies it) and no block type; the fragment shader reads that from a per-chunk 16³ block texture. Faces between solid blocks are never emitted, and chunk-AABB frustum culling cuts submitted triangles about 4×."
                </p>
            </section>

            <section aria-labelledby="measurements" class="sec">
                <h2 id="measurements" class="h-sec">"Measurements"</h2>
                <p class="prose prose-col">
                    "MacBook Air, M2 (4 performance + 4 efficiency cores), macOS, nightly rustc, release profile, seed 42, camera at spawn. CPU figures come from a std-only harness with a counting allocator: median of 5 runs after a warm-up. Frame times come from a scripted camera route through the real renderer and are wall-clock frame intervals, not isolated GPU time. "
                    <a href=PERF>"Full report and caveats"</a>
                    "."
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
                                            <td class="mono nowrap">{cost}</td>
                                            <td class="muted">{note}</td>
                                        </tr>
                                    }
                                })
                                .collect_view()}
                        </tbody>
                    </table>
                </div>
                <figure class="fig">
                    <ScalingChart/>
                    <figcaption class="caption">
                        "Figure 2. Wall time to generate the default 12,288-chunk region, by worker count. Scaling is near-linear up to 4 threads (the app’s default, highlighted). The jump to 8 only reaches 5.9×, 74% efficiency, most likely because the M2 in a MacBook Air has 4 performance and 4 efficiency cores: past four threads, the extra work lands on the slower cores."
                    </figcaption>
                </figure>
            </section>

            <section aria-labelledby="next" class="sec prose-col">
                <h2 id="next" class="h-sec">"What’s next"</h2>
                <p class="prose">
                    "Meshing comes first. The mesher emits one quad per visible face rather than greedy-meshed runs, and that’s where most of the remaining vertex count and meshing time goes. After that, budgeting mesh uploads per frame: today the first load has a few long frames (78 ms at worst) before it settles."
                </p>
                <p class="prose">
                    "Then memory and edges. Generated chunks are never evicted from the CPU (about 56 MiB at the default view, growing as you travel), and generation covers only the render region with no neighbour halo, so chunks on the boundary can stay unmeshed until you move."
                </p>
            </section>

            <div class="back">
                <a href="/projects">"← All projects"</a>
            </div>
        </Page>
    }
}
