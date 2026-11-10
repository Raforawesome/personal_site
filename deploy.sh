#!/usr/bin/env bash
# Build and serve the site in release mode. Safe to re-run: every step is
# skipped when it's already done.
#
#   ./deploy.sh                 build what's missing, then serve
#   ./deploy.sh --rebuild-demo  also pull and rebuild the voxel engine demo
#   ./deploy.sh --build-only    build but don't serve (e.g. when systemd runs the binary)
#
# Environment (all optional):
#   SITE_ADDR     address to listen on (default 127.0.0.1:3000; put a reverse proxy in front)
#   BETWEEN_DIR   voxel engine checkout (default ../between-wgpu, cloned if missing)
#   BETWEEN_REPO  where to clone it from
#   SLANGC        path to an existing slangc (otherwise found on PATH or downloaded)
#
# Needs rustup, git and curl. Everything else (nightly Rust, the wasm target,
# cargo-leptos, wasm-bindgen-cli, Slang) is installed on first run.
set -euo pipefail

cd "$(dirname "$0")"
SITE_DIR=$PWD
BETWEEN_DIR=${BETWEEN_DIR:-$SITE_DIR/../between-wgpu}
BETWEEN_REPO=${BETWEEN_REPO:-https://github.com/Raforawesome/between-wgpu.git}
SITE_ADDR=${SITE_ADDR:-127.0.0.1:3000}
DEMO_DIR=$SITE_DIR/public/demo/voxel
TOOLS_DIR=$SITE_DIR/.tools          # downloaded tools; gitignored
SLANG_VERSION=2026.5.2              # what between-wgpu's README pins

REBUILD_DEMO=false
SERVE=true
for arg in "$@"; do
  case "$arg" in
    --rebuild-demo) REBUILD_DEMO=true ;;
    --build-only) SERVE=false ;;
    -h|--help) awk 'NR > 1 && !/^#/ { exit } NR > 1 { sub(/^# ?/, ""); print }' "$0"; exit 0 ;;
    *) echo "unknown option: $arg (try --help)" >&2; exit 1 ;;
  esac
done

step() { printf '\n==> %s\n' "$*"; }
die() { printf 'error: %s\n' "$*" >&2; exit 1; }
for cmd in rustup git curl; do
  command -v "$cmd" >/dev/null || die "$cmd is required"
done

# --- Toolchain --------------------------------------------------------------
# The engine needs nightly (rapidhash's `nightly` feature); the site builds on
# it too, so use one toolchain for everything.
step "Rust nightly + wasm target"
# Install nightly only if it's missing: `rustup toolchain install` would also
# update an existing nightly, which means surprise rebuilds (or breakage).
# Run `rustup update nightly` yourself when you want a newer one.
if ! rustup run nightly rustc --version >/dev/null 2>&1; then
  rustup toolchain install nightly --profile minimal
fi
rustup target add --toolchain nightly wasm32-unknown-unknown
export RUSTUP_TOOLCHAIN=nightly
# Keep this machine's home directory out of the compiled binaries.
export RUSTFLAGS="--remap-path-prefix=$HOME=~"

if ! command -v cargo-leptos >/dev/null; then
  step "Installing cargo-leptos (one-time, takes a few minutes)"
  cargo install cargo-leptos --locked
fi

# --- Voxel engine demo -------------------------------------------------------
ensure_slangc() {
  if [[ -n "${SLANGC:-}" ]]; then return; fi
  if command -v slangc >/dev/null; then export SLANGC=$(command -v slangc); return; fi

  local dir=$TOOLS_DIR/slang-$SLANG_VERSION
  if [[ ! -x $dir/bin/slangc ]]; then
    local os arch
    case "$(uname -s)" in Linux) os=linux ;; Darwin) os=macos ;; *) die "no Slang build for $(uname -s)" ;; esac
    case "$(uname -m)" in x86_64|amd64) arch=x86_64 ;; aarch64|arm64) arch=aarch64 ;; *) die "no Slang build for $(uname -m)" ;; esac
    local url=https://github.com/shader-slang/slang/releases/download/v$SLANG_VERSION/slang-$SLANG_VERSION-$os-$arch.tar.gz
    step "Downloading Slang $SLANG_VERSION"
    mkdir -p "$dir"
    curl -fL --progress-bar "$url" | tar -xz -C "$dir"
    [[ -x $dir/bin/slangc ]] || die "slangc not found in $url"
  fi
  export SLANGC=$dir/bin/slangc
}

ensure_wasm_bindgen() {
  # The CLI must match the wasm-bindgen version the engine's web crate locks.
  local want have=""
  want=$(awk '$0 == "name = \"wasm-bindgen\"" { getline; gsub(/[^0-9.]/, ""); print; exit }' \
    "$BETWEEN_DIR/web/Cargo.lock")
  [[ -n $want ]] || die "couldn't read wasm-bindgen's version from web/Cargo.lock"
  if command -v wasm-bindgen >/dev/null; then have=$(wasm-bindgen --version | awk '{print $2}'); fi
  if [[ $have != "$want" ]]; then
    step "Installing wasm-bindgen-cli $want"
    cargo install wasm-bindgen-cli --version "$want" --locked
  fi
}

if $REBUILD_DEMO || [[ ! -f $DEMO_DIR/between_web_bg.wasm ]]; then
  if [[ -d $BETWEEN_DIR/.git ]]; then
    step "Updating voxel engine in $BETWEEN_DIR"
    git -C "$BETWEEN_DIR" pull --ff-only
  else
    step "Cloning voxel engine into $BETWEEN_DIR"
    git clone "$BETWEEN_REPO" "$BETWEEN_DIR"
  fi
  [[ -x $BETWEEN_DIR/web/build.sh ]] || die "$BETWEEN_DIR has no web/build.sh (is the web build pushed?)"

  ensure_slangc
  ensure_wasm_bindgen
  step "Building voxel engine demo"
  "$BETWEEN_DIR/web/build.sh" "$DEMO_DIR"
else
  step "Voxel engine demo already built (use --rebuild-demo to rebuild)"
fi

# --- Site ------------------------------------------------------------------
step "Building site (release)"
cargo leptos build --release

if ! $SERVE; then
  step "Built. Start it with ./target/release/personal_site (from this directory)"
  exit 0
fi

step "Serving on http://$SITE_ADDR"
# Run from the repo root so the default site root (target/site) resolves.
LEPTOS_SITE_ADDR=$SITE_ADDR exec ./target/release/personal_site
