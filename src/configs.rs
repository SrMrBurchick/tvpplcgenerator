use std::{rc::Rc, cell::RefCell};

use crate::configuration:: {
    FrameTypes, SignalTypes, SubprogramTypes, Operators, IOElementStates,
};

pub static mut IO_CONFIG: Option<Rc<RefCell<IOConfig>>> = None;
pub static mut SUBPROGRAMS_CONFIG: Option<Rc<RefCell<SubprogramConfig>>> = None;

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

#[derive(Debug, Clone)]
pub enum IOElementCoditionsMessage {
    AddNewElement,
    DeleteElement,
    IOElementMessage(usize, IOElementMessage),
    StateChanged(IOElementStates),
    IOElementChanged(),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IOElementCoditions {
    element: Option<Rc<RefCell<IOElement>>>,
    state: IOElementStates,
}

impl IOElementCoditions {
    pub fn new() -> Self {
        IOElementCoditions {
            element: None,
            state: IOElementStates::Any,
        }
    }

    pub fn update(&mut self, message: IOElementCoditionsMessage) {
        match message {
            IOElementCoditionsMessage::StateChanged(state) => {
                self.state = state
            },
            _ => {}
        }
    }

    pub fn get_data(&self) -> (Option<Rc<RefCell<IOElement>>>, IOElementStates) {
        (self.element.clone(), self.state.clone())
    }

}

#[derive(Debug, Clone)]
pub enum SubprogramStepMessage {
    ChangeId(usize),
    DeleteStep,
    EditControlConditions(usize, IOElementCoditionsMessage),
    EditStateConditions(usize, IOElementCoditionsMessage),
    OperatorSelected(Operators),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubprogramStep {
    id: usize,
    merge_operator: Operators,
    state_conditions: Vec<Rc<RefCell<IOElementCoditions>>>,
    control_conditions: Vec<Rc<RefCell<IOElementCoditions>>>,
}

impl SubprogramStep {
    pub fn new() -> Self {
        SubprogramStep {
            id: 0,
            merge_operator: Operators::AND,
            state_conditions: vec![],
            control_conditions: vec![],
        }
    }

    pub fn add_new_conditon(
        &mut self,
        condition :Rc<RefCell<IOElementCoditions>>
    ) {
        let (element, _) = condition.as_ref().borrow().get_data();
        let (_, frame_type, _, _) = element.unwrap().borrow().get_data();

        match frame_type {
            FrameTypes::State => {
                self.state_conditions.push(condition.clone())
            },
            FrameTypes::Control => {
                self.control_conditions.push(condition.clone())
            }
        }
    }

    pub fn get_conditions(
        &self, frame_type: FrameTypes
    ) -> Vec<Rc<RefCell<IOElementCoditions>>> {
        match frame_type {
            FrameTypes::State => {
                self.state_conditions.clone()
            },
            FrameTypes::Control => {
                self.control_conditions.clone()
            }
        }
    }

    pub fn update(
        &mut self,
        message: SubprogramStepMessage
    ) {
        match message {
            SubprogramStepMessage::ChangeId(id) => {
                self.id = id
            },
            SubprogramStepMessage::OperatorSelected(operator) => {
                self.merge_operator = operator
            },
            SubprogramStepMessage::EditStateConditions(i, message) => {
                match message {
                    IOElementCoditionsMessage::DeleteElement => {
                        self.state_conditions.remove(i);
                    },
                    IOElementCoditionsMessage::AddNewElement => {
                        self.state_conditions.push(
                            Rc::new(
                                RefCell::new(IOElementCoditions::new()
                            )));
                    },
                    _ => {
                        if let Some(condition) = self.state_conditions.get_mut(i) {
                            let mut mut_condition = condition.borrow_mut();
                            mut_condition.update(message);
                        }
                    }
                }
            },
            SubprogramStepMessage::EditControlConditions(i, message) => {
                match message {
                    IOElementCoditionsMessage::DeleteElement => {
                        self.control_conditions.remove(i);
                    },
                    IOElementCoditionsMessage::AddNewElement => {
                        self.control_conditions.push(
                            Rc::new(
                                RefCell::new(IOElementCoditions::new()
                            )));
                    },
                    _ => {
                        if let Some(condition) = self.control_conditions.get_mut(i) {
                            let mut mut_condition = condition.borrow_mut();
                            mut_condition.update(message);
                        }
                    }
                }
            },
            SubprogramStepMessage::DeleteStep => ()
        }
    }
}

#[derive(Debug, Clone)]
pub enum SubprogramMessage {
    AddNewSubprogramStep,
    SubprogramEdit,
    SubprogramDelete,
    SubprogramStepMessage(usize, SubprogramStepMessage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subprogram {
    address: usize,
    name: String,
    priority_type: SubprogramTypes,
    steps: Vec<Rc<RefCell<SubprogramStep>>>,
}

impl Subprogram {
    pub fn new() -> Self {
        Subprogram {
            address: 0,
            name: String::new(),
            priority_type: SubprogramTypes::Dflt,
            steps: vec![],
        }
    }

    pub fn get_data(&self) -> (usize, String, SubprogramTypes, Vec<Rc<RefCell<SubprogramStep>>>) {
        (self.address, self.name.clone(), self.priority_type, self.steps.clone())
    }

    pub fn update(
        &mut self,
        message: SubprogramMessage
    ) {
        match message {
            SubprogramMessage::SubprogramStepMessage(i, message) => {
                match message {
                    SubprogramStepMessage::DeleteStep => {
                        self.steps.remove(i);
                    },
                    _ => {
                        if let Some(step) = self.steps.get_mut(i) {
                            let mut mut_step = step.borrow_mut();
                            mut_step.update(message);
                        }
                    }
                }
            },
            SubprogramMessage::AddNewSubprogramStep => {
                self.steps.push(Rc::new(
                        RefCell::new(SubprogramStep::new())
                ))
            },
            _ => (),
        }
    }

}

#[derive(Debug, Clone)]
pub enum SubprogramConfigMessage {
    AddNewSubprogram,
    SubprogramMessage(usize, SubprogramMessage),
}

#[derive(Debug, Clone)]
pub struct SubprogramConfig {
    subprograms: Vec<Rc<RefCell<Subprogram>>>,
}

impl SubprogramConfig {
    pub fn new() -> Self {
        SubprogramConfig {
            subprograms: vec![],
        }
    }

    pub fn get_last_subprogram(&self) -> Rc<RefCell<Subprogram>> {
        self.subprograms.last().unwrap().clone()
    }

    pub fn update(
        &mut self,
        message: SubprogramConfigMessage
    ) {
        match message {
            SubprogramConfigMessage::SubprogramMessage(i, message) => {
                match message {
                    SubprogramMessage::SubprogramDelete => {
                        self.subprograms.remove(i);
                    },
                    _ => {
                        if let Some(subprogram) = self.subprograms.get_mut(i) {
                            let mut mut_subprogram = subprogram.borrow_mut();
                            mut_subprogram.update(message);
                        }
                    }
                }
            },
            SubprogramConfigMessage::AddNewSubprogram => {
                self.subprograms.push(Rc::new(
                        RefCell::new(Subprogram::new())
                ))
            },
            _ => (),
        }
    }

}

