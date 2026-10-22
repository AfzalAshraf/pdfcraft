//! Exercise the actual dependency-free production policies even on an offline machine:
//! `rustc --edition 2024 --test crates/ui-egui/tests/interaction_policies.rs -o policy-tests`
//! `./policy-tests`
//! This is not a replacement for the full UI/control and native printer tests.

#[path = "../src/print_destination.rs"]
mod destination;
#[cfg(not(target_arch = "wasm32"))]
#[path = "../src/print_discovery.rs"]
mod discovery;
#[path = "../src/hand_pan.rs"]
mod hand_pan;
