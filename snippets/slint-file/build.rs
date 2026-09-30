fn main() {
    // Compiles ui/app.slint to Rust at build time.
    slint_build::compile("ui/app.slint").unwrap();
}
