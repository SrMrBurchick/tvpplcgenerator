use iced::{
    button, Align, Button, Column, Element, Length, Text, Row, TextInput,
    text_input, pick_list, PickList
};

use crate::configuration:: {
    language_pack_conastants::{
        FIELD_NAME, FIELD_TYPE, FIELD_SIGNAL, FIELD_HW
    },
    style_config::DEFAULT_SPACING,
    FrameTypes, GLOBAL_CONFIG, SignalTypes,
    delete_icon
};

#[derive(Debug, Clone)]
pub enum IOConfigElementMessage {
    NameInputChanged(String),
    FrameTypeSelected(FrameTypes),
    SignalTypeSelected(SignalTypes),
    HwSelected(String),
    DeleteElement,
}

#[derive(Debug)]
pub struct IOConfigElement {
    name_input: text_input::State,
    name_input_value: String,
    type_list: pick_list::State<FrameTypes>,
    selected_type: FrameTypes,
    signal_list: pick_list::State<SignalTypes>,
    selected_signal: SignalTypes,
    hw_input: text_input::State,
    selected_hw: String,
    delete_button: button::State,
}

static FRAME_TYPES_ALL: &[FrameTypes] = &[
    FrameTypes::State,
    FrameTypes::Control,
];

static SIGNAL_TYPES_ALL: &[SignalTypes] = &[
    SignalTypes::Input,
    SignalTypes::Output,
];

impl<'a> IOConfigElement {
    pub fn new() -> Self {
        IOConfigElement {
            name_input: text_input::State::new(),
            name_input_value: String::new(),
            type_list: pick_list::State::default(),
            selected_type: FrameTypes::State,
            signal_list: pick_list::State::default(),
            selected_signal: SignalTypes::Input,
            hw_input: text_input::State::new(),
            selected_hw: String::new(),
            delete_button: button::State::new(),
        }
    }

    pub fn update(&'a mut self, message: IOConfigElementMessage) {
        match message {
            IOConfigElementMessage::FrameTypeSelected(.., frametype) => {
                self.selected_type = frametype
            },
            IOConfigElementMessage::NameInputChanged(.., name) => {
                self.name_input_value = name
            },
            IOConfigElementMessage::SignalTypeSelected(.., signal_type) => {
                self.selected_signal = signal_type
            },
            IOConfigElementMessage::HwSelected(.., hw) => {
                self.selected_hw = hw
            }
            IOConfigElementMessage::DeleteElement => {}
        }
    }

    pub fn view(&'a mut self) -> Element<'a, IOConfigElementMessage> {
        let name_input = TextInput::new(
            &mut self.name_input,
            "", &self.name_input_value.as_str(),
            IOConfigElementMessage::NameInputChanged
        ).size(30).width(Length::Units(140));

        let type_list = PickList::new(
            &mut self.type_list,
            FRAME_TYPES_ALL,
            Some(self.selected_type),
            IOConfigElementMessage::FrameTypeSelected
        );

        let signal_list = PickList::new(
            &mut self.signal_list,
            SIGNAL_TYPES_ALL,
            Some(self.selected_signal),
            IOConfigElementMessage::SignalTypeSelected
        );

        let hw_input = TextInput::new(
            &mut self.hw_input,
            "", &self.selected_hw.as_str(),
            IOConfigElementMessage::HwSelected
        ).size(30).width(Length::Units(30));

        let config = unsafe {
            &GLOBAL_CONFIG
        }.as_ref().unwrap();

        let delete_button = Button::new(
            &mut self.delete_button, delete_icon())
            .on_press(IOConfigElementMessage::DeleteElement);

        Row::new()
            .spacing(DEFAULT_SPACING)
            .push(Column::new()
                .push(Text::new(config.get_field(FIELD_NAME).to_string()
                                                              .as_str()))
                .push(name_input))
            .push(Column::new()
                .push(Text::new(config.get_field(FIELD_TYPE).to_string()
                                                              .as_str()))
                .push(type_list))
            .push(Column::new()
                .push(Text::new(config.get_field(FIELD_SIGNAL).to_string()
                                                              .as_str()))
                .push(signal_list))
            .push(Column::new()
                .push(Text::new(config.get_field(FIELD_HW).to_string()
                                                              .as_str()))
                .push(hw_input))
            .push(Column::new().width(Length::Units(40)).height(Length::Units(40)).align_items(Align::Center).push(delete_button))

            .into()

    }
}
