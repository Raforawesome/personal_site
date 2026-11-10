// Public copy of the résumé, served at /resume.pdf. Rebuild with:
//   typst compile resume/resume.typ public/resume.pdf
// Only details that are already on the site belong here (no phone number).
#import "@preview/basic-resume:0.2.8": *

#let name = "Tahir Chaudhry"
#let location = "Waterloo, ON"
#let email = "raforawesome@raforaweso.me"
#let github = "github.com/Raforawesome"
#let linkedin = "linkedin.com/in/tchaudhry-dev"
#let personal-site = "raforaweso.me"

#show: resume.with(
  author: name,
  // location: location, // removed so the contact line fits on one line with LinkedIn
  email: email,
  github: github,
  linkedin: linkedin,
  personal-site: personal-site,
  accent-color: "#02367b",
  font: "New Computer Modern",
  paper: "us-letter",
  author-position: left,
  personal-info-position: left,
)

== Education

#edu(
  institution: "Wilfrid Laurier University",
  location: "Waterloo, ON",
  dates: dates-helper(
    start-date: "Sep 2023",
    end-date: "Dec 2027 (Expected)",
  ),
  degree: "Bachelor of Science, Computer Science",
)
- Science Faculty Honour Roll | Relevant Coursework: Algorithm Design, Software Development, AI Algorithms

== Selected Projects

#project(
  name: "Voxel Engine | Rust, wgpu, Slang",
  url: "x.com/raforawesome/status/2114935770566062370",
  dates: dates-helper(start-date: "Jan 2026", end-date: "Present"),
)
- Renders procedural chunk-based terrain across a 32×12×32-chunk view at \~240 FPS (4.0 ms median frame time) at 3000×1800 on an Apple M2 integrated GPU
- Built a lock-free generation and meshing pipeline (crossbeam work queue, immutable neighbour references) that scales 4.2× on 4 threads, builds the full 12k-chunk view in 78 ms, and keeps p99 frame time under 10 ms while streaming new terrain
- Cut drawn triangles 77% and raised median FPS 16% with per-chunk frustum culling (8.5 ns per test), on top of neighbour-aware face culling that removes 99.5% of faces
- Cut per-chunk terrain generation cost 9.4× (185.6 → 19.7 µs) by sampling a coarse 5³ noise grid with trilinear interpolation, altering only 0.11% of voxels
- Built an O(1) LRU GPU mesh cache under a 64 MiB VRAM budget (47× faster than a linear-scan LRU) and a headless benchmark suite for frame-time percentiles and thread scaling

#project(
  name: "Sprout | Rust, Dioxus, GitHub Actions",
  dates: dates-helper(start-date: "Apr 2024", end-date: "Present"),
)
- Built a desktop mod manager for Stardew Valley entirely in Rust, with a React-style, component-based Dioxus + Tailwind frontend (hooks, signals, reusable UI components)
- Released to and used by a Discord community of non-technical players
- Automated testing and release builds with a GitHub Actions CI/CD pipeline

#project(
  name: "Silk | Rust",
  dates: dates-helper(start-date: "Dec 2025", end-date: "Present"),
)
- Building a zero-copy HTTP request parser that borrows from the input buffer and performs no heap allocation on the hot path, using memchr for vectorized delimiter search
- Built a custom thread pool for fine-grained control over request scheduling

== Infrastructure

#work(
  title: "Homelab & Self-Hosted Infrastructure",
  company: "Personal Project",
  dates: dates-helper(start-date: "Jan 2021", end-date: "Present"),
)
- Built and operate a Docker-based media and file-sharing platform for 20+ users across Google Cloud, Oracle Cloud, and self-hosted Linux servers
- Automated provisioning and maintenance: scheduled backups, container health monitoring, log rotation, and alerting
- Configured a Caddy reverse proxy, custom DNS routing, and firewall rules to expose public services while isolating internal ones

== Awards & Activities

- *1st Place*, Laurier Cybersecurity Club CTF — won with a team of 2 in a 4-person-team competition

== Technical Skills

- *Languages*: Rust, C, C++, Python, Java, JavaScript, TypeScript, Bash, Slang
- *Systems & Infrastructure*: Linux (Arch, Fedora, RHEL, Debian, Ubuntu), Docker, Git, GCP, Oracle Cloud, Caddy, GitHub Actions
- *Frameworks & Libraries*: wgpu, crossbeam, Axum, Dioxus (React-like), Tailwind CSS
