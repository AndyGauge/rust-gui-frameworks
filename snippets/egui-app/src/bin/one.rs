use book_error::Result;
use eframe::egui;

#[derive(Default)]
struct MyApp {
    name: String,
    clicks: u32,
}

impl eframe::App for MyApp {
    // Immediate mode: this runs every frame and *describes* the UI.
    // There is no widget tree to keep in sync with your state.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.text_edit_singleline(&mut self.name);
        if ui.button("Click me").clicked() {
            self.clicks += 1;
        }
        ui.label(format!("{} clicked {} times", self.name, self.clicks));
    }
}

fn main() -> Result<()> {
    eframe::run_native(
        "egui demo",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )?;
    Ok(())
}
