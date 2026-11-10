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

On the server (needs `rustup`, `git` and `curl`):

```bash
git clone https://github.com/Raforawesome/personal_site.git
cd personal_site
./deploy.sh
```

`deploy.sh` installs what's missing (nightly Rust, the wasm target,
cargo-leptos, wasm-bindgen-cli, Slang), clones
[between-wgpu](https://github.com/Raforawesome/between-wgpu) next to the site
and builds the voxel engine demo if it isn't built yet, builds the site in
release mode, then serves it on `127.0.0.1:3000` (`SITE_ADDR` changes that).
The first run takes a while; later runs only rebuild what changed. Pass
`--rebuild-demo` to pull and rebuild the engine demo. Put it behind your
reverse proxy (Caddy/nginx) for TLS; the app compresses responses itself.

To keep it running across reboots, let systemd run the built binary and use
`./deploy.sh --build-only` for updates:

```ini
# /etc/systemd/system/personal-site.service
[Service]
WorkingDirectory=/path/to/personal_site
ExecStart=/path/to/personal_site/target/release/personal_site
Environment=LEPTOS_SITE_ADDR=127.0.0.1:3000
User=you
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

```bash
git pull && ./deploy.sh --build-only && sudo systemctl restart personal-site
```

## Voxel engine demo

`public/demo/voxel/` holds the WebGPU build of the engine. It's a build output,
so it isn't in git; `deploy.sh` builds it. It loads in an iframe only when the
visitor presses the button. To build it by hand:

```bash
../between-wgpu/web/build.sh "$PWD/public/demo/voxel"
```

## Résumé

`public/resume.pdf` is built from `resume/resume.typ`, a public copy with only
details that are already on the site:

```bash
typst compile resume/resume.typ public/resume.pdf
```
