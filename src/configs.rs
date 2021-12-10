use std::{rc::Rc, cell::RefCell};

use crate::configuration:: {
    FrameTypes, SignalTypes, SubprogramTypes, Operators, IOElementStates,
};

pub static mut IO_CONFIG: Option<Rc<RefCell<IOConfig>>> = None;

#[derive(Debug, Clone)]
pub enum IOElementMessage {
    NameInputChanged(String),
    FrameTypeSelected(FrameTypes),
    SignalTypeSelected(SignalTypes),
    HwSelected(String),
    DeleteElement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IOElement {
    name: String,
    frame_type: FrameTypes,
    signal_type: SignalTypes,
    hw_address: u8,
}

impl IOElement {
    pub fn new() -> Self {
        IOElement {
            name: String::new(),
            frame_type: FrameTypes::State,
            signal_type: SignalTypes::Input,
            hw_address: 0
        }
    }

    pub fn update(&mut self, message: IOElementMessage) {
        match message {
            IOElementMessage::FrameTypeSelected(frame_type) => {
                self.frame_type = frame_type
            },
            IOElementMessage::NameInputChanged(name) => {
                self.name = name
            },
            IOElementMessage::SignalTypeSelected(signal_type) => {
                self.signal_type = signal_type
            },
            IOElementMessage::HwSelected(hw) => {
                self.hw_address = hw.parse().unwrap()
            }
            IOElementMessage::DeleteElement => {}
        }
    }

    pub fn get_data(&self) -> (String, FrameTypes, SignalTypes, u8) {
        (self.name.clone(), self.frame_type, self.signal_type, self.hw_address)
    }
}

#[derive(Debug, Clone)]
pub enum IOConfigMessage {
    AddNewElement,
    IOElementMessage(usize, IOElementMessage),
}

#[derive(Debug, Clone)]
pub struct IOConfig {
    elements: Vec<Rc<RefCell<IOElement>>>,
}

impl IOConfig {
    pub fn new() -> Self {
        IOConfig {
            elements: vec![]
        }
    }

    pub fn add_new_element(&mut self, element :IOElement) {
        self.elements.push(Rc::new(RefCell::new(element)))
    }

    pub fn get_elements_by_frame_type(
        &self, frame_type: FrameTypes
    ) -> Vec<Rc<RefCell<IOElement>>> {
        let mut elements: Vec<Rc<RefCell<IOElement>>> = vec![];

        for element in &self.elements {
            if element.borrow().frame_type == frame_type {
                elements.push(element.clone());
            }
        }

        elements
    }

    pub fn get_elements_by_signal_type(
        &self, signal_type: SignalTypes
    ) -> Vec<Rc<RefCell<IOElement>>> {
        let mut elements: Vec<Rc<RefCell<IOElement>>> = vec![];

        for element in &self.elements {
            if element.borrow().signal_type == signal_type {
                elements.push(element.clone());
            }
        }

        elements
    }

    pub fn get_all_elelments(&self) -> Vec<Rc<RefCell<IOElement>>> {
        self.elements.clone()
    }

    pub fn get_elemnt_by_id(&self, id: usize) -> Rc<RefCell<IOElement>> {
        self.elements[id].clone()
    }

    pub fn get_last_element(&self) -> Rc<RefCell<IOElement>> {
        self.elements.last().unwrap().clone()
    }

    pub fn update(
        &mut self,
        message: IOConfigMessage
    ) {
        match message {
            IOConfigMessage::IOElementMessage(i, message) => {
                match message {
                    IOElementMessage::DeleteElement => {
                        self.elements.remove(i);
                    },
                    _ => {
                        if let Some(element) = self.elements.get_mut(i) {
                            let mut mut_element = element.borrow_mut();
                            mut_element.update(message);
                        }
                    }
                }
            },
            IOConfigMessage::AddNewElement => {
                self.add_new_element(IOElement::new())
            }
        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IOElementCoditions {
    element: Option<IOElement>,
    state: IOElementStates,
}

impl IOElementCoditions {
    pub fn new() -> Self {
        IOElementCoditions {
            element: None,
            state: IOElementStates::Any,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubprogramStep {
    id: usize,
    merge_operator: Operators,
    state_conditions: Vec<IOElementCoditions>,
    control_conditions: Vec<IOElementCoditions>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subprogram {
    addres: usize,
    name: String,
    priority_type: SubprogramTypes,
    steps: Vec<SubprogramStep>,
}

#[derive(Debug, Clone)]
pub struct SubprogramConfig {
    subprograms: Vec<Subprogram>,
}

