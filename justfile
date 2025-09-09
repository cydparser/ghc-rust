default:
    @just --list --justfile {{justfile()}}

generate-symbols:
    cargo run -p scripts --bin classify-symbols > generate/src/symbols.rs
    rustfmt generate/src/symbols.rs
