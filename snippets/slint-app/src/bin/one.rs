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

fn main() {
    App::new().unwrap().run().unwrap();
}
