use book_error::Result;
use iced::widget::{button, column, text, Column};

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    // The Elm loop: a message comes in, state changes...
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    // ...and the view is rebuilt from state. Widgets emit messages, never mutate.
    fn view(&self) -> Column<'_, Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value).size(40),
            button("-").on_press(Message::Decrement),
        ]
    }
}

fn main() -> Result<()> {
    iced::run(Counter::update, Counter::view)?;
    Ok(())
}
