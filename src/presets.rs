use std::rc::Rc;

use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command,
    Container, Element, HorizontalAlignment, Length, Settings, Text, Scrollable,
    scrollable, Row, TextInput, text_input, futures::SinkExt, pick_list,
    PickList, Svg, Image, Font
};

use crate::{configuration:: {
    Config,
    language_pack_conastants::{
        CREATE_NEW, LOAD_TABLE, FIELD_NAME, BUTTON_ADD_NEW, FIELD_TYPE,
        FIELD_SIGNAL, FIELD_HW, IOCONFIG_EMPTY
    },
    style_config::{DEFAULT_PADDING, DEFAULT_SPACING, FONT_SIZE, self},
    FrameTypes, GLOBAL_CONFIG, SignalTypes, DELETE_BUTTON_PATH, FONTS_PATH
}, Message};

#[derive(Debug, Clone)]
pub enum PresetMessage {
    NextPresset,
    InputChanged(String),
    IOConfigMessage(IOConfigMessage)
}

#[derive(Debug)]
pub enum Presets {
    Entry {
        create_new_button: button::State,
        load_table_button: button::State,
    },
    IOConfig {
        scroll: scrollable::State,
        create_new_button: button::State,
        elements: Vec<IOConfigElement>,
    },
//    SubprogramConfig {
//        scroll: scrollable::State,
//        create_new_button: button::State,
//        subprogramms: Vec<Subprogram>,
//    },
//    SubprogramStepsConfig {
//        scroll: scrollable::State,
//        create_new_button: button::State,
//        steps: Vec<Step>
//    },
//    ConditionsConfig {
//        scroll: scrollable::State,
//        create_new_button: button::State,
//        conditions: Vec<Conditions>
//    }
}

#[derive(Debug, Clone)]
pub enum IOConfigMessage {
    AddNewElement,
    IOConfigElementMessage(usize, IOConfigElementMessage),
}

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

// Fonts
const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("./fonts/icons.ttf"),
};

fn icon(unicode: char) -> Text {
    Text::new(&unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(HorizontalAlignment::Center)
        .size(20)
}

fn edit_icon() -> Text {
    icon('\u{F303}')
}

fn delete_icon() -> Text {
    icon('\u{F1F8}')
}

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

impl <'a> Presets {
    pub fn view(&mut self) -> Element<PresetMessage> {
        match self {
           Presets::Entry {
                create_new_button,
                load_table_button
            } => Self::entry_view(create_new_button, load_table_button),
            Presets::IOConfig {
                scroll,
                create_new_button,
                elements
            } => Column::new()
                    .push(Self::ioconfig_view(
                            scroll,
                            create_new_button,
                            elements
                          ).map(PresetMessage::IOConfigMessage))
//           Presets::SubprogramConfig {
//                create_new_button,
//                load_table_button
//            } => Self::submprogtramconfig_view(create_new_button, load_table_button),
//           Presets::SubprogramStepsConfig {
//                create_new_button,
//                load_table_button
//            } => Self::subprogram_steps_view(create_new_button, load_table_button),
//           Presets::Subprogram {
//                create_new_button,
//                load_table_button
//            } => Self::subprogram_view(create_new_button, load_table_button),
//           Presets::ConditionsConfig {
//                create_new_button,
//                load_table_button
//            } => Self::conditions_view(create_new_button, load_table_button),
        }
        .into()
    }

    pub fn update(&mut self, message: PresetMessage) {
        match self {
            Presets::Entry {..} => Self::entry_view_update(message),
            Presets::IOConfig {elements, ..} => Self::ioconfig_view_update(elements, message)
        }
    }

    fn entry_view(
        create_new_button: &'a mut button::State,
        load_table_button: &'a mut button::State
    ) -> Column<'a, PresetMessage> {
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
                .on_press(PresetMessage::NextPresset))
            .push(Button::new(load_table_button,
                              Text::new(config.get_field(LOAD_TABLE)
                                        .to_string().as_str())
                              .size(FONT_SIZE))
                .style(style_config::Button::Primary)
                .on_press(PresetMessage::NextPresset))
    }

    fn entry_view_update(message: PresetMessage) {
        //TODO
    }

    fn ioconfig_view(
        scroll: &'a mut scrollable::State,
        create_new_button: &'a mut button::State,
        elements: &'a mut Vec<IOConfigElement>
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
                            IOConfigMessage::IOConfigElementMessage(i, message)
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
        elements: &'a mut Vec<IOConfigElement>,
        message: PresetMessage
    ) {
        match message {
            PresetMessage::IOConfigMessage(ioconfig_message) => {
                match ioconfig_message {
                    IOConfigMessage::IOConfigElementMessage(i, message) => {
                        match message {
                            IOConfigElementMessage::DeleteElement => {
                                elements.remove(i);
                            },
                            _ => {
                                if let Some(element) = elements.get_mut(i) {
                                    element.update(message)
                                }
                            }
                        }
                    },
                    IOConfigMessage::AddNewElement => {
                        elements.push(IOConfigElement::new())
                    }
                }
            },
            _ => ()
        }
    }

}
