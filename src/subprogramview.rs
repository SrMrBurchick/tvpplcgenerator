use std::{rc::Rc, cell::RefCell};

use iced::{
    button, Button, Column, Element, Length, Text, Row, pick_list, PickList
};

use crate::{configuration:: {
    language_pack_conastants::{
        FIELD_NAME,  FIELD_ADDRESS, BUTTON_EDIT_STATES_SUBPROGRAM_STEP, SUBPROGRAM_STEP, OPERATOR, BUTTON_EDIT_CONTROLS_SUBPROGRAM_STEP
    },
    style_config::DEFAULT_SPACING, GLOBAL_CONFIG, delete_icon, edit_icon, Operators
}, configs::{SubprogramStep, SubprogramStepMessage}};

use crate::configs::{
    SubprogramMessage, Subprogram
};

static OPERATORS_ALL: &[Operators] = &[
    Operators::AND,
    Operators::OR,
];

#[derive(Debug)]
pub struct SubprogramView {
    delete_button: button::State,
    edit_button: button::State,
    subprogram: Rc<RefCell<Subprogram>>,
}

impl<'a> SubprogramView {
    pub fn new(subprogram: Rc<RefCell<Subprogram>>) -> Self {
        SubprogramView {
            delete_button: button::State::new(),
            edit_button: button::State::new(),
            subprogram: subprogram.clone(),
        }
    }

    pub fn view(&'a mut self) -> Element<'a, SubprogramMessage> {
        let (address, name, ..) = self.subprogram.borrow().get_data();

        let address_label = Text::new(String::from(format!("{}", address)),
        ).size(30).width(Length::Units(140));

        let description_label = Text::new(name).size(30)
            .width(Length::Units(140));

        let edit_button = Button::new(&mut self.edit_button, edit_icon())
            .on_press(SubprogramMessage::SubprogramEdit);

        let delete_button = Button::new(&mut self.delete_button, delete_icon())
            .on_press(SubprogramMessage::SubprogramDelete);

        let config = unsafe {
            &GLOBAL_CONFIG
        }.as_ref().unwrap();

        Row::new()
            .spacing(DEFAULT_SPACING)
            .push(Column::new()
                .push(Text::new(config.get_field(FIELD_ADDRESS).to_string()
                                                              .as_str()))
                .push(address_label))
            .push(Column::new()
                .push(Text::new(config.get_field(FIELD_NAME).to_string()
                                                              .as_str()))
                .push(description_label))
            .push(edit_button)
            .push(delete_button)

            .into()
    }
}

#[derive(Debug)]
pub struct SubprogramStepView {
    state_edit_button: button::State,
    control_edit_button: button::State,
    delete_button: button::State,
    operator_list: pick_list::State<Operators>,
    subprogramstep: Rc<RefCell<SubprogramStep>>,
}

impl<'a> SubprogramStepView {
    pub fn new(subprogramstep: Rc<RefCell<SubprogramStep>>) -> Self {
        SubprogramStepView {
            delete_button: button::State::new(),
            state_edit_button: button::State::new(),
            control_edit_button: button::State::new(),
            operator_list: pick_list::State::default(),
            subprogramstep: subprogramstep.clone(),
        }
    }

    pub fn view(&'a mut self) -> Element<'a, SubprogramStepMessage> {
        let (step, operator, ..) = self.subprogramstep.borrow().get_data();

        let step_label = Text::new(String::from(format!("{}", step)),
        ).size(30).width(Length::Units(140));

        let operator_list = PickList::new(
            &mut self.operator_list,
            OPERATORS_ALL,
            Some(operator),
            SubprogramStepMessage::OperatorSelected
        );

        let config = unsafe {
            &GLOBAL_CONFIG
        }.as_ref().unwrap();

        let edit_state_button = Button::new(
            &mut self.state_edit_button,
            Text::new(config.get_field(BUTTON_EDIT_STATES_SUBPROGRAM_STEP).to_string())
        ).on_press(SubprogramStepMessage::PickStateConditions);

        let edit_control_button = Button::new(
            &mut self.control_edit_button,
            Text::new(config.get_field(BUTTON_EDIT_CONTROLS_SUBPROGRAM_STEP).to_string())
        ).on_press(SubprogramStepMessage::PickControlConditions);

        let delete_button = Button::new(&mut self.delete_button, delete_icon())
            .on_press(SubprogramStepMessage::DeleteStep);

        Row::new()
            .spacing(DEFAULT_SPACING)
            .push(Column::new()
                .push(Text::new(config.get_field(SUBPROGRAM_STEP).to_string()
                                                              .as_str()))
                .push(step_label))
            .push(Column::new()
                .push(Text::new(config.get_field(OPERATOR).to_string()
                                                              .as_str()))
                .push(operator_list))
            .push(edit_state_button)
            .push(edit_control_button)
            .push(delete_button)

            .into()
    }
}

