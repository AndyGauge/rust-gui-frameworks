use book_error::Result;

slint::slint! {
    import { Button, VerticalBox } from "std-widgets.slint";

    export component App inherits Window {
        in-out property <int> counter: 0;
        VerticalBox {
            Text { text: @tr("Clicked {0} times", root.counter); }
            Button {
                text: "Click me";
                clicked => { root.counter += 1; }
            }
        }
    }
}

fn main() -> Result<()> {
    // Both creating the window and running the event loop can fail
    // (for example with no display available), and both use `?`.
    App::new()?.run()?;
    Ok(())
}
