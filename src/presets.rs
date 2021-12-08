use std::rc::Rc;

use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command,
    Container, Element, HorizontalAlignment, Length, Settings, Text,
};

use crate::configuration:: {
    Config,
    language_pack_conastants::{CREATE_NEW, LOAD_TABLE},
    style_config::{DEFAULT_PADDING, DEFAULT_SPACING, FONT_SIZE, self}
};

#[derive(Debug, Clone)]
pub enum PresetMessage {
    NextPresset,
}

#[derive(Debug)]
pub enum Presets {
    Entry {
        config: Rc<Config>,
        create_new_button: button::State,
        load_table_button: button::State,
    },
}

impl <'a> Presets {
    pub fn view(&mut self) -> Element<PresetMessage> {
        match self {
            Presets::Entry {
                config,
                create_new_button,
                load_table_button
            } => Self::entry_view(config, create_new_button, load_table_button)
        }
        .into()
    }

    fn entry_view(
        config: &'a Rc<Config>,
        create_new_button: &'a mut button::State,
        load_table_button: &'a mut button::State
    ) -> Column<'a, PresetMessage> {
        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(DEFAULT_SPACING)
            .padding(DEFAULT_PADDING)
            .align_items(Align::Center)
            .push(Button::new(create_new_button,
                              Text::new(config.get_field(CREATE_NEW)
                                        .to_string().as_str())
                              .size(FONT_SIZE))
                .style(style_config::Button::Primary)
                .on_press(PresetMessage::NextPresset))
            .push(Button::new(load_table_button,
                              Text::new(config.get_field(LOAD_TABLE)
                                        .to_string().as_str())
                              .size(FONT_SIZE))
                .style(style_config::Button::Primary)
                .on_press(PresetMessage::NextPresset))
    }
}
