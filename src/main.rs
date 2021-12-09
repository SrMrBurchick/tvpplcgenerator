use std::rc::Rc;

use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command,
    Container, Element, HorizontalAlignment, Length, Settings, Text, Scrollable, scrollable, Background, Color, Rectangle, Row, Space
};

mod configuration;
use configuration:: {
    Config, LanguagePack, language_pack_conastants::{self, BUTTON_NEXT, BUTTON_BACK}, GLOBAL_CONFIG, style_config::{self, DEFAULT_SPACING, FONT_SIZE, DEFAULT_PADDING}
};

mod presets;
use presets:: {
    PresetMessage, Presets
};

mod ioconfig;
use ioconfig:: {IOConfigElement};

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
    presets: Vec<Presets>,
    next_preset: button::State,
    back_preset: button::State,
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
                ],
                next_preset: button::State::new(),
                back_preset: button::State::new(),
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
                        if (self.active_preset + 1) < self.presets.len() {
                            self.active_preset += 1
                        }
                    },
                    _ => {
                        self.presets[self.active_preset].update(preset_message)
                    }
                }
            },
            Message::NextPresset => {
                if (self.active_preset + 1) < self.presets.len() {
                    self.active_preset += 1
                }
            },
            Message::BackPresset => {
                self.active_preset -= 1;
            },
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let config = unsafe {
            &GLOBAL_CONFIG
        }.as_ref().unwrap();
        let active_preset = self.active_preset;
        let mut content = Column::new();

        content = content.push(Container::new(self.presets[active_preset]
                                .view().map(Message::PresetMessage))
                .width(Length::Fill));

        let controls = Row::new()
            .align_items(Align::Center)
            .padding(DEFAULT_PADDING)
            .push(Button::new(&mut self.back_preset,
                              Text::new(config.get_field(BUTTON_BACK)
                                        .to_string().as_str()).size(FONT_SIZE))
                  .on_press(Message::BackPresset)
                  .style(style_config::Button::Secondary))
            .push(Space::with_width(Length::Fill))
            .push(Button::new(&mut self.next_preset,
                              Text::new(config.get_field(BUTTON_NEXT)
                                        .to_string().as_str()).size(FONT_SIZE)) 
                  .on_press(Message::NextPresset)
                  .style(style_config::Button::Primary));

        if self.active_preset > 0 {
            content = content
                .push(Space::with_height(Length::Fill))
                .push(controls.align_items(Align::End));
        }

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
