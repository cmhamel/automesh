fn main() {
    println!("cargo:rerun-if-changed=assets/doc");
    std::fs::copy("docs/logo.png", "target/doc/logo.png")
        .expect("Failed to copy crate logo when building documentation.");
}
