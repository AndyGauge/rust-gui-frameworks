Every other framework in this book builds a Rust UI toolkit. In 2026 two companies that already **own** major UI toolkits started offering them to Rust directly. Neither is a Rust GUI framework in the usual sense. Each is a bridge, and they compete with the rest of the book for the same apps.

**Microsoft** added `windows-reactor` to the windows-rs repository, first published on 27 April 2026. It builds WinUI 3 apps from Rust components that use typed messages, which will be familiar from iced. It is Windows-only and labelled a preview.

::inline rust | label=windows-reactor README (illustrative, not compiled; Windows only)
impl Component for Counter {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Increment => self.count += 1,
            Message::Reset => self.count = 0,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let content = StackPanel::new().spacing(8.0).children((
            format!("Count: {}", self.count),
            Button::new().on_click(context.message(Message::Increment)).content("Increment"),
        ));
        context.window_frame("Counter", content)
    }
}
::end

**Qt** ships `qtbridge`, which has been on crates.io since August 2025 and went to public beta with Qt's 1 July 2026 announcement. It is the mirror image of windows-reactor. The UI is written in QML, and Rust supplies the logic: structs marked `#[qobject]` become QML types, and their `#[qslot]` methods can be called from QML. Building it currently needs a C++ toolchain and `qmake` on the PATH.

::inline rust | label=Qt's hello world, Rust side (illustrative, not compiled)
#[derive(Default)]
pub struct Backend {}

#[qobject]
impl Backend {
    #[qslot]
    fn say_hello(&self) {
        println!("Hello World!");
    }
}
::end

::inline qml | label=and the QML that calls it
ApplicationWindow {
    visible: true
    Backend { id: backend }
    Button {
        anchors.centerIn: parent
        text: "Hello World!"
        onClicked: backend.say_hello()
    }
}
::end

Both bridges skip every shared layer in this book. There is no winit, no wgpu, no Taffy and no AccessKit, because the host toolkit already does windowing, rendering, layout and accessibility, and has for decades. That is the case for them: mature accessibility and native look today, from a vendor with a support contract. The case against is the lock-in a native Rust stack avoids, either to one OS (WinUI) or to a C++ toolkit and its licence (Qt).
