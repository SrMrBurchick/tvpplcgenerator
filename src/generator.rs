use std::slice::SliceIndex;

use crate::configs::{
    IO_CONFIG, SUBPROGRAMS_CONFIG, CONDTIONS_CONFIG
};
use crate::configuration::language_pack_conastants::{TABLE_SHEET_CONDITIONS, TABLE_CONTENT_DESCRIPTION, TABLE_CONTENT_SENSOR_STATES, TABLE_CONTENT_CONTROL_STATES, TABLE_CONTENT_SIGN_OF_TRANSITION, TABLE_CONTENT_TRASITION_ADDRESS, TABLE_CONTENT_SIGN_OF_BLOCKING, TABLE_SHEET_SUBPROGRAMS, FIELD_ADDRESS, OPERATOR, TABLE_CONTENT_SIGN_OF_FINISH, TABLE_CONTENT_SUBPROGRAM_INITIAL};
use crate::configuration:: {
    GLOBAL_CONFIG, FrameTypes, IOElementStates, Operators,
};

use xlsxwriter::*;

static RESULT_TABLE: &str = "./tpvg_generated_table.xlsx";

static STATE_ACTIVE: &str = "10";
static STATE_INACTIVE: &str = "01";
static STATE_ANY: &str = "00";
static STATE_BLOCKED: &str = "11";

static OPERATOR_AND: &str = "&";
static OPERATOR_OR: &str = "|";

fn fill_condtions_sheet<'a>(
    conditions_sheet: &'a mut Worksheet,
    rotadet_format: &'a Format,
    description_format: &'a Format,
    default_format: &'a Format,
) -> Result<String, XlsxError> {
    let config = unsafe {
        &GLOBAL_CONFIG
    }.as_ref().unwrap();

    let conditionsconfig = unsafe {
        &CONDTIONS_CONFIG
    }.as_ref().unwrap();

    let ioconfig = unsafe {
        &IO_CONFIG
    }.as_ref().unwrap();

    let state_elements = ioconfig.borrow()
        .get_elements_by_frame_type(FrameTypes::State);

    let control_elements = ioconfig.borrow()
        .get_elements_by_frame_type(FrameTypes::Control);

    let conditions_list = conditionsconfig.borrow().get_conditions();

    let description_offset_row: u32 = 2;
    let description_offset_col: u16 = 3;
    let state_elements_offset_col: u16 =
        description_offset_col + state_elements.len() as u16;
    let control_elements_offset_col: u16 =
        state_elements_offset_col + control_elements.len() as u16;
    let transition_sign_offset_col = control_elements_offset_col + 1;
    let address_offset_col = transition_sign_offset_col + 1;
    let blocked_sign_offset_col = address_offset_col + 1;
    let states_description_offset_row: u32 = 1;
    let states_number_offset_row: u32 = 2;

    // Top descripton fields
    conditions_sheet.merge_range(
        0, 0, description_offset_row, description_offset_col,
        config.get_field(TABLE_CONTENT_DESCRIPTION).to_string().as_str(),
        Some(description_format)
    )?;
    conditions_sheet.merge_range(
        0, description_offset_col + 1, 0, state_elements_offset_col,
        config.get_field(TABLE_CONTENT_SENSOR_STATES).to_string().as_str(),
        Some(description_format)
    )?;
    conditions_sheet.merge_range(
        0, state_elements_offset_col + 1, 0, control_elements_offset_col,
        config.get_field(TABLE_CONTENT_CONTROL_STATES).to_string().as_str(),
        Some(description_format)
    )?;
    conditions_sheet.merge_range(
        0, control_elements_offset_col + 1,
        description_offset_row, transition_sign_offset_col,
        config.get_field(TABLE_CONTENT_SIGN_OF_TRANSITION).to_string().as_str(),
        Some(rotadet_format)
    )?;
    conditions_sheet.merge_range(
        0, transition_sign_offset_col + 1,
        description_offset_row, address_offset_col,
        config.get_field(TABLE_CONTENT_TRASITION_ADDRESS).to_string().as_str(),
        Some(rotadet_format)
    )?;
    conditions_sheet.merge_range(
        0, address_offset_col + 1,
        description_offset_row, blocked_sign_offset_col,
        config.get_field(TABLE_CONTENT_SIGN_OF_BLOCKING).to_string().as_str(),
        Some(rotadet_format)
    )?;

    let mut index = 1;
    for state_elemnt in state_elements.clone() {
        let (name, ..) = state_elemnt.borrow().get_data();
        conditions_sheet.write_string(
            states_description_offset_row,
            description_offset_col + index as u16,
            name.as_str(), Some(rotadet_format)
        )?;
        conditions_sheet.write_number(
            states_number_offset_row,
            description_offset_col + index as u16,
            index as f64, Some(default_format)
        )?;

        index += 1;
    }

    index = 1;

    for control_element in control_elements.clone() {
        let (name, ..) = control_element.borrow().get_data();
        conditions_sheet.write_string(
            states_description_offset_row,
            state_elements_offset_col + index as u16,
            name.as_str(), Some(rotadet_format)
        )?;
        conditions_sheet.write_number(
            states_number_offset_row,
            state_elements_offset_col + index as u16,
            index as f64, Some(default_format)
        )?;

        index += 1;
    }

    // Data fields

    index = 1;

    for condition in conditions_list {
        let (description, states, controls, blocked, critical, address) =
            condition.borrow().get_data();
        let mut states_index = 0;

        conditions_sheet.merge_range(
            description_offset_row + index as u32, 0,
            description_offset_row + index as u32, description_offset_col,
            description.as_str(),
            Some(description_format)
        )?;

        for state in state_elements.clone() {
            let mut content = String::new();

            for condition in states.clone() {
                let (io_element, state_type, _) = condition.borrow().get_data();

                if io_element != None {
                    let (name_first, ..) = io_element.unwrap().borrow().get_data();
                    let (name_second, ..) = state.borrow().get_data();

                    if name_first == name_second {
                        match state_type {
                            IOElementStates::Active => {
                                content = String::from(STATE_ACTIVE);
                            },
                            IOElementStates::Inactive => {
                                content = String::from(STATE_INACTIVE);
                            },
                            IOElementStates::Any => {
                                content = String::from(STATE_ANY);
                            }
                        }

                        break;
                    }
                }
            }

            states_index += 1;

            conditions_sheet.write_string(
                description_offset_row + index as u32,
                description_offset_col + states_index as u16,
                content.as_str(), Some(default_format)
            )?;
        }

        states_index = 0;

        for control in control_elements.clone() {
            let mut content = String::new();
            for condition in controls.clone() {
                let (io_element, state_type, _) = condition.borrow().get_data();

                if None != io_element {
                    let (name_first, ..) = io_element.unwrap().borrow().get_data();
                    let (name_second, ..) = control.borrow().get_data();

                    if name_first == name_second {
                        match state_type {
                            IOElementStates::Active => {
                                content = String::from(STATE_ACTIVE);
                            },
                            IOElementStates::Inactive => {
                                content = String::from(STATE_INACTIVE);
                            },
                            IOElementStates::Any => {
                                content = String::from(STATE_ANY);
                            }
                        }

                        break;
                    }
                }
            }

            states_index += 1;

            conditions_sheet.write_string(
                description_offset_row + index as u32,
                state_elements_offset_col + states_index as u16,
                content.as_str(), Some(default_format)
            )?;
        }

        conditions_sheet.write_string(
            description_offset_row + index as u32,
            transition_sign_offset_col,
            if blocked {STATE_ACTIVE} else {""},
            Some(default_format)
        )?;

        conditions_sheet.write_number(
            description_offset_row + index as u32,
            address_offset_col,
            address as f64,
            Some(default_format)
        )?;

        conditions_sheet.write_string(
            description_offset_row + index as u32,
            blocked_sign_offset_col,
            if critical {STATE_ACTIVE} else {""},
            Some(default_format)
        )?;

        index += 1;

    }

    Ok(String::from("Ok"))
}

fn fill_subprograms_sheet<'a>(
    subprograms_sheet: &'a mut Worksheet,
    rotadet_format: &'a Format,
    description_format: &'a Format,
    default_format: &'a Format,
) -> Result<String, XlsxError> {
    let config = unsafe {
        &GLOBAL_CONFIG
    }.as_ref().unwrap();

    let subprogramconfig = unsafe {
        &SUBPROGRAMS_CONFIG
    }.as_ref().unwrap();

    let ioconfig = unsafe {
        &IO_CONFIG
    }.as_ref().unwrap();

    let state_elements = ioconfig.borrow()
        .get_elements_by_frame_type(FrameTypes::State);

    let control_elements = ioconfig.borrow()
        .get_elements_by_frame_type(FrameTypes::Control);

    let subprograms_list = subprogramconfig.borrow().get_subprograms();

    let description_offset_row: u32 = 2;
    let description_offset_col: u16 = 3;
    let address_offset_col = description_offset_col + 1;
    let operator_offset_col = address_offset_col + 1;
    let state_elements_offset_col: u16 =
        operator_offset_col + state_elements.len() as u16;
    let control_elements_offset_col: u16 =
        state_elements_offset_col + control_elements.len() as u16;
    let end_sign_offset_col = control_elements_offset_col + 1;
    let states_description_offset_row: u32 = 1;
    let states_number_offset_row: u32 = 2;
    let subprogram_step_description_offest_col: u16 = 1;

    // Top descripton fields
    subprograms_sheet.merge_range(
        0, 0, description_offset_row, description_offset_col,
        config.get_field(TABLE_CONTENT_DESCRIPTION).to_string().as_str(),
        Some(description_format)
    )?;
    subprograms_sheet.merge_range(
        0, address_offset_col,
        description_offset_row, address_offset_col,
        config.get_field(FIELD_ADDRESS).to_string().as_str(),
        Some(rotadet_format)
    )?;
    subprograms_sheet.merge_range(
        0, operator_offset_col,
        description_offset_row, operator_offset_col,
        config.get_field(OPERATOR).to_string().as_str(),
        Some(description_format)
    )?;
    subprograms_sheet.merge_range(
        0, operator_offset_col + 1, 0, state_elements_offset_col,
        config.get_field(TABLE_CONTENT_SENSOR_STATES).to_string().as_str(),
        Some(description_format)
    )?;
    subprograms_sheet.merge_range(
        0, state_elements_offset_col + 1, 0, control_elements_offset_col,
        config.get_field(TABLE_CONTENT_CONTROL_STATES).to_string().as_str(),
        Some(description_format)
    )?;
    subprograms_sheet.merge_range(
        0, control_elements_offset_col + 1,
        description_offset_row, end_sign_offset_col,
        config.get_field(TABLE_CONTENT_SIGN_OF_FINISH).to_string().as_str(),
        Some(rotadet_format)
    )?;

    let mut index = 1;
    for state_elemnt in state_elements.clone() {
        let (name, ..) = state_elemnt.borrow().get_data();
        subprograms_sheet.write_string(
            states_description_offset_row,
            operator_offset_col + index as u16,
            name.as_str(), Some(rotadet_format)
        )?;
        subprograms_sheet.write_number(
            states_number_offset_row,
            operator_offset_col + index as u16,
            index as f64, Some(default_format)
        )?;

        index += 1;
    }

    index = 1;

    for control_element in control_elements.clone() {
        let (name, ..) = control_element.borrow().get_data();
        subprograms_sheet.write_string(
            states_description_offset_row,
            state_elements_offset_col + index as u16,
            name.as_str(), Some(rotadet_format)
        )?;
        subprograms_sheet.write_number(
            states_number_offset_row,
            state_elements_offset_col + index as u16,
            index as f64, Some(default_format)
        )?;

        index += 1;
    }

    // Data fields

    index = 1;

    subprograms_sheet.merge_range(
        description_offset_row + index as u32, 0,
        description_offset_row + index as u32, description_offset_col,
        config.get_field(TABLE_CONTENT_SUBPROGRAM_INITIAL).to_string().as_str(),
        Some(description_format)
    )?;
    subprograms_sheet.write_number (
        description_offset_row + index as u32, address_offset_col,
        0.,
        Some(default_format)
    )?;
    subprograms_sheet.write_string(
        description_offset_row + index as u32, operator_offset_col,
        OPERATOR_AND,
        Some(default_format)
    )?;

    let mut states_index: usize = 1;

    for _ in state_elements.clone() {
        subprograms_sheet.write_string(
            description_offset_row + index as u32,
            operator_offset_col + states_index as u16,
            "", Some(default_format)
        )?;

        states_index += 1;
    }

    states_index = 1;

    for _ in control_elements.clone() {
        subprograms_sheet.write_string(
            description_offset_row + index as u32,
            state_elements_offset_col + states_index as u16,
            "", Some(default_format)
        )?;

        states_index += 1;
    }

    subprograms_sheet.write_string(
        description_offset_row + index as u32,
        end_sign_offset_col,
        STATE_ACTIVE,
        Some(default_format)
    )?;

    index += 1;

    for subprogram in subprograms_list {
        let (address, description, _, steps) =
            subprogram.borrow().get_data();
        let mut subprogram_index = 0;

        subprograms_sheet.merge_range(
            description_offset_row + index as u32, 0,
            description_offset_row + (index + steps.len() - 1) as u32, 0,
            description.as_str(),
            Some(rotadet_format)
        )?;

        for step in steps {
            let mut content = String::new();
            let (_, operator, states, controls, description) = step.borrow().get_data();

            states_index = 0;

            subprograms_sheet.merge_range(
                description_offset_row + index as u32, subprogram_step_description_offest_col,
                description_offset_row + index as u32, description_offset_col,
                description.as_str(),
                Some(default_format)
            )?;

            subprograms_sheet.write_number (
                description_offset_row + index as u32, address_offset_col,
                (address + subprogram_index) as f64,
                Some(default_format)
            )?;

            subprograms_sheet.write_string(
                description_offset_row + index as u32, operator_offset_col,
                if operator == Operators::AND {OPERATOR_AND}else{OPERATOR_OR},
                Some(default_format)
            )?;

            for state in state_elements.clone() {
                content = String::new();
                for condition in states.clone() {
                    let (io_element, state_type, _) = condition.borrow().get_data();

                    if None != io_element {
                        let (name_first, ..) = state.borrow().get_data();
                        let (name_second, ..) = io_element.unwrap().borrow().get_data();

                        if name_first == name_second {
                            match state_type {
                                IOElementStates::Active => {
                                    content = String::from(STATE_ACTIVE);
                                },
                                IOElementStates::Inactive => {
                                    content = String::from(STATE_INACTIVE);
                                },
                                IOElementStates::Any => {
                                    content = String::from(STATE_ANY);
                                }
                            }

                            break;
                        }
                    }
                }

                states_index += 1;

                subprograms_sheet.write_string(
                    description_offset_row + index as u32,
                    operator_offset_col + states_index as u16,
                    content.as_str(), Some(default_format)
                )?;
            }

            states_index = 0;

            for control in control_elements.clone() {
                content = String::new();
                for condition in controls.clone() {
                    let (io_element, state_type, _) = condition.borrow().get_data();

                    if None != io_element {
                        let (name_first, ..) = control.borrow().get_data();
                        let (name_second, ..) = io_element.unwrap().borrow().get_data();

                        if name_first == name_second {
                            match state_type {
                                IOElementStates::Active => {
                                    content = String::from(STATE_ACTIVE);
                                },
                                IOElementStates::Inactive => {
                                    content = String::from(STATE_INACTIVE);
                                },
                                IOElementStates::Any => {
                                    content = String::from(STATE_ANY);
                                }
                            }
                        }
                    }
                }

                states_index += 1;

                subprograms_sheet.write_string(
                    description_offset_row + index as u32,
                    state_elements_offset_col + states_index as u16,
                    content.as_str(), Some(default_format)
                )?;
            }
            subprogram_index += 1;
            index += 1;
        }

        subprograms_sheet.write_string(
            description_offset_row + (index - 1) as u32,
            end_sign_offset_col,
            STATE_ACTIVE,
            Some(default_format)
        )?;
    }

    Ok(String::from("Ok"))
}

pub fn generate_tables() -> Result<String, XlsxError>{
    let workbook = Workbook::new(RESULT_TABLE);
    let format_rotated_description = workbook.add_format()
        .set_align(FormatAlignment::Center)
        .set_align(FormatAlignment::VerticalCenter)
        .set_rotation(90)
        .set_border_top(FormatBorder::Double)
        .set_border_left(FormatBorder::Double)
        .set_border_right(FormatBorder::Double)
        .set_border_bottom(FormatBorder::Double);
    let format_description = workbook.add_format()
        .set_align(FormatAlignment::Center)
        .set_align(FormatAlignment::VerticalCenter)
        .set_border_top(FormatBorder::Double)
        .set_border_left(FormatBorder::Double)
        .set_border_right(FormatBorder::Double)
        .set_border_bottom(FormatBorder::Double);
    let format_default = workbook.add_format()
        .set_align(FormatAlignment::Center)
        .set_align(FormatAlignment::VerticalCenter)
        .set_border_top(FormatBorder::Thin)
        .set_border_left(FormatBorder::Thin)
        .set_border_right(FormatBorder::Thin)
        .set_border_bottom(FormatBorder::Thin);


    let config = unsafe {
        &GLOBAL_CONFIG
    }.as_ref().unwrap();

    let mut condtions_sheet = workbook.add_worksheet(
        Some(config.get_field(TABLE_SHEET_CONDITIONS).to_string().as_str())
    )?;

    fill_condtions_sheet(
        &mut condtions_sheet,
        &format_rotated_description,
        &format_description,
        &format_default
    )?;

    let mut subprograms_sheet = workbook.add_worksheet(
        Some(config.get_field(TABLE_SHEET_SUBPROGRAMS).to_string().as_str())
    )?;

    fill_subprograms_sheet(
        &mut subprograms_sheet,
        &format_rotated_description,
        &format_description,
        &format_default
    )?;

    workbook.close()?;

    Ok(String::from("Ok"))
}
