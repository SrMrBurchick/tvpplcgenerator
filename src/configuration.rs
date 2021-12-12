use iced::{Font, Length, Text, HorizontalAlignment};
use json::{self, JsonValue};
use std::{collections::HashMap, fs::File, io::BufReader, io::Read, rc::Rc};

use crate::configuration::language_pack_conastants::{FIELD_TYPE_STATE, FIELD_TYPE_CONTROL, FIELD_SIGNAL_INPUT, FIELD_SIGNAL_OUTPUT, SUBPROGRAM_TYPE_DEFAULT, SUBPROGRAM_TYPE_CRITICAL, SUBPROGRAM_TYPE_BLOCKED, IO_STATE_ACTIVE, IO_STATE_INACTIVE, IO_STATE_ANY};

use self::language_pack_conastants::DEFAULT;

pub static DEFAULT_LANGUAGE_PACK: &str = "./src/languages/US.json";
pub static DELETE_BUTTON_PATH: &str = "./src/images/DeleteButton.svg";
pub static FONTS_PATH: &str = "./src/fonts/icons.ttf";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameTypes {
    State,
    Control,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalTypes{
    Input,
    Output,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operators{
    AND,
    OR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubprogramTypes{
    Dflt,
    Critical,
    Blocked
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOElementStates{
    Active,
    Inactive,
    Any,
}


pub static mut GLOBAL_CONFIG: Option<Rc<Config>> = None;

pub mod language_pack_conastants {
    pub static BUTTON_ADD_NEW: &str = "BUTTON_ADD_NEW";
    pub static IOCONFIG_EMPTY: &str = "IOCONFIG_EMPTY";
    pub static BUTTON_BACK: &str = "BUTTON_BACK";
    pub static BUTTON_EDIT_STATES_SUBPROGRAM_STEP: &str = "BUTTON_EDIT_STATES_SUBPROGRAM_STEP";
    pub static BUTTON_EDIT_CONTROLS_SUBPROGRAM_STEP: &str = "BUTTON_EDIT_CONTROLS_SUBPROGRAM_STEP";
    pub static SUBPROGRAM_STEP: &str = "SUBPROGRAM_STEP";
    pub static OPERATOR: &str = "OPERATOR";
    pub static BUTTON_FINISH: &str = "BUTTON_FINISH";
    pub static BUTTON_GENERATE_TABLE: &str = "BUTTON_GENERATE_TABLE";
    pub static BUTTON_NEXT: &str = "BUTTON_NEXT";
    pub static BUTTON_TO_CONDITIONS: &str = "BUTTON_TO_CONDITIONS";
    pub static BUTTON_TO_IO_CONFIGURATION: &str = "BUTTON_TO_IO_CONFIGURATION";
    pub static BUTTON_TO_SUBPROGRAMMS_CONFIGURATION: &str = "BUTTON_TO_SUBPROGRAMMS_CONFIGURATION";
    pub static CREATE_NEW: &str = "CREATE_NEW";
    pub static DEFAULT: &str = "DEFAULT";
    pub static FIELD_ADDRESS: &str = "FIELD_ADDRESS";
    pub static FIELD_DESCRIPTION: &str = "FIELD_DESCRIPTION";
    pub static FIELD_HW: &str = "FIELD_HW";
    pub static FIELD_NAME: &str = "FIELD_NAME";
    pub static FIELD_SIGNAL: &str = "FIELD_SIGNAL";
    pub static FIELD_SIGNAL_INPUT: &str = "FIELD_SIGNAL_INPUT";
    pub static FIELD_SIGNAL_OUTPUT: &str = "FIELD_SIGNAL_OUTPUT";
    pub static FIELD_TYPE: &str = "FIELD_TYPE";
    pub static FIELD_TYPE_CONTROL: &str = "FIELD_TYPE_CONTROL";
    pub static FIELD_TYPE_STATE: &str = "FIELD_TYPE_STATE";
    pub static INFO: &str = "INFO";
    pub static LOAD_TABLE: &str = "LOAD_TABLE";
    pub static SUBPROGRAM_TYPE_BLOCKED: &str = "SUBPROGRAM_TYPE_BLOCKED";
    pub static SUBPROGRAM_TYPE_CRITICAL: &str = "SUBPROGRAM_TYPE_CRITICAL";
    pub static SUBPROGRAM_TYPE_DEFAULT: &str = "SUBPROGRAM_TYPE_DEFAULT";
    pub static IO_STATE_ACTIVE: &str = "IO_STATE_ACTIVE";
    pub static IO_STATE_INACTIVE: &str = "IO_STATE_INACTIVE";
    pub static IO_STATE_ANY: &str = "IO_STATE_ANY";
    pub static TABLE_CONTENT_SENSOR_STATES: &str = "TABLE_CONTENT_SENSOR_STATES";
    pub static TABLE_CONTENT_CONTROL_STATES: &str = "TABLE_CONTENT_CONTROL_STATES";
    pub static TABLE_CONTENT_TRASITION_ADDRESS: &str = "TABLE_CONTENT_TRASITION_ADDRESS";
    pub static TABLE_CONTENT_SIGN_OF_TRANSITION: &str = "TABLE_CONTENT_SIGN_OF_TRANSITION";
    pub static TABLE_CONTENT_SIGN_OF_BLOCKING: &str = "TABLE_CONTENT_SIGN_OF_BLOCKING";
    pub static TABLE_CONTENT_DESCRIPTION: &str = "TABLE_CONTENT_DESCRIPTION";
    pub static TABLE_SHEET_CONDITIONS: &str = "TABLE_SHEET_CONDITIONS";
    pub static TABLE_SHEET_SUBPROGRAMS: &str = "TABLE_SHEET_SUBPROGRAMS";
    pub static TABLE_CONTENT_SIGN_OF_FINISH: &str = "TABLE_CONTENT_SIGN_OF_FINISH";
    pub static TABLE_CONTENT_SUBPROGRAM_INITIAL: &str = "TABLE_CONTENT_SUBPROGRAM_INITIAL";
}

#[derive(Debug, PartialEq, Eq)]
pub struct LanguagePack {
    file_path: String,
    content_str: String,
    content_json: JsonValue,
}

impl LanguagePack {
    pub fn new() -> Self {
        LanguagePack {
            file_path: String::new(),
            content_str: String::new(),
            content_json: JsonValue::Null,
        }
    }

    pub fn load_language_pack(&mut self, path: String) -> std::io::Result<()> {
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);

        self.content_str = String::new();
        buf_reader.read_to_string(&mut self.content_str)?;

        self.content_json = json::parse(self.content_str.as_str()).unwrap();

        Ok(())
    }

    pub fn get_value(&self, name: &str) -> JsonValue {
        if JsonValue::Null != self.content_json {
            self.content_json[name].clone()
        } else {
            JsonValue::Null
        }
    }
}

pub mod style_config {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                })),
                border_radius: 5.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 2.0),
                ..self.active()
            }
        }
    }

    pub const FONT_SIZE: u16 = 40;
    pub const DEFAULT_PADDING: u16 = 50;
    pub const DEFAULT_SPACING: u16 = 40;
    pub const SUBPRORAM_DESCRIPTION_WIDTH: u16 = 300;
    pub const SUBPRORAM_DESCRIPTION_HEIGTH: u16 = 300;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    active_language_pack: String,
    languages_pack: HashMap<String, LanguagePack>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            languages_pack: {
                let mut map = HashMap::new();
                let mut default = LanguagePack::new();

                default.load_language_pack(String::from(DEFAULT_LANGUAGE_PACK));

                map.insert(String::from(DEFAULT), default);

                map
            },
            active_language_pack: String::from(DEFAULT),
        }
    }

    pub fn get_field(&self, name: &str) -> JsonValue {
        let mut lp = self.languages_pack.get(&self.active_language_pack);
        let mut value = lp.unwrap().get_value(name);

        if JsonValue::Null == value && String::from(DEFAULT) != self.active_language_pack {
            lp = self.languages_pack.get(&String::from(DEFAULT));
            value = lp.unwrap().get_value(name);
        }

        value
    }

    fn add_language_pack(&mut self, path: &str) {
        let mut language_pack = LanguagePack::new();

        language_pack.load_language_pack(path.to_string());

        let info = language_pack.get_value(language_pack_conastants::INFO);

        let value = self.languages_pack.get(info.to_string().as_str());
        if None == Some(value) {
            self.languages_pack.insert(info.to_string(), language_pack);
        }
    }

    pub fn search_language_packs(&mut self, path: &str) {
        //TODO: Add language packs searching
    }
}

impl std::fmt::Display for FrameTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let config = unsafe {
            &GLOBAL_CONFIG
        }.as_ref().unwrap();

        let state_string = config.get_field(FIELD_TYPE_STATE).to_string();
        let control_string = config.get_field(FIELD_TYPE_CONTROL).to_string();

        write!(
            f,
            "{}",
            match self {
                FrameTypes::State => state_string.as_str(),
                FrameTypes::Control => control_string.as_str(),
            }
        )
    }
}

impl std::fmt::Display for SignalTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let config = unsafe {
            &GLOBAL_CONFIG
        }.as_ref().unwrap();

        let input_string = config.get_field(FIELD_SIGNAL_INPUT).to_string();
        let output_string = config.get_field(FIELD_SIGNAL_OUTPUT).to_string();

        write!(
            f,
            "{}",
            match self {
                SignalTypes::Input => input_string.as_str(),
                SignalTypes::Output => output_string.as_str(),
            }
        )
    }
}


// Fonts
pub const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("./fonts/icons.ttf"),
};

pub fn icon(unicode: char) -> Text {
    Text::new(&unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(HorizontalAlignment::Center)
        .size(20)
}

pub fn edit_icon() -> Text {
    icon('\u{F303}')
}

pub fn delete_icon() -> Text {
    icon('\u{F1F8}')
}

impl std::fmt::Display for Operators {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operators::AND => "AND",
                Operators::OR => "OR",
            }
        )
    }
}

impl std::fmt::Display for SubprogramTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let config = unsafe {
            &GLOBAL_CONFIG
        }.as_ref().unwrap();

        let deafult_string = config.get_field(SUBPROGRAM_TYPE_DEFAULT).to_string();
        let critical_string = config.get_field(SUBPROGRAM_TYPE_CRITICAL).to_string();
        let blocked_string = config.get_field(SUBPROGRAM_TYPE_BLOCKED).to_string();

        write!(
            f,
            "{}",
            match self {
                SubprogramTypes::Dflt => deafult_string.as_str(),
                SubprogramTypes::Critical => critical_string.as_str(),
                SubprogramTypes::Blocked => blocked_string.as_str(),
            }
        )
    }
}

impl std::fmt::Display for IOElementStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let config = unsafe {
            &GLOBAL_CONFIG
        }.as_ref().unwrap();

        let active_string = config.get_field(IO_STATE_ACTIVE).to_string();
        let inactive_string = config.get_field(IO_STATE_INACTIVE).to_string();
        let any_string = config.get_field(IO_STATE_ANY).to_string();

        write!(
            f,
            "{}",
            match self {
                IOElementStates::Active => active_string.as_str(),
                IOElementStates::Inactive => inactive_string.as_str(),
                IOElementStates::Any => any_string.as_str(),
            }
        )
    }
}
