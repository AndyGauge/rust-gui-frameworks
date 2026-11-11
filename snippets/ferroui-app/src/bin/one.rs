use book_error::Result;
use ferroui::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

// Avalonia's class hierarchy, carried into Rust: an App "derives" from
// Application through macros, and overrides virtual methods via a trait.
#[repr(C)]
pub struct App {
    base: Application,
}

ferro_class!(App: Application);
ferro_impl_classes!(App: FerroObjectImpl);

impl NewApplication for App {
    fn new_application() -> Ref<Self> {
        instantiate(Self { base: Application::construct() })
    }
}

impl ApplicationImpl for App {
    fn initialize(this: &Self) {
        Self::parent_initialize(this);
        this.styles().add(FluentTheme::new().as_style());
    }

    fn on_framework_initialization_completed(this: &Self) {
        let lifetime = this.application_lifetime();
        if let Some(desktop) = lifetime.as_ref().and_then(|l| l.as_classic_desktop_style_application_lifetime()) {
            desktop.set_main_window(Some(main_window()));
        }
        Self::parent_on_framework_initialization_completed(this);
    }
}

// Controls are retained objects with setters, events and a children
// collection, exactly as in C# Avalonia (or WPF before it).
fn main_window() -> Ref<Window> {
    let text = TextBlock::new();
    text.set_text(Some("Hello from FerroUI"));

    let clicks = Rc::new(Cell::new(0u32));
    let button = Button::new();
    button.set_content(Some(Rc::new("Click me".to_string())));
    button.click({
        let text = text.clone();
        move |_, _| {
            clicks.set(clicks.get() + 1);
            text.set_text(Some(format!("Clicked {} times", clicks.get()).as_str()));
        }
    });

    let panel = StackPanel::new();
    panel.set_spacing(12.0);
    panel.children().add(text);
    panel.children().add(button);

    let window = Window::new();
    window.set_title(Some("FerroUI".to_string()));
    window.set_content(Some(Control::boxed(panel)));
    window
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match AppBuilder::configure::<App>().use_platform_detect().start_with_classic_desktop_lifetime(&args) {
        0 => Ok(()),
        code => Err(std::io::Error::other(format!("exited with code {code}")).into()),
    }
}
