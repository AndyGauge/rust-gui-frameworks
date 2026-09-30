use book_error::{Context, Result};

// Each layer adds what *it* was trying to do; the original cause is kept.
fn read_config(path: &str) -> Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

fn start_app(path: &str) -> Result<()> {
    let _config = read_config(path).context(format!("starting app with config {path}"))?;
    Ok(())
}

fn main() {
    // Only the outermost layer decides what to do with a failure.
    if let Err(err) = start_app("/nonexistent/app.toml") {
        eprintln!("error: {}", err.report());
    }
}
