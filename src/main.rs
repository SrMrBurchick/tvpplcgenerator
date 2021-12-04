use iced::{executor, Application, Command, Element, Settings, Text, Executor, Clipboard};

#[derive(Debug)]
struct Generator;

impl Application for Generator {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Generator, Command<Self::Message>) {
        (Generator, Command::none())
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, _message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello, world!").into()
    }
}

fn main() -> iced::Result {
    Generator::run(Settings::default())
}
