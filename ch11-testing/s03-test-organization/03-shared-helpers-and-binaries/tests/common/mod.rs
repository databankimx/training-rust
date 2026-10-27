// tests/common/mod.rs is a module shared by the integration tests. Being in a subdirectory,
// cargo does not treat it as a test crate, so it gets no section of its own in the output.
pub fn setup() -> &'static str {
    "setup done"
}
