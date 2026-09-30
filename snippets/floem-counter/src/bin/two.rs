use floem::prelude::*;

fn main() {
    floem::launch(view);
}

// The view tree is built once. Only the closures re-run, and only when a
// signal they read changes, so updates never rebuild the tree.
fn view() -> impl IntoView {
    let count = RwSignal::new(0);
    let doubled = move || count.get() * 2;

    v_stack((
        label(move || format!("count = {}", count.get())),
        label(move || format!("doubled = {}", doubled())),
        button("+1").action(move || count.update(|n| *n += 1)),
    ))
    .style(|s| s.padding(20).gap(8))
}
