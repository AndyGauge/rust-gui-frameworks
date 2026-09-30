slint::slint! {
    export component App inherits Window {
        callback save(string);
        in-out property <string> status: "idle";
        TextInput { text: "draft"; accepted => { root.save(self.text); } }
        Text { y: 40px; text: root.status; }
    }
}

fn main() {
    let ui = App::new().unwrap();
    let weak = ui.as_weak();
    // The .slint side declares callbacks and properties; Rust supplies behaviour.
    ui.on_save(move |text| {
        let ui = weak.upgrade().unwrap();
        ui.set_status(format!("saved {} bytes", text.len()).into());
    });
    ui.run().unwrap();
}
