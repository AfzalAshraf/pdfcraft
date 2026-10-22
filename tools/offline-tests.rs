//! Dependency-free checks of actual production policies. This entry point intentionally
//! compiles with rustc alone, without resolving the Cargo workspace or accessing a registry.
//! It does NOT build the application, test egui integration, or certify printer output.
//!
//! From the repository root:
//! rustc --edition 2024 --test tools/offline-tests.rs -o /tmp/pdfcraft-offline-tests
//! /tmp/pdfcraft-offline-tests

#![forbid(unsafe_code)]

#[path = "../crates/ui-egui/src/print_destination.rs"]
mod destination;
#[cfg(not(target_arch = "wasm32"))]
#[path = "../crates/ui-egui/src/print_discovery.rs"]
mod discovery;
#[path = "../crates/ui-egui/src/autoscroll_gesture.rs"]
mod gesture;
#[path = "../crates/ui-egui/src/hand_pan.rs"]
mod hand_pan;
