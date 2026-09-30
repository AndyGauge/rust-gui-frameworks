use book_error::Result;
use xilem::view::{flex_col, label, text_button};
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem};

struct AppState {
    count: i32,
}

// A pure function from state to a view description. Xilem diffs the
// descriptions and updates the retained widgets (Masonry) underneath.
fn app_logic(state: &mut AppState) -> impl WidgetView<AppState> + use<> {
    flex_col((
        label(format!("count: {}", state.count)),
        text_button("increment", |state: &mut AppState| state.count += 1),
    ))
}

fn main() -> Result<()> {
    let app = Xilem::new_simple(AppState { count: 0 }, app_logic, WindowOptions::new("Counter"));
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}
