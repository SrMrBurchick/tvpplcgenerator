use std::{rc::Rc, cell::RefCell};

use configs::{IOConfig, IO_CONFIG, SUBPROGRAMS_CONFIG, SubprogramConfig};
use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command,
    Container, Element, Length, Settings, Text, scrollable, Row, Space
};

mod configuration;
use configuration:: {
    Config, language_pack_conastants::{BUTTON_NEXT, BUTTON_BACK}, GLOBAL_CONFIG,
    style_config::{self, FONT_SIZE, DEFAULT_PADDING}
};
use view::{PresetViewMessage, PresetViews};

mod view;
mod ioconfigview;
mod subprogramview;
mod configs;

#[derive(Debug, Clone)]
pub enum Message {
    BackPresset,
    NextPresset,
    PresetViewMessage(PresetViewMessage),
}

#[derive(Debug)]
pub struct Generator {
    active_preset: usize,
    scroll: scrollable::State,
    presets: Vec<PresetViews>,
    next_preset: button::State,
    back_preset: button::State,
}

impl Application for Generator {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Generator, Command<Message>) {
        (
            Generator {
                active_preset: 0,
                scroll: scrollable::State::new(),
                presets: vec![
                    PresetViews::EntryView {
                        create_new_button: button::State::new(),
                        load_table_button: button::State::new(),
                    },
                    PresetViews::IOConfigView {
                        scroll: scrollable::State::new(),
                        create_new_button: button::State::new(),
                        elements: vec![]
                    },
                    PresetViews::SubprogramConfigView {
                        scroll: scrollable::State::new(),
                        create_new_button: button::State::new(),
                        subprograms: vec![]
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
            Message::PresetViewMessage(preset_message) => {
                match preset_message {
                    PresetViewMessage::NextPresset => {
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
                                .view().map(Message::PresetViewMessage))
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

fn init() {
    unsafe {
        GLOBAL_CONFIG = Some(Rc::new(Config::new()));
        IO_CONFIG = Some(Rc::new(RefCell::new(IOConfig::new())));
        SUBPROGRAMS_CONFIG = Some(Rc::new(RefCell::new(SubprogramConfig::new())))
    }
}

fn main() -> iced::Result {
    init();

    Generator::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
