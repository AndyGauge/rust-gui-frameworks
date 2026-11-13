use book_error::{Context, Result};
use rune::{Env, Runtime};
use rune_hello::{State, view};

// No window and no GPU: the same app, driven the way an agent checks its work.
fn main() -> Result<()> {
    let env = Env { system_dark: true, scale: 2.0 };
    let mut rt = Runtime::new(State::default(), view, (320.0, 200.0), env);
    rt.frame(0.0);
    let knob_x = |rt: &Runtime<State>| rt.presented("knob").map(|(rect, _)| rect[0]).context("no knob on screen");
    let off = knob_x(&rt)?;

    let (x, y) = rt.find("switch").context("no switch on screen")?;
    rt.pointer_down(x, y);
    rt.pointer_up(x, y);
    for _ in 0..6 { rt.frame(1.0 / 60.0); }
    let mid = knob_x(&rt)?;
    for _ in 0..60 { rt.frame(1.0 / 60.0); }
    let on = knob_x(&rt)?;

    println!("on: {}", rt.state.on);
    println!("knob x: {off:.2} -> {mid:.2} (6 frames) -> {on:.2} (66 frames)");
    Ok(())
}
