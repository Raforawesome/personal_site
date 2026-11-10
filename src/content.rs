//! Site content. Everything here renders to static HTML on the server.
//! `[BRACKETS]` mark content that still needs to be supplied.

pub const HANDLE: &str = "Raforawesome";
pub const TAGLINE: &str = "Renderers, parsers, and other machines.";
pub const GITHUB: &str = "https://github.com/Raforawesome";

pub struct Project {
    /// Projects with a slug get a detail page at `/projects/{slug}`.
    pub slug: Option<&'static str>,
    pub stack: &'static str,
    pub status: &'static str,
    pub title: &'static str,
    pub blurb: &'static str,
}

pub const PROJECTS: &[Project] = &[
    Project {
        slug: Some("voxel-engine"),
        stack: "Rust · wgpu",
        status: "v0.1 shipped",
        title: "Voxel engine",
        blurb: "Multithreaded voxel renderer. Chunk generation, meshing and streaming, with the profiling to back it up.",
    },
    Project {
        slug: None,
        stack: "Rust · Dioxus",
        status: "In use",
        title: "Sprout",
        blurb: "Mod manager for Stardew Valley, used by a small Discord community of non-technical players.",
    },
    Project {
        slug: None,
        stack: "Rust",
        status: "Early",
        title: "Silk",
        blurb: "An HTTP parser. Early, and being fleshed out.",
    },
    Project {
        slug: None,
        stack: "Rust",
        status: "[Status]",
        title: "Ferrum Automata",
        blurb: "Finite-automata toolkit: subset construction and Hopcroft minimization.",
    },
];

pub struct Post {
    pub slug: &'static str,
    pub date: &'static str,
    pub read: &'static str,
    pub tags: &'static str,
    pub title: &'static str,
    pub blurb: &'static str,
}

pub const POSTS: &[Post] = &[Post {
    slug: "voxel-engine-v0-1",
    date: "[Date]",
    read: "[N] min read",
    tags: "rust, wgpu",
    title: "Voxel engine v0.1 writeup",
    blurb: "How chunks are generated, meshed and streamed, and what each stage costs.",
}];

pub struct Tool {
    pub href: Option<&'static str>,
    pub kind: &'static str,
    pub size: &'static str,
    pub title: &'static str,
    pub blurb: &'static str,
}

pub const TOOLS: &[Tool] = &[
    Tool {
        href: None,
        kind: "Runs in browser",
        size: "[size] KB",
        title: "[Tool name]",
        blurb: "[What it does, in one line.]",
    },
    Tool {
        href: None,
        kind: "Runs in browser",
        size: "[size] KB",
        title: "[Tool name]",
        blurb: "[What it does, in one line.]",
    },
];

pub struct ContactLink {
    pub label: &'static str,
    pub text: &'static str,
    pub href: &'static str,
}

pub const CONTACT: &[ContactLink] = &[
    ContactLink {
        label: "Résumé",
        text: "PDF →",
        href: "/resume.pdf",
    },
    ContactLink {
        label: "Email",
        text: "raforawesome@raforaweso.me →",
        href: "mailto:raforawesome@raforaweso.me",
    },
    ContactLink {
        label: "GitHub",
        text: "Raforawesome →",
        href: GITHUB,
    },
    ContactLink {
        label: "LinkedIn",
        text: "linkedin.com/in/tchaudhry-dev →",
        href: "https://linkedin.com/in/tchaudhry-dev",
    },
];
