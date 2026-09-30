use makepad_widgets::*;

live_design! {
    use link::widgets::*;

    App = {{App}} {
        ui: <Root> {
            <Window> {
                body = <View> {
                    flow: Down, spacing: 20, align: { x: 0.5, y: 0.5 }
                    button1 = <Button> { text: "Click me" }
                    label1 = <Label> { text: "Clicked 0 times" }
                }
            }
        }
    }
}

app_main!(App);

fn main() {
    app_main()
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    counter: usize,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(button1)).clicked(actions) {
            self.counter += 1;
            self.ui.label(id!(label1)).set_text(cx, &format!("Clicked {} times", self.counter));
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
