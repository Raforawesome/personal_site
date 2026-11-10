use leptos::prelude::*;

use super::{Animation, Code, Post, PostLayout};

pub const POST: Post = Post {
    slug: "voxel-engine-v0-1",
    date: "27 Oct 2026",
    read: "6 min read",
    tags: "rust, wgpu, performance",
    title: "Voxel engine v0.1 writeup",
    blurb: "How chunks are generated, meshed and streamed, and what each stage costs.",
    body,
};

/// Path to a file in this post's media folder, `public/blog/voxel-engine-v0-1/`.
macro_rules! media {
    ($file:literal) => {
        concat!("/blog/voxel-engine-v0-1/", $file)
    };
}
const PERF: &str = "https://github.com/Raforawesome/between-wgpu/blob/master/docs/performance.md";

const TOC: &[(&str, &str)] = &[
    ("pipeline", "A lock-free pipeline"),
    ("trilinear", "Sampling less noise"),
    ("memory", "Walking memory in order"),
    ("packing", "A vertex in 32 bits"),
    ("faces", "Not meshing faces at all"),
    ("culling", "Culling and caching"),
    ("numbers", "Where it stands"),
];

const PACK_RUST: &str = r#"// bits 0..=4 x, 5..=9 y, 10..=14 z (chunk-local, 0..=16),
// 15..=16 UV corner, 17..=19 face id, 20..=31 spare
packed: (u32::from(position.x) << X_SHIFT)
    | (u32::from(position.y) << Y_SHIFT)
    | (u32::from(position.z) << Z_SHIFT)
    | (u32::from(uv_corner) << UV_CORNER_SHIFT)
    | (face_id << FACE_ID_SHIFT),"#;

const UNPACK_SLANG: &str = r#"let x = unpack_field(vert.packed, 0, 0b11111);
let y = unpack_field(vert.packed, 5, 0b11111);
let z = unpack_field(vert.packed, 10, 0b11111);
let uv_corner = unpack_field(vert.packed, 15, 0b11);
let face_id = unpack_field(vert.packed, 17, 0b111);"#;

const LOOP_ORDER: &str = r#"// blocks[x + y * 16 + z * 256]: x is contiguous, so it goes innermost
for lz in 0..CHUNK_SIZE {
    for ly in 0..CHUNK_SIZE {
        for lx in 0..CHUNK_SIZE {
            let idx = lx + ly * CHUNK_SIZE + lz * CHUNK_SIZE * CHUNK_SIZE;
            // ...
        }
    }
}"#;

fn body() -> AnyView {
    view! {
        <PostLayout toc=TOC>
            <p class="intro">
                "I’ve been building a voxel engine in Rust with wgpu and the Slang shading language. It’s been incredibly fun and satisfying to work on, and it’s taught me a lot about building high-performance compute pipelines. This is a tour of the parts I’m most excited about, with the numbers behind each one."
            </p>
            <p>
                "The engine is called Between. You can "
                <a href="/projects/voxel-engine">"fly around it in your browser"</a>
                ", and the source is "
                <a href="https://github.com/Raforawesome/between-wgpu">"on GitHub"</a>
                ". All measurements are from my MacBook Air (M2); the "
                <a href=PERF>"performance report"</a>
                " has the method and caveats."
            </p>

            <h2 id="pipeline">"A lock-free pipeline"</h2>
            <p>
                "The default view is 32 × 12 × 32 chunks of 16³ voxels: 12,288 chunks around the camera, all of which need generating and meshing."
            </p>
            <p>
                "My first attempt was a simple shared world state that could be passed anywhere. I tried dashmap and then papaya for it, but depending on the crate I kept running into either locking issues or temporal issues. So I rethought the architecture: one main thread owns the world, and the workers only ever see immutable snapshots of it."
            </p>
            <p>
                "The main thread spawns four persistent worker threads. Each one pulls generate or mesh tasks off a shared crossbeam channel, then either generates a chunk and returns it to the chunk manager, or meshes one. I went with crossbeam over rayon because crossbeam lets me build my own pipeline, while rayon seems to sit an abstraction level higher and is built more for splitting up batches of work than for long-lived workers. With crossbeam, an idle worker just blocks on the channel instead of polling, so persistent workers are very efficient."
            </p>
            <p>
                "Meshing a chunk needs its six neighbours, to know which faces on its borders are hidden. Instead of sharing the world behind a lock, the chunk manager hands each mesh task immutable references to those neighbours. Workers never touch shared mutable state, so there are no locks at all. Because the snapshots are just "
                <code>"Arc"</code>
                " references, the memory footprint was much lower than I expected, and the "
                <code>"Arc"</code>
                " overhead turned out to be insignificant. I’m not sure how much of that is down to Apple Silicon’s fast atomics."
            </p>
            <p>
                "Four workers scale almost perfectly: 4.2× faster than one. Eight is still faster, but efficiency drops to 74%, probably because the M2 has 4 performance and 4 efficiency cores. The result: the full 12k-chunk view generates and meshes in 78 ms, and flying over brand-new terrain runs as fast as standing still. Streaming doesn’t show up in steady-state frame time."
            </p>

            <h2 id="trilinear">"Sampling less noise"</h2>
            <p>
                "Terrain comes from layered simplex noise, and when I profiled chunk generation, noise sampling dominated it. Evaluating the noise at every voxel means 4,096 samples per chunk. Instead I sample a coarse 5 × 5 × 5 grid and fill in everything between with trilinear interpolation."
            </p>
            <Animation
                src=media!("trilinear.mp4")
                poster=media!("trilinear.jpg")
                label="Noise sampled at every voxel compared with a coarse grid filled in by trilinear interpolation"
                caption="Figure 1. Sampling a 5³ grid and interpolating instead of sampling all 4,096 voxels."
            />
            <p>
                "That’s a 9.4× speedup, from 186 to 20 µs per chunk. The best part is that only 0.11% of voxels change block type, all of them on the surface, so the difference is essentially invisible."
            </p>
            <p>
                "I could have switched to something cheaper, like 2D noise, but it gave less interesting worlds and wasn’t as fast as this anyway. 3D noise also leaves the door open for things like cave carving later."
            </p>

            <h2 id="memory">"Walking memory in order"</h2>
            <p>
                "Profiling didn’t point me to this one, but even small things matter. Chunks are flat 4 KB arrays laid out x → y → z. My loops originally went x, y, z from the outside in, the way most people would write them, so every step of the inner loop jumped 256 bytes through memory. Flipping the order to z outermost and x innermost makes the inner loop walk memory in a straight line."
            </p>
            <Code lang="rust" code=LOOP_ORDER/>
            <Animation
                src=media!("cache_iteration.mp4")
                poster=media!("cache_iteration.jpg")
                label="Two loop orders walking a chunk array: x-innermost reads memory in order, z-innermost jumps around"
                caption="Figure 2. The same chunk, two loop orders. The animation uses a toy cache."
            />
            <p>
                "Walking memory in order lets the hardware prefetcher stay ahead and gives the compiler a shot at auto-vectorizing the loop."
                <sup>
                    <a href="#fn1" class="fn-ref">"1"</a>
                </sup>
            </p>

            <h2 id="packing">"A vertex in 32 bits"</h2>
            <p>
                "This is one of my favourite optimizations, and it’s all about smart layouts and custom data formats. A typical vertex is 32 bytes: a float position, a normal and a UV. Each of mine is a single 32-bit integer, which means 8× less vertex memory and bandwidth."
            </p>
            <ul>
                <li>
                    <strong>"x, y, z: "</strong>
                    "5 bits each, 0–16 within the chunk. The vertex shader adds the chunk’s world offset."
                </li>
                <li>
                    <strong>"UV corner: "</strong>
                    "2 bits."
                </li>
                <li>
                    <strong>"Face: "</strong>
                    "3 bits."
                </li>
                <li>
                    <strong>"Spare: "</strong>
                    "12 bits for later."
                </li>
            </ul>
            <p>
                "There’s no normal, because the face id implies it, and no block type: the fragment shader looks that up in a per-chunk 16³ block texture. Packing on the CPU is a few shifts:"
            </p>
            <Code lang="rust" code=PACK_RUST/>
            <p>"and the vertex shader undoes it:"</p>
            // Slang has no grammar here, but this snippet reads fine as Rust.
            <Code lang="rust" code=UNPACK_SLANG/>
            <Animation
                src=media!("vertex_packing.mp4")
                poster=media!("vertex_packing.jpg")
                label="The fields of a vertex being packed bit by bit into one 32-bit integer"
                caption="Figure 3. Everything the shader needs about a vertex, in one u32."
            />

            <h2 id="faces">"Not meshing faces at all"</h2>
            <p>
                "Packing is nice, but the bigger win is not meshing most faces in the first place. Workers check each block’s neighbours, across chunk borders via those immutable neighbour references, and skip any face that can’t be seen."
            </p>
            <Animation
                src=media!("face_culling.mp4")
                poster=media!("face_culling.jpg")
                label="Hidden faces between solid blocks being removed until only the outer surface remains"
                caption="Figure 4. Only faces that touch air get meshed."
            />
            <p>
                "Across the region I benchmarked, that’s 78 thousand faces emitted, against 15.8 million if every solid block drew all six: 99.5% removed."
            </p>

            <h2 id="culling">"Culling and caching"</h2>
            <p>
                "Most of what’s left is outside the camera’s view at any moment. A conservative frustum test against each chunk’s bounding box, which works with the renderer’s infinite reversed-Z projection, cuts the submitted triangles about 4× and raises the median frame rate by about 16%. Each test costs 8.5 ns."
            </p>
            <p>
                "16% from 4× fewer triangles surprised me at first. At about 4 ms a frame, the renderer clearly isn’t limited by triangle count at this scene size, so per-draw overhead or presentation is probably the bottleneck instead. I haven’t profiled which yet."
            </p>
            <p>
                "Meshes that drop out of view aren’t thrown away immediately. An LRU cache keeps up to 2,048 recently visible meshes on the GPU within a 64 MiB budget, so turning around doesn’t mean remeshing. It’s O(1) per operation: 47× faster than a simple linear-scan LRU at that size."
            </p>

            <h2 id="numbers">"Where it stands"</h2>
            <p>
                "At 3000 × 1800 with the default view, the engine runs at about 240 FPS median uncapped on the M2 (4.0 ms p50, 7.6 ms p95), and holds 60 FPS under vsync with plenty of headroom."
            </p>
            <p>
                "Next up is meshing. It still emits one quad per visible face, so greedy meshing (merging runs of faces into larger quads) is the obvious next step, and I’ll measure it against the baseline in the report."
            </p>

            <p id="fn1" class="footnote">
                "1. The animation uses a toy cache to make the effect visible. A real 16³ chunk is 4 KB, so it fits entirely in L1."
            </p>
        </PostLayout>
    }
    .into_any()
}
