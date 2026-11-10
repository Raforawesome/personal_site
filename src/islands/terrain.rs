//! Home page voxel terrain. The server renders the first frame as plain SVG,
//! so it shows without JS; the island then animates it and reacts to the pointer.

use leptos::prelude::*;

const N: usize = 22;
const TW: f64 = 30.0;
const TH: f64 = 15.0;
const ZH: f64 = 10.0;
const OX: f64 = 350.0;
const OY: f64 = 190.0;
const MAXH: i32 = 13;
const VX: f64 = 0.0;
const VY: f64 = 90.0;
const VW: f64 = 700.0;
const VH: f64 = 460.0;
const HW: f64 = TW / 2.0;
const HH: f64 = TH / 2.0;
const NO_PTR: (i32, i32) = (-99, -99);

const LOW: [f64; 3] = [0x2E as f64, 0x42 as f64, 0x39 as f64];
const HIGH: [f64; 3] = [0x9D as f64, 0xB8 as f64, 0xAB as f64];
const ACC: [f64; 3] = [0xE8 as f64, 0x64 as f64, 0x3A as f64];

/// Seeded terrain heights (mulberry32), so a given day always gives the same world.
fn make_terrain(seed: u32) -> Vec<i32> {
    let mut a = seed;
    let mut rnd = move || {
        a = a.wrapping_add(0x6D2B_79F5);
        let mut t = (a ^ (a >> 15)).wrapping_mul(1 | a);
        t = t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t)) ^ t;
        f64::from(t ^ (t >> 14)) / 4_294_967_296.0
    };
    let tau = 6.28;
    let f1 = 0.26 + rnd() * 0.16;
    let f2 = 0.24 + rnd() * 0.16;
    let f3 = 0.14 + rnd() * 0.12;
    let f4 = 0.24 + rnd() * 0.16;
    let p1 = rnd() * tau;
    let p2 = rnd() * tau;
    let p3 = rnd() * tau;
    let p4 = rnd() * tau;
    let a1 = 1.6 + rnd() * 1.2;
    let a2 = 0.9 + rnd() * 0.9;
    let a3 = 0.5 + rnd() * 0.7;
    let mid = (N as f64 - 1.0) / 2.0;
    let mut grid = Vec::with_capacity(N * N);
    for i in 0..N {
        for j in 0..N {
            let (fi, fj) = (i as f64, j as f64);
            let r = ((fi - mid).powi(2) + (fj - mid).powi(2)).sqrt();
            let h = 4.2
                + a1 * (fi * f1 + p1).sin() * (fj * f2 + p2).cos()
                + a2 * ((fi + fj) * f3 + p3).sin()
                + a3 * ((fi - fj) * f4 + p4).cos()
                - (r - 7.5).max(0.0) * 0.8;
            grid.push(h.round().max(0.0) as i32);
        }
    }
    grid
}

#[derive(Clone, PartialEq, Default)]
struct Cell {
    visible: bool,
    top: String,
    left: String,
    right: String,
    ft: String,
    fl: String,
    fr: String,
}

#[derive(Clone, PartialEq)]
struct Frame {
    /// Indexed by draw slot (back to front).
    cells: Vec<Cell>,
    glow: Option<(f64, f64)>,
}

/// Grid indices in painter's order: by diagonal, then row.
fn draw_order() -> Vec<(usize, usize)> {
    let mut out = Vec::with_capacity(N * N);
    for d in 0..=2 * (N - 1) {
        for i in 0..N {
            if let Some(j) = d.checked_sub(i).filter(|&j| j < N) {
                out.push((i, j));
            }
        }
    }
    out
}

fn mix(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

fn css(c: [f64; 3], k: f64) -> String {
    format!(
        "rgb({},{},{})",
        (c[0] * k).round(),
        (c[1] * k).round(),
        (c[2] * k).round()
    )
}

fn pts(p: [(f64, f64); 4]) -> String {
    p.iter()
        .map(|(x, y)| format!("{x:.1},{y:.1}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn compute(base: &[i32], order: &[(usize, usize)], tick: u32, ptr: (i32, i32), built: &[i32]) -> Frame {
    let mid = (N as f64 - 1.0) / 2.0;
    let ring = (f64::from(tick) * 0.45) % 17.0;
    let (px, py) = ptr;

    let mut hs = vec![0i32; N * N];
    let mut heats = vec![0f64; N * N];
    for i in 0..N {
        for j in 0..N {
            let k = i * N + j;
            let (fi, fj) = (i as f64, j as f64);
            let r = ((fi - mid).powi(2) + (fj - mid).powi(2)).sqrt();
            let b = base[k];
            let pulse = i32::from(b >= 1 && (r - ring).abs() < 1.2);
            let dp2 = (fi - f64::from(px)).powi(2) + (fj - f64::from(py)).powi(2);
            let bump = (7.0 * (-dp2 / 9.0).exp()).round() as i32;
            let placed = built[k];
            hs[k] = (b + pulse + bump + placed).min(MAXH);
            heats[k] = (f64::from(bump) / 7.0
                + f64::from(pulse) * 0.55
                + if placed > 0 { 0.35 } else { 0.0 })
            .min(1.0);
        }
    }
    let h_at = |i: isize, j: isize| -> f64 {
        if i < 0 || j < 0 || i >= N as isize || j >= N as isize {
            0.0
        } else {
            f64::from(hs[i as usize * N + j as usize])
        }
    };

    let cells = order
        .iter()
        .map(|&(ci, cj)| {
            let h = hs[ci * N + cj];
            if h < 1 {
                return Cell::default();
            }
            let heat = heats[ci * N + cj];
            let fh = f64::from(h);
            let t = (fh - 1.0) / f64::from(MAXH - 1);
            let top_c = mix(mix(LOW, HIGH, t), ACC, heat * 0.9);
            let (si, sj) = (ci as isize, cj as isize);
            let nb = (h_at(si - 1, sj) + h_at(si + 1, sj) + h_at(si, sj - 1) + h_at(si, sj + 1)) / 4.0;
            let occl = ((nb - fh) * 0.09).clamp(0.0, 0.3);
            let haze = 0.6 + 0.4 * ((ci + cj) as f64 / (2.0 * (N as f64 - 1.0)));
            let mut k = haze * (1.0 - occl);
            k += (1.0 - k) * heat * 0.85;
            let cx = OX + (ci as f64 - cj as f64) * HW;
            let cy = OY + (ci + cj) as f64 * HH - fh * ZH;
            let dd = fh * ZH;
            Cell {
                visible: true,
                top: pts([(cx, cy), (cx + HW, cy + HH), (cx, cy + TH), (cx - HW, cy + HH)]),
                left: pts([
                    (cx - HW, cy + HH),
                    (cx, cy + TH),
                    (cx, cy + TH + dd),
                    (cx - HW, cy + HH + dd),
                ]),
                right: pts([
                    (cx, cy + TH),
                    (cx + HW, cy + HH),
                    (cx + HW, cy + HH + dd),
                    (cx, cy + TH + dd),
                ]),
                ft: css(top_c, k),
                fl: css(top_c, k * 0.76),
                fr: css(top_c, k * 0.58),
            }
        })
        .collect();

    let n = N as i32;
    let glow = (px >= 0 && py >= 0 && px < n && py < n).then(|| {
        let (pi, pj) = (px as usize, py as usize);
        (
            OX + f64::from(px - py) * HW,
            OY + f64::from(px + py) * HH + HH - f64::from(hs[pi * N + pj]) * ZH,
        )
    });

    Frame { cells, glow }
}

#[island]
pub fn Terrain(seed: u32) -> impl IntoView {
    let base = StoredValue::new(make_terrain(seed));
    let order = StoredValue::new(draw_order());

    let (tick, set_tick) = signal(0u32);
    let (ptr, set_ptr) = signal(NO_PTR);
    let (built, set_built) = signal(vec![0i32; N * N]);
    let (touched, set_touched) = signal(false);

    let frame = Memo::new(move |_| {
        base.with_value(|b| {
            order.with_value(|o| built.with(|bl| compute(b, o, tick.get(), ptr.get(), bl)))
        })
    });

    // Client only: effects never run during SSR.
    Effect::new(move |_| {
        let reduce = window()
            .match_media("(prefers-reduced-motion: reduce)")
            .ok()
            .flatten()
            .is_some_and(|m| m.matches());
        if reduce {
            return;
        }
        if let Ok(handle) = set_interval_with_handle(
            move || set_tick.update(|t| *t = t.wrapping_add(1)),
            std::time::Duration::from_millis(140),
        ) {
            on_cleanup(move || handle.clear());
        }
    });

    let svg_ref = NodeRef::<leptos::svg::Svg>::new();
    let cell_at = move |e: &leptos::ev::MouseEvent| -> Option<(i32, i32)> {
        let el = svg_ref.get_untracked()?;
        let rc = el.get_bounding_client_rect();
        if rc.width() <= 0.0 || rc.height() <= 0.0 {
            return None;
        }
        let sx = VX + (f64::from(e.client_x()) - rc.left()) / rc.width() * VW;
        let sy = VY + (f64::from(e.client_y()) - rc.top()) / rc.height() * VH;
        let a = (sx - OX) / HW;
        let b = (sy - OY - HH + 5.0 * ZH) / HH;
        Some((((a + b) / 2.0).round() as i32, ((b - a) / 2.0).round() as i32))
    };

    let on_move = move |e: leptos::ev::MouseEvent| {
        if let Some(c) = cell_at(&e) {
            if c != ptr.get_untracked() {
                set_ptr.set(c);
            }
            if !touched.get_untracked() {
                set_touched.set(true);
            }
        }
    };
    let on_leave = move |_| set_ptr.set(NO_PTR);
    let on_build = move |e: leptos::ev::MouseEvent| {
        let Some((i, j)) = cell_at(&e) else { return };
        let n = N as i32;
        if i < 0 || j < 0 || i >= n || j >= n {
            return;
        }
        set_built.update(|b| {
            let k = i as usize * N + j as usize;
            b[k] = (b[k] + 1).min(4);
        });
        set_touched.set(true);
    };

    let cell = move |slot: usize| {
        let get = move |f: fn(&Cell) -> String| move || frame.with(|fr| f(&fr.cells[slot]));
        view! {
            <g style:display=move || {
                if frame.with(|f| f.cells[slot].visible) { "inline" } else { "none" }
            }>
                <polygon points=get(|c| c.left.clone()) fill=get(|c| c.fl.clone())></polygon>
                <polygon points=get(|c| c.right.clone()) fill=get(|c| c.fr.clone())></polygon>
                <polygon points=get(|c| c.top.clone()) fill=get(|c| c.ft.clone())></polygon>
            </g>
        }
    };

    view! {
        <figure class="terrain">
            <svg
                node_ref=svg_ref
                viewBox="0 90 700 460"
                aria-hidden="true"
                class="terrain-svg"
                on:mousemove=on_move
                on:mouseleave=on_leave
                on:click=on_build
            >
                <defs>
                    <radialGradient id="tfade" cx="50%" cy="58%" r="54%">
                        <stop offset="45%" stop-color="#0F1311" stop-opacity="0"></stop>
                        <stop offset="96%" stop-color="#0F1311" stop-opacity="1"></stop>
                    </radialGradient>
                    <radialGradient id="tglow">
                        <stop offset="0%" stop-color="#E8643A" stop-opacity="0.55"></stop>
                        <stop offset="100%" stop-color="#E8643A" stop-opacity="0"></stop>
                    </radialGradient>
                </defs>
                <g class="terrain-cells">{(0..N * N).map(cell).collect_view()}</g>
                <circle
                    cx=move || frame.with(|f| f.glow.map_or(-999.0, |g| g.0))
                    cy=move || frame.with(|f| f.glow.map_or(-999.0, |g| g.1))
                    r="80"
                    fill="url(#tglow)"
                    opacity=move || if frame.with(|f| f.glow.is_some()) { "0.5" } else { "0" }
                    pointer-events="none"
                ></circle>
                <rect x="0" y="90" width="700" height="460" fill="url(#tfade)" pointer-events="none"></rect>
            </svg>
            <figcaption class="terrain-caption mono" class:faded=touched>
                "move to raise · click to build"
            </figcaption>
        </figure>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terrain_is_deterministic_and_bounded() {
        let a = make_terrain(752_000);
        assert_eq!(a, make_terrain(752_000));
        assert_eq!(a.len(), N * N);
        assert!(a.iter().all(|&h| (0..=MAXH).contains(&h)));
        assert!(a.iter().any(|&h| h > 0));
    }

    #[test]
    fn draw_order_covers_grid() {
        let o = draw_order();
        assert_eq!(o.len(), N * N);
        assert_eq!(o[0], (0, 0));
        assert_eq!(o[N * N - 1], (N - 1, N - 1));
    }
}
