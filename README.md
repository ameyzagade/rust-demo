h2 Experimenting with Rust

h3 Intro
h4 Rust focuses on 
    ul
        li Fearless concurrency
        li Speed
        li Type safety (via 0 cost abstractions).
h4 Extension: .rs
h4 Compiler: rustc
h4 Compilation time higher, lot of checks during compile time
h4 Compiled binary location: target/[release/debug] directory

h3 Cargo
h4 Cargo is a
    ul
        li package (crate) manager for Rust applications
        li built-in build system
        li test-runner
        li docs generator
h4 Check cargo version: cargo --version
h4 Create a project: cargo new <name-of-the-project> OR cargo init .
h4 Running a project: cargo run [--release]
h4 Updating installed Rust crates: rustup update

h3 Tom's Obvious Minimal Language (TOML)
h4: Cargo.toml contains info 
    ul
        li author
        li version (semver)
        li dependencies

