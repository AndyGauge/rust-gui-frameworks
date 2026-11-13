use rune::AppSpec;
use rune_hello::{State, view};

// `rune::run` owns startup errors: it reports them and exits, so main has nothing to return.
fn main() {
    rune::run(AppSpec::<State> {
        scenes: &[("on", |s| s.on = true)],
        ..AppSpec::new("Hello", (320.0, 200.0), State::default, view)
    });
}
