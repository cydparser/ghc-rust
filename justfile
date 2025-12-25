default:
    @just --list --justfile {{justfile()}}

generate-symbols:
    cargo run -p generate-symbol-consumers
    treefmt -q generate/symbols/src/lib.rs

generate-headers:
    cargo run -p generate-headers
    treefmt -q include/*.h

generate-includes:
    scripts/locate-ghc-rts-header-includes > generate/includes.json

regenerate *args:
    scripts/regenerate {{ args }}

test: test-sys && test-rts

test-sys:
    cargo test -p ghc-rts-sys

test-rts: test-rts-rust && test-rts-sys

test-rts-rust:
    cargo test -p ghc-rts --no-default-features --features tracing

test-rts-sys:
    cargo test -p ghc-rts --no-default-features --features sys,tracing
