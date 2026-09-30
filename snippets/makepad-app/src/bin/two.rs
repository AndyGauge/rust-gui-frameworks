use makepad_widgets::*;

// Makepad widgets are drawn by shaders you can write inline. This is a GPU
// fragment shader, in Makepad's own shading language, styling a plain View.
live_design! {
    use link::widgets::*;

    App = {{App}} {
        ui: <Root> {
            <Window> {
                body = <View> {
                    show_bg: true
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            return mix(#e5484d, #30a46c, self.pos.x);
                        }
                    }
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
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
