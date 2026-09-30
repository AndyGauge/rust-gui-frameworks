use iced::time::{self, Duration};
use iced::widget::text;
use iced::{Element, Subscription};

struct Clock {
    ticks: u64,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
}

impl Clock {
    fn update(&mut self, _message: Message) {
        self.ticks += 1;
    }

    fn view(&self) -> Element<'_, Message> {
        text(format!("{} seconds", self.ticks)).into()
    }

    // Anything that happens outside the UI (timers, sockets, file watchers)
    // arrives as a Subscription that produces ordinary Messages.
    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}

fn main() -> iced::Result {
    iced::application(|| Clock { ticks: 0 }, Clock::update, Clock::view)
        .subscription(Clock::subscription)
        .run()
}
