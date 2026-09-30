slint::include_modules!();

fn main() {
    let ui = App::new().unwrap();
    let weak = ui.as_weak();
    ui.on_clicked(move || {
        let ui = weak.upgrade().unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });
    ui.run().unwrap();
}
