// Thin wrapper to reuse the original CLI implementation that remains in the core crate.
// Keeping the code in one place preserves history.
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../core/src/main.rs"));
