## Experimenting with Rust

### Intro
#### Rust focuses on 
    - Fearless concurrency
    - Speed
    - Type safety (via 0 cost abstractions).
#### Extension: .rs
#### Compiler: rustc
#### Compilation time higher, lot of checks during compile time
#### Compiled binary location: target/[release/debug] directory

### Cargo
#### Cargo is a
    - package (crate) manager for Rust applications
    - built-in build system
    - test-runner
    - docs generator
#### Check cargo version: cargo --version
#### Create a project: cargo new <name-of-the-project> OR cargo init .
#### Running a project: cargo run [--release]
#### Updating installed Rust crates: rustup update

### Tom's Obvious Minimal Language (TOML)
#### Cargo.toml contains info 
    - author
    - version (semver)
    - dependencies

