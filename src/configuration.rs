use std::{fs::File, io::BufReader, io::Read, collections::HashMap, borrow::Borrow};
use json::{self, JsonValue};

use self::language_pack_conastants::DEFAULT;

pub static DEFAULT_LANGUAGE_PACK: &str = "./src/languages/US.json";

pub mod language_pack_conastants {
    pub static BUTTON_ADD_NEW: &str = "BUTTON_ADD_NEW";
    pub static BUTTON_BACK: &str = "BUTTON_BACK";
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
}

#[derive(Debug, PartialEq, Eq)]
pub struct LanguagePack {
    file_path: String,
    content_str: String,
    content_json: JsonValue
}

impl LanguagePack {
    pub fn new() -> Self {
        LanguagePack {
            file_path: String::new(),
            content_str: String::new(),
            content_json: JsonValue::Null
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
    pub const FONT_SIZE: u16 = 14;
    pub const DEFAULT_PADDING: u16 = 10;
    pub const DEFAULT_SPACING: u16 = 20;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    active_language_pack: String,
    languages_pack: HashMap<String, LanguagePack>
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
            active_language_pack: String::from(DEFAULT)
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
