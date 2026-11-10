use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::content::{HANDLE, TAGLINE};
use crate::islands::terrain::Terrain;

const DOORS: &[(&str, &str, &str)] = &[
    ("01", "Projects", "/projects"),
    ("02", "Blog", "/blog"),
    ("03", "Tools", "/tools"),
];

/// Seed for the terrain: the server's date, so every visitor sees the same
/// world on a given day.
fn day_seed() -> u32 {
    #[cfg(feature = "ssr")]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let (y, m, d) = civil_from_days((secs / 86_400) as i64);
        (y as u32) * 372 + (m - 1) * 31 + d
    }
    #[cfg(not(feature = "ssr"))]
    {
        0
    }
}

/// Days since 1970-01-01 to (year, month 1..=12, day 1..=31). Howard Hinnant's algorithm.
#[cfg(feature = "ssr")]
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = yoe + era * 400 + i64::from(m <= 2);
    (y, m, d)
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text=HANDLE/>
        <Meta name="description" content=TAGLINE/>
        <div class="home">
            <main class="home-main">
                <section class="hero">
                    <div class="hero-text">
                        <h1 class="h-hero">{HANDLE}</h1>
                        <p class="tagline">{TAGLINE}</p>
                    </div>
                    <Terrain seed=day_seed()/>
                </section>
                <nav aria-label="Sections" class="doors">
                    {DOORS
                        .iter()
                        .map(|&(no, title, href)| {
                            view! {
                                <a class="door" href=href>
                                    <span class="mono muted door-no">{no}</span>
                                    <span class="door-title">{title}</span>
                                </a>
                            }
                        })
                        .collect_view()}
                </nav>
            </main>
            <footer class="home-footer">
                <a href="/who">"Who am I? →"</a>
            </footer>
        </div>
    }
}
