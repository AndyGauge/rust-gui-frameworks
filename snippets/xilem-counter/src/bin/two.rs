use xilem::core::lens;
use xilem::view::{flex_row, label, text_button};
use xilem::winit::error::EventLoopError;
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem};

#[derive(Default)]
struct AppState {
    likes: u32,
    dislikes: u32,
}

// A reusable component that only knows about the slice of state it edits.
fn votes(n: &mut u32) -> impl WidgetView<u32> + use<> {
    flex_row((label(format!("votes: {n}")), text_button("+", |n: &mut u32| *n += 1)))
}

// `lens` focuses the component on one field, like Druid's Lens did.
fn app_logic(_state: &mut AppState) -> impl WidgetView<AppState> + use<> {
    flex_row((
        lens(votes, |s: &mut AppState| &mut s.likes),
        lens(votes, |s: &mut AppState| &mut s.dislikes),
    ))
}

fn main() -> Result<(), EventLoopError> {
    Xilem::new_simple(AppState::default(), app_logic, WindowOptions::new("Votes"))
        .run_in(EventLoop::with_user_event())?;
    Ok(())
}
