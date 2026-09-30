// Build scripts conventionally return a boxed error: Cargo prints it and fails the build.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compiles ui/app.slint to Rust at build time.
    slint_build::compile("ui/app.slint")?;
    Ok(())
}
