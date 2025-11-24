default:
    @just --list --justfile {{justfile()}}

generate-symbols:
    cargo run -p generate-symbol-consumers
    rustfmt generate/symbols/src/lib.rs
