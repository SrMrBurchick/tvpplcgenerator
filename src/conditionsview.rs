use std::{rc::Rc, cell::RefCell};

use iced::{
    button, Button, Column, Element, Length, Text, Row, pick_list, PickList, text_input, TextInput, Checkbox
};

use crate::configuration:: {
    language_pack_conastants::{
        FIELD_NAME,  FIELD_ADDRESS, BUTTON_EDIT_STATES_SUBPROGRAM_STEP,
        BUTTON_EDIT_CONTROLS_SUBPROGRAM_STEP,  SUBPROGRAM_TYPE_BLOCKED,
        SUBPROGRAM_TYPE_CRITICAL
    },
    style_config::{DEFAULT_SPACING, SUBPRORAM_DESCRIPTION_WIDTH}, GLOBAL_CONFIG,
    delete_icon, FrameTypes,
};

use crate::configs::{
    ConditionsConfigElement, ConditionsConfigElementMessage, SUBPROGRAMS_CONFIG
};

#[derive(Debug)]
pub struct ConditonsElementView {
    state_edit_button: button::State,
    control_edit_button: button::State,
    delete_button: button::State,
    address_list: pick_list::State<usize>,
    description_input: text_input::State,
    conditonselement: Rc<RefCell<ConditionsConfigElement>>,
}

impl<'a> ConditonsElementView {
    pub fn new(conditonselement: Rc<RefCell<ConditionsConfigElement>>) -> Self {
        ConditonsElementView {
            delete_button: button::State::new(),
            state_edit_button: button::State::new(),
            control_edit_button: button::State::new(),
            description_input: text_input::State::new(),
            address_list: pick_list::State::default(),
            conditonselement: conditonselement.clone(),
        }
    }

    fn generate_address_list() -> Vec<usize> {
        let subprogramconfig = unsafe {
            &SUBPROGRAMS_CONFIG
        }.as_ref().unwrap();

        let mut address_list: Vec<usize> = vec![];

        for address in 1..subprogramconfig.borrow().get_last_address() {
            address_list.push(address);
        }

        address_list

    }

    pub fn view(&'a mut self) -> Element<'a, ConditionsConfigElementMessage> {
        let (description, _, _, blocked, critical, address) =
            self.conditonselement.borrow().get_data();

        let description_input = TextInput::new(
            &mut self.description_input,
            "", &description, ConditionsConfigElementMessage::DescriptionChanged
        ).size(30).width(Length::Units(SUBPRORAM_DESCRIPTION_WIDTH));

        let address_list = PickList::new(
            &mut self.address_list,
            Self::generate_address_list(),
            Some(address),
            ConditionsConfigElementMessage::AddressSelected
        );

        let config = unsafe {
            &GLOBAL_CONFIG
        }.as_ref().unwrap();

        let edit_state_button = Button::new(
            &mut self.state_edit_button,
            Text::new(config.get_field(BUTTON_EDIT_STATES_SUBPROGRAM_STEP).to_string())
        ).on_press(ConditionsConfigElementMessage::PickConditions(FrameTypes::State));

        let edit_control_button = Button::new(
            &mut self.control_edit_button,
            Text::new(config.get_field(BUTTON_EDIT_CONTROLS_SUBPROGRAM_STEP).to_string())
        ).on_press(ConditionsConfigElementMessage::PickConditions(FrameTypes::Control));

        let blocked_check = Checkbox::new(
            blocked,
            config.get_field(SUBPROGRAM_TYPE_BLOCKED).to_string().as_str(),
            ConditionsConfigElementMessage::BlockedPicked
         );

        let critical_check = Checkbox::new(
            critical,
            config.get_field(SUBPROGRAM_TYPE_CRITICAL).to_string().as_str(),
            ConditionsConfigElementMessage::CriticalPicked
         );

        let delete_button = Button::new(&mut self.delete_button, delete_icon())
            .on_press(ConditionsConfigElementMessage::DeleteCondition);

        Row::new()
            .spacing(DEFAULT_SPACING)
            .push(Column::new()
                .push(Text::new(config.get_field(FIELD_NAME).to_string()
                                                              .as_str()))
                .push(description_input))
            .push(edit_state_button)
            .push(edit_control_button)
            .push(blocked_check)
            .push(critical_check)
            .push(Column::new()
                .push(Text::new(config.get_field(FIELD_ADDRESS).to_string()
                                                              .as_str()))
                .push(address_list))
            .push(delete_button)

            .into()
    }
}
