use iced::widget::{button, column, text, Column};
use iced::Theme;


// Structs
#[derive(Default)]
struct Calculator {
    value: i32,
}

// Enums
#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement
}


// Impls
impl Calculator {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }
    
    fn view(&self) -> Column<Message> {
        let increment_button = button("+").on_press(Message::Increment);
        let decrement_button = button("-").on_press(Message::Decrement);
        let counter = text!("{}", self.value); 

        column![increment_button, counter, decrement_button]
    }
}

// Main
fn main() -> iced::Result {
    iced::application("The Count", Calculator::update, Calculator::view)
        .theme( |_| Theme::Dark )
        .antialiasing(true)
        .run()
}
