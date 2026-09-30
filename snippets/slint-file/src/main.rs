use book_error::Result;

slint::include_modules!();

fn main() -> Result<()> {
    let ui = App::new()?;
    let weak = ui.as_weak();
    ui.on_clicked(move || {
        if let Some(ui) = weak.upgrade() {
            ui.set_counter(ui.get_counter() + 1);
        }
    });
    ui.run()?;
    Ok(())
}
