use std::rc::Rc;

use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command,
    Container, Element, HorizontalAlignment, Length, Settings, Text, Scrollable, scrollable, Background, Color, Rectangle
};

mod configuration;
use configuration:: {
    Config, LanguagePack, language_pack_conastants
};

mod presets;
use presets:: {
    PresetMessage, Presets
};

#[derive(Debug, Clone)]
pub enum Message {
    BackPresset,
    NextPresset,
    PresetMessage(PresetMessage),
}

#[derive(Debug)]
pub struct Generator {
    config: Rc<Config>,
    active_preset: Presets,
    scroll: scrollable::State,
}

impl Application for Generator {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Generator, Command<Message>) {
        let config: Rc<Config> = Rc::new(Config::new());

        (
            Generator {
                config: { config.clone() },
                active_preset: {
                    Presets::Entry {
                        config: config.clone(),
                        create_new_button: button::State::new(),
                        load_table_button: button::State::new(),
                    }
                },
                scroll: scrollable::State::new()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("TVP PLC Generator")
    }

    fn update(
        &mut self,
        _message: Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let content =
            Column::new().
            push(Container::new(self.active_preset
                                .view().map(Message::PresetMessage))
                .width(Length::Fill));

        Container::new(content)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_y()
            .into()
    }
}

fn main() -> iced::Result {
    Generator::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
