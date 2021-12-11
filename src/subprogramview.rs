use std::{rc::Rc, cell::RefCell};

use iced::{
    button, Button, Column, Element, Length, Text, Row
};

use crate::configuration:: {
    language_pack_conastants::{
        FIELD_NAME,  FIELD_ADDRESS
    },
    style_config::DEFAULT_SPACING, GLOBAL_CONFIG, delete_icon, edit_icon
};

use crate::configs::{
    SubprogramMessage, Subprogram
};

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
