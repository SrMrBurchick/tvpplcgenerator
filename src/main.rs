use std::rc::Rc;

use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command,
    Container, Element, HorizontalAlignment, Length, Settings, Text, Scrollable, scrollable, Background, Color, Rectangle
};

mod configuration;
use configuration:: {
    Config, LanguagePack, language_pack_conastants, GLOBAL_CONFIG
};

mod presets;
use presets:: {
    PresetMessage, Presets, IOConfigElement
};

#[derive(Debug, Clone)]
pub enum Message {
    BackPresset,
    NextPresset,
    PresetMessage(PresetMessage),
}

#[derive(Debug)]
pub struct Generator {
    active_preset: usize,
    scroll: scrollable::State,
    presets: Vec<Presets>
}

impl Application for Generator {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Generator, Command<Message>) {
        unsafe {
            GLOBAL_CONFIG = Some(Rc::new(Config::new()));
        }

        (
            Generator {
                active_preset: 0,
                scroll: scrollable::State::new(),
                presets: vec![
                    Presets::Entry {
                        create_new_button: button::State::new(),
                        load_table_button: button::State::new(),
                    },
                    Presets::IOConfig {
                        scroll: scrollable::State::new(),
                        create_new_button: button::State::new(),
                        elements: vec![]
                    }
                ]
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("TVP PLC Generator")
    }

    fn update(
        &mut self,
        message: Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Message> {
        match message {
            Message::PresetMessage(preset_message) => {
                match preset_message {
                    PresetMessage::NextPresset => {
                        self.active_preset += 1
                    },
                    _ => {
                        self.presets[self.active_preset].update(preset_message)
                    }
                }
            },
            _ => ()
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let active_preset = self.active_preset;
        let content =
            Column::new()
            .push(Container::new(self.presets[active_preset]
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
