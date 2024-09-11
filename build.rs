#[cfg(feature = "docs")]
fn main() {
    std::fs::copy("docs/logo.png", "target/doc/logo.png")
        .expect("Failed to copy crate logo when building documentation.");
}

#[cfg(not(feature = "docs"))]
fn main() {}
