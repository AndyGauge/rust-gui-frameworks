use book_error::Result;

slint::slint! {
    export component App inherits Window {
        callback save(string);
        in-out property <string> status: "idle";
        TextInput { text: "draft"; accepted => { root.save(self.text); } }
        Text { y: 40px; text: root.status; }
    }
}

fn main() -> Result<()> {
    let ui = App::new()?;
    let weak = ui.as_weak();
    // The .slint side declares callbacks and properties; Rust supplies behaviour.
    ui.on_save(move |text| {
        // The window may already be gone when this fires; that is not an error.
        if let Some(ui) = weak.upgrade() {
            ui.set_status(format!("saved {} bytes", text.len()).into());
        }
    });
    ui.run()?;
    Ok(())
}
