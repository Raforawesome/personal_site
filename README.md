# personal_site

Leptos 0.8 in islands mode, served by Axum. Every page is server-rendered
static HTML; only two islands hydrate:

- `src/islands/terrain.rs`: the voxel terrain on Home (seeded by the server's date)
- `src/islands/run_demo.rs`: "Run it in your browser" on the voxel engine page

Content lives in `src/content.rs`; `[BRACKETS]` mark text still to be written.

## Develop

```bash
cargo leptos watch
```

## Deploy

```bash
cargo leptos build --release
```

Copy `target/release/personal_site` and the `target/site/` directory to the
server, then run the binary with:

```bash
LEPTOS_OUTPUT_NAME=personal_site \
LEPTOS_SITE_ROOT=site \
LEPTOS_SITE_PKG_DIR=pkg \
LEPTOS_SITE_ADDR=127.0.0.1:3000 \
LEPTOS_ENV=PROD \
./personal_site
```

Put it behind your reverse proxy (Caddy/nginx) for TLS. Responses are
compressed (br/gzip) by the app itself.

## Still to supply

- `public/resume.pdf` (linked from /who)
- The engine's web build in `public/demo/voxel/` (loaded in an iframe only
  when the visitor presses the button)
