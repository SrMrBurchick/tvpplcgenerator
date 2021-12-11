use std::{rc::Rc, cell::RefCell};

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

use crate::configs:: {
    IOElement, IOElementMessage
};

#[derive(Debug)]
pub struct IOElementView {
    name_input: text_input::State,
    type_list: pick_list::State<FrameTypes>,
    signal_list: pick_list::State<SignalTypes>,
    hw_input: text_input::State,
    delete_button: button::State,
    ioelemnt: Rc<RefCell<IOElement>>,
}

static FRAME_TYPES_ALL: &[FrameTypes] = &[
    FrameTypes::State,
    FrameTypes::Control,
];

static SIGNAL_TYPES_ALL: &[SignalTypes] = &[
    SignalTypes::Input,
    SignalTypes::Output,
];

impl<'a> IOElementView {
    pub fn new(ioelemnt: Rc<RefCell<IOElement>>) -> Self {
        IOElementView {
            name_input: text_input::State::new(),
            type_list: pick_list::State::default(),
            signal_list: pick_list::State::default(),
            hw_input: text_input::State::new(),
            delete_button: button::State::new(),
            ioelemnt: ioelemnt.clone(),
        }
    }

    pub fn view(&'a mut self) -> Element<'a, IOElementMessage> {
        let (name, frame_type, signal_type, hw_address) =
            self.ioelemnt.borrow().get_data();

        let name_input = TextInput::new(
            &mut self.name_input,
            "", &name.as_str(),
            IOElementMessage::NameInputChanged
        ).size(30).width(Length::Units(140));

        let type_list = PickList::new(
            &mut self.type_list,
            FRAME_TYPES_ALL,
            Some(frame_type),
            IOElementMessage::FrameTypeSelected
        );

        let signal_list = PickList::new(
            &mut self.signal_list,
            SIGNAL_TYPES_ALL,
            Some(signal_type),
            IOElementMessage::SignalTypeSelected
        );

        let hw_input = TextInput::new(
            &mut self.hw_input,
            "", String::from(format!("{}", hw_address)).as_str(),
            IOElementMessage::HwSelected
        ).size(30).width(Length::Units(30));

        let config = unsafe {
            &GLOBAL_CONFIG
        }.as_ref().unwrap();

        let delete_button = Button::new(
            &mut self.delete_button, delete_icon())
            .on_press(IOElementMessage::DeleteElement);

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
