default:
    @just --list --justfile {{justfile()}}

generate-symbols:
    cargo run -p scripts --bin classify-symbols > symbols.rs
    mv symbols.rs generate/src/symbols.rs
    rustfmt generate/src/symbols.rs
