use semver::{Version, VersionReq};

fn main() {
    // What `winit = "0.30"` in Cargo.toml actually means:
    let req = VersionReq::parse("0.30").unwrap();
    for v in ["0.30.13", "0.31.0-beta.3", "1.0.0"] {
        let ok = req.matches(&Version::parse(v).unwrap());
        println!("{v:>14} satisfies ^0.30? {ok}");
    }
}
