use semver::{Version, VersionReq};

fn main() {
    let v = Version::parse("0.30.13").unwrap();
    for req in ["0.30", "~0.30.1", "=0.30.13", ">=0.30, <0.32"] {
        let ok = VersionReq::parse(req).unwrap().matches(&v);
        println!("{req:>14} accepts 0.30.13? {ok}");
    }
    // Bumping to 0.31 is a *breaking* change under cargo's rules for 0.x crates.
    let next = Version::parse("0.31.0").unwrap();
    println!("^0.30 accepts 0.31.0? {}", VersionReq::parse("0.30").unwrap().matches(&next));
}
