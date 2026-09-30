use book_error::Result;
use semver::{Version, VersionReq};

fn main() -> Result<()> {
    // What `winit = "0.30"` in Cargo.toml actually means:
    let req = VersionReq::parse("0.30")?;
    for v in ["0.30.13", "0.31.0-beta.3", "1.0.0"] {
        let ok = req.matches(&Version::parse(v)?);
        println!("{v:>14} satisfies ^0.30? {ok}");
    }
    Ok(())
}
