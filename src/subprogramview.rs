use std::{rc::Rc, cell::RefCell};

use iced::{
    button, Button, Column, Element, Length, Text, Row, pick_list, PickList, text_input, TextInput
};

use crate::{configuration:: {
    language_pack_conastants::{
        FIELD_NAME,  FIELD_ADDRESS, BUTTON_EDIT_STATES_SUBPROGRAM_STEP, SUBPROGRAM_STEP, OPERATOR, BUTTON_EDIT_CONTROLS_SUBPROGRAM_STEP, FIELD_SIGNAL, FIELD_TYPE_STATE, FIELD_DESCRIPTION, FIELD_TYPE
    },
    style_config::{DEFAULT_SPACING, SUBPRORAM_DESCRIPTION_WIDTH}, GLOBAL_CONFIG, delete_icon, edit_icon, Operators, IOElementStates, FrameTypes, SubprogramTypes
}, configs::{SubprogramStep, SubprogramStepMessage, IOElementCoditions, IOElementCoditionsMessage, IO_CONFIG, IOElement}};

use crate::configs::{
    SubprogramMessage, Subprogram
};

static OPERATORS_ALL: &[Operators] = &[
    Operators::AND,
    Operators::OR,
];

static IO_STATES_ALL: &[IOElementStates] = &[
    IOElementStates::Active,
    IOElementStates::Inactive,
    IOElementStates::Any,
];

static SUBPROGRAM_TYPES_ALL: &[SubprogramTypes] = &[
    SubprogramTypes::Dflt,
    SubprogramTypes::Critical,
    SubprogramTypes::Blocked,
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
            .width(Length::Units(SUBPRORAM_DESCRIPTION_WIDTH));

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
        ).on_press(SubprogramStepMessage::PickConditions(FrameTypes::State));

        let edit_control_button = Button::new(
            &mut self.control_edit_button,
            Text::new(config.get_field(BUTTON_EDIT_CONTROLS_SUBPROGRAM_STEP).to_string())
        ).on_press(SubprogramStepMessage::PickConditions(FrameTypes::Control));

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

#[derive(Debug)]
pub struct SubprogramIOConditionsView {
    io_element_list: pick_list::State<String>,
    state_list: pick_list::State<IOElementStates>,
    delete_button: button::State,
    io_condition: Rc<RefCell<IOElementCoditions>>,
    io_list: Vec<String>,
}

impl<'a> SubprogramIOConditionsView {
    pub fn new(io_condition: Rc<RefCell<IOElementCoditions>>) -> Self {
        SubprogramIOConditionsView {
            io_element_list: pick_list::State::default(),
            state_list: pick_list::State::default(),
            delete_button: button::State::new(),
            io_condition: io_condition.clone(),
            io_list: vec![],
        }
    }

    fn get_names_list(io_list: &'a Vec<Rc<RefCell<IOElement>>>) -> Vec<String> {
        let mut name_list: Vec<String> = vec![];

        for io_element in io_list {
            let (name, ..) = io_element.borrow().get_data();
            name_list.push(name.clone());
        }

        name_list
    }

    pub fn view(&'a mut self) -> Element<'a, IOElementCoditionsMessage> {
        let (io_element, state, frame_type) = self.io_condition.borrow().get_data();
        let io_config = unsafe {
            &IO_CONFIG
        }.as_ref().unwrap();
        let io_list = io_config.borrow().get_elements_by_frame_type(frame_type);

        let (name, ..) = if io_element != None {
            io_element.unwrap().borrow().get_data()
        } else {
            io_list.first().unwrap().borrow().get_data()
        };

        let io_element_list = PickList::new(
            &mut self.io_element_list,
            Self::get_names_list(&io_list),
            Some(name),
            IOElementCoditionsMessage::IOElementSelected
        );

        let state_list = PickList::new(
            &mut self.state_list,
            IO_STATES_ALL,
            Some(state),
            IOElementCoditionsMessage::StateChanged
        );

        let config = unsafe {
            &GLOBAL_CONFIG
        }.as_ref().unwrap();

        let delete_button = Button::new(&mut self.delete_button, delete_icon())
            .on_press(IOElementCoditionsMessage::DeleteElement(frame_type));

        Row::new()
            .spacing(DEFAULT_SPACING)
            .push(Column::new()
                .push(Text::new(config.get_field(FIELD_SIGNAL).to_string()
                                                              .as_str()))
                .push(io_element_list))
            .push(Column::new()
                .push(Text::new(config.get_field(FIELD_TYPE_STATE).to_string()
                                                              .as_str()))
                .push(state_list))
            .push(delete_button)

            .into()
    }
}

#[derive(Debug, Clone)]
pub struct SubprogramDescriptionEditView {
    type_list: pick_list::State<SubprogramTypes>,
    description_input: text_input::State,
    subprogram: Rc<RefCell<Subprogram>>,
}

impl<'a> SubprogramDescriptionEditView {
    pub fn new(subprogram: Rc<RefCell<Subprogram>>) -> Self {
        SubprogramDescriptionEditView {
            type_list: pick_list::State::default(),
            description_input: text_input::State::new(),
            subprogram: subprogram.clone(),
        }
    }

    pub fn get_subprogram(&self) -> Rc<RefCell<Subprogram>> {
        self.subprogram.clone()
    }

    pub fn view(&'a mut self) -> Element<'a, SubprogramMessage> {
        let (address, desription, subprogram_type, _) = self.subprogram.borrow().get_data();

        let type_list = PickList::new(
            &mut self.type_list,
            SUBPROGRAM_TYPES_ALL,
            Some(subprogram_type),
            SubprogramMessage::SubprogramTypeSelected
        );

        let description_input = TextInput::new(
            &mut self.description_input,
            "",
            &desription,
            SubprogramMessage::SubprogramDescrptionChanged
        ).size(30)
            .width(Length::Units(SUBPRORAM_DESCRIPTION_WIDTH));

        let config = unsafe {
            &GLOBAL_CONFIG
        }.as_ref().unwrap();

        Row::new()
            .spacing(DEFAULT_SPACING)
            .push(Column::new()
                .push(Text::new(config.get_field(FIELD_ADDRESS).to_string()
                                                              .as_str()))
                .push(Text::new(format!("{}",address))))
            .push(Column::new()
                .push(Text::new(config.get_field(FIELD_DESCRIPTION).to_string()
                                                              .as_str()))
                .push(description_input))
            .push(Column::new()
                .push(Text::new(config.get_field(FIELD_TYPE).to_string()
                                                              .as_str()))
                .push(type_list))

            .into()
    }
}
