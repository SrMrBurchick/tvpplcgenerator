use std::rc::Rc;

use iced::{
    button, Align, Button, Column, Container, Element, HorizontalAlignment,
    Length, Text, Scrollable, scrollable
};

use crate::{configuration:: {
    language_pack_conastants::{
        CREATE_NEW, LOAD_TABLE, BUTTON_ADD_NEW, IOCONFIG_EMPTY
    },
    style_config::{DEFAULT_PADDING, DEFAULT_SPACING, FONT_SIZE, self},
    GLOBAL_CONFIG
}, configs::{IOConfig, SubprogramConfigMessage, SUBPROGRAMS_CONFIG, SubprogramMessage}, subprogramview::SubprogramView};

use crate::ioconfigview::IOElementView;
use crate::configs::{IOConfigMessage, IOElementMessage, IO_CONFIG};

#[derive(Debug, Clone)]
pub enum PresetViewMessage {
    NextPresset,
    InputChanged(String),
    IOConfigMessage(IOConfigMessage),
    SubprogramConfigMessage(SubprogramConfigMessage),
}

#[derive(Debug)]
pub enum PresetViews {
    EntryView {
        create_new_button: button::State,
        load_table_button: button::State,
    },
    IOConfigView {
        scroll: scrollable::State,
        create_new_button: button::State,
        elements: Vec<IOElementView>,
    },
    SubprogramConfigView {
        scroll: scrollable::State,
        create_new_button: button::State,
        subprograms: Vec<SubprogramView>,
    }
}

fn empty_message<'a>(message: &str) -> Element<'a, IOConfigMessage> {
    Container::new(
        Text::new(message)
            .width(Length::Fill)
            .size(25)
            .horizontal_alignment(HorizontalAlignment::Center)
            .color([0.7, 0.7, 0.7]),
    )
    .width(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

impl <'a> PresetViews {
    pub fn view(&mut self) -> Element<PresetViewMessage> {
        match self {
           PresetViews::EntryView {
                create_new_button,
                load_table_button
            } => {
                Self::entry_view(create_new_button, load_table_button)
            },
            PresetViews::IOConfigView {
                scroll,
                create_new_button,
                elements
            } => {
                Column::new()
                    .push(Self::ioconfig_view(
                            scroll,
                            create_new_button,
                            elements
                          ).map(PresetViewMessage::IOConfigMessage))
            },
            PresetViews::SubprogramConfigView {
                scroll,
                subprograms,
                create_new_button
            } => {
                Column::new()
                    .push(Self::subrogramconfig_view(
                            scroll,
                            create_new_button,
                            subprograms
                          ).map(PresetViewMessage::SubprogramConfigMessage))
            },

        }
        .into()
    }

    pub fn update(&mut self, message: PresetViewMessage) {
        match self {
            PresetViews::EntryView {..} => Self::entry_view_update(message),
            PresetViews::IOConfigView {elements, ..} => Self::ioconfig_view_update(elements, message),
            PresetViews::SubprogramConfigView {subprograms, ..} => Self::subprogramconfig_view_update(subprograms, message),
        }
    }

    fn entry_view(
        create_new_button: &'a mut button::State,
        load_table_button: &'a mut button::State
    ) -> Column<'a, PresetViewMessage> {
        let config = unsafe {
            &GLOBAL_CONFIG
        }.as_ref().unwrap();
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
                .on_press(PresetViewMessage::NextPresset))
            .push(Button::new(load_table_button,
                              Text::new(config.get_field(LOAD_TABLE)
                                        .to_string().as_str())
                              .size(FONT_SIZE))
                .style(style_config::Button::Primary)
                .on_press(PresetViewMessage::NextPresset))
    }

    fn entry_view_update(message: PresetViewMessage) {
        //TODO
    }

    fn ioconfig_view(
        scroll: &'a mut scrollable::State,
        create_new_button: &'a mut button::State,
        elements: &'a mut Vec<IOElementView>
    ) -> Element<'a, IOConfigMessage> {
        let config = unsafe {
            &GLOBAL_CONFIG
        }.as_ref().unwrap();

        let add_new = Column::new()
                  .align_items(Align::Center)
                  .width(Length::Fill)
                  .push(Button::new(create_new_button,
                              Text::new(config.get_field(BUTTON_ADD_NEW)
                                        .to_string().as_str())
                              .size(FONT_SIZE))
                        .style(style_config::Button::Primary)
                        .on_press(IOConfigMessage::AddNewElement));

        let elements_view: Element<_> = if elements.len() > 0 {
                elements
                    .iter_mut()
                    .enumerate()
                    .fold(Column::new().spacing(20), |column, (i, element)| {
                        column.push(element.view().map(move |message| {
                            IOConfigMessage::IOElementMessage(i, message)
                        }))
                    })
                .into()
        } else {
            empty_message(config.get_field(IOCONFIG_EMPTY).to_string().as_str())
        };


        let scrollable = Scrollable::new(scroll)
            .align_items(Align::Start)
            .spacing(DEFAULT_SPACING)
            .padding(DEFAULT_PADDING)
            .push(Column::new()
                    .push(Container::new(elements_view))
                    .width(Length::Fill)
                    .align_items(Align::Center))
            .push(add_new);


        Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(scrollable)
        .into()
    }

    fn ioconfig_view_update(
        elements: &'a mut Vec<IOElementView>,
        message: PresetViewMessage
    ) {
        match message {
            PresetViewMessage::IOConfigMessage(ioconfig_message) => {
                let ioconfig = unsafe {
                    &IO_CONFIG
                }.as_ref().unwrap();

                ioconfig.borrow_mut().update(ioconfig_message.clone());

                match ioconfig_message {
                    IOConfigMessage::IOElementMessage(i, message) => {
                        match message {
                            IOElementMessage::DeleteElement => {
                                elements.remove(i);
                            },
                            _ => {
                            }
                        }
                    },
                    IOConfigMessage::AddNewElement => {
                        elements.push(IOElementView::new(
                                        unsafe{&IO_CONFIG}.as_ref().unwrap()
                                        .borrow().get_last_element()
                                    ))
                    }
                }
            },
            _ => ()
        }
    }

    fn subrogramconfig_view(
        scroll: &'a mut scrollable::State,
        create_new_button: &'a mut button::State,
        elements: &'a mut Vec<SubprogramView>
    ) -> Element<'a, SubprogramConfigMessage> {
        let config = unsafe {
            &GLOBAL_CONFIG
        }.as_ref().unwrap();

        let add_new = Column::new()
                  .align_items(Align::Center)
                  .width(Length::Fill)
                  .push(Button::new(create_new_button,
                              Text::new(config.get_field(BUTTON_ADD_NEW)
                                        .to_string().as_str())
                              .size(FONT_SIZE))
                        .style(style_config::Button::Primary)
                        .on_press(SubprogramConfigMessage::AddNewSubprogram));

        let elements_view: Element<_> = if elements.len() > 0 {
                elements
                    .iter_mut()
                    .enumerate()
                    .fold(Column::new().spacing(20), |column, (i, element)| {
                        column.push(element.view().map(move |message| {
                            SubprogramConfigMessage::SubprogramMessage(i, message)
                        }))
                    })
                .into()
        } else {
            Container::new(
                Text::new(config.get_field(IOCONFIG_EMPTY).to_string().as_str())
                    .width(Length::Fill)
                    .size(25)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .color([0.7, 0.7, 0.7]),
            )
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into()
        };


        let scrollable = Scrollable::new(scroll)
            .align_items(Align::Start)
            .spacing(DEFAULT_SPACING)
            .padding(DEFAULT_PADDING)
            .push(Column::new()
                    .push(Container::new(elements_view))
                    .width(Length::Fill)
                    .align_items(Align::Center))
            .push(add_new);


        Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(scrollable)
        .into()
    }

    fn subprogramconfig_view_update(
        elements: &'a mut Vec<SubprogramView>,
        message: PresetViewMessage
    ) {
        match message {
            PresetViewMessage::SubprogramConfigMessage(subprogramconfig_message) => {
                let subrogramconfig = unsafe {
                    &SUBPROGRAMS_CONFIG
                }.as_ref().unwrap();

                subrogramconfig.borrow_mut().update(subprogramconfig_message.clone());

                match subprogramconfig_message {
                    SubprogramConfigMessage::SubprogramMessage(i, message) => {
                        match message {
                            SubprogramMessage::SubprogramDelete => {
                                elements.remove(i);
                            },
                            _ => {
                            }
                        }
                    },
                    SubprogramConfigMessage::AddNewSubprogram => {
                        elements.push(SubprogramView::new(
                                        unsafe{&SUBPROGRAMS_CONFIG}.as_ref().unwrap()
                                        .borrow().get_last_subprogram()
                                    ))
                    }
                }
            },
            _ => ()
        }
    }


}
