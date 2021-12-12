use std::{rc::Rc, cell::RefCell};

use crate::configuration:: {
    FrameTypes, SignalTypes, SubprogramTypes, Operators, IOElementStates,
};

pub static mut IO_CONFIG: Option<Rc<RefCell<IOConfig>>> = None;
pub static mut SUBPROGRAMS_CONFIG: Option<Rc<RefCell<SubprogramConfig>>> = None;
pub static mut CONDTIONS_CONFIG: Option<Rc<RefCell<CondtionsConfig>>> = None;

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
                let parsed = hw.parse::<u8>();
                if parsed.is_ok() {
                    self.hw_address = parsed.unwrap()
                }
            }
            _ => {}
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

    pub fn get_elelment_by_name(
        &self, name: String
    ) -> Option<Rc<RefCell<IOElement>>> {
        let mut element = None;

        for io_element in self.elements.clone() {
            let (io_name, ..) = io_element.borrow().get_data();

            if io_name == name {
                element = Some(io_element.clone());
            }
        }

        element
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
    DeleteElement(FrameTypes),
    IOElementMessage(usize, IOElementMessage),
    StateChanged(IOElementStates),
    IOElementSelected(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IOElementCoditions {
    element: Option<Rc<RefCell<IOElement>>>,
    state: IOElementStates,
    frame_type: FrameTypes,
}

impl IOElementCoditions {
    pub fn new(frame_type: FrameTypes) -> Self {
        IOElementCoditions {
            element: None,
            state: IOElementStates::Any,
            frame_type: frame_type
        }
    }

    pub fn update(&mut self, message: IOElementCoditionsMessage) {
        match message {
            IOElementCoditionsMessage::StateChanged(state) => {
                self.state = state
            },
            IOElementCoditionsMessage::IOElementSelected(name) => {
                let io_config = unsafe {
                    &IO_CONFIG
                }.as_ref().unwrap();

                self.element = io_config.borrow().get_elelment_by_name(name)
            }
            _ => {}
        }
    }

    pub fn get_data(&self) -> (Option<Rc<RefCell<IOElement>>>, IOElementStates, FrameTypes) {
        (self.element.clone(), self.state.clone(), self.frame_type)
    }

}

#[derive(Debug, Clone)]
pub enum SubprogramStepMessage {
    ChangeId(usize),
    DeleteStep,
    AddCondition(FrameTypes),
    DescriptionChanged(String),
    PickConditions(FrameTypes),
    IOElementCoditionsMessage(usize, IOElementCoditionsMessage),
    OperatorSelected(Operators),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubprogramStep {
    id: usize,
    descripton: String,
    merge_operator: Operators,
    state_conditions: Vec<Rc<RefCell<IOElementCoditions>>>,
    control_conditions: Vec<Rc<RefCell<IOElementCoditions>>>,
    pub active_condion: FrameTypes,
}

impl SubprogramStep {
    pub fn new() -> Self {
        SubprogramStep {
            id: 0,
            merge_operator: Operators::AND,
            state_conditions: vec![],
            control_conditions: vec![],
            active_condion: FrameTypes::State,
            descripton: String::new()
        }
    }

    pub fn add_new_conditon(
        &mut self,
        condition :Rc<RefCell<IOElementCoditions>>
    ) {
        let (_, _, frame_type) = condition.as_ref().borrow().get_data();

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

    pub fn get_last_condition(
        &self, frame_type: FrameTypes
    ) -> Rc<RefCell<IOElementCoditions>> {
        match frame_type {
            FrameTypes::State => {
                self.state_conditions.last().unwrap().clone()
            },
            FrameTypes::Control => {
                self.control_conditions.last().unwrap().clone()
            }
        }
    }

    pub fn get_data(&self) -> (
        usize, Operators, Vec<Rc<RefCell<IOElementCoditions>>>,
        Vec<Rc<RefCell<IOElementCoditions>>>, String
    ) {
        (
            self.id,
            self.merge_operator,
            self.state_conditions.clone(),
            self.control_conditions.clone(),
            self.descripton.clone()
        )
    }

    pub fn update(
        &mut self,
        message: SubprogramStepMessage
    ) {
        match message {
            SubprogramStepMessage::ChangeId(id) => {
                self.id = id
            },
            SubprogramStepMessage::AddCondition(frame_type) => {
                self.add_new_conditon(Rc::new(RefCell::new(IOElementCoditions::new(frame_type))))
            }
            SubprogramStepMessage::OperatorSelected(operator) => {
                self.merge_operator = operator
            },
            SubprogramStepMessage::DescriptionChanged(descripton) => {
                self.descripton = descripton
            }
            SubprogramStepMessage::IOElementCoditionsMessage(i, message) => {
                match message {
                    IOElementCoditionsMessage::DeleteElement(frame_type) => {
                        match frame_type {
                            FrameTypes::State => {
                                self.state_conditions.remove(i);
                            },
                            FrameTypes::Control => {
                                self.control_conditions.remove(i);
                            }
                        }
                    },
                    _ => {
                        match self.active_condion {
                            FrameTypes::State => {
                                if let Some(condition) = self.state_conditions.get_mut(i) {
                                    let mut mut_condition = condition.borrow_mut();
                                    mut_condition.update(message);
                                }
                            },
                            FrameTypes::Control => {
                                if let Some(condition) = self.control_conditions.get_mut(i) {
                                    let mut mut_condition = condition.borrow_mut();
                                    mut_condition.update(message);
                                }
                            }
                        }
                    }
                }
            },
            _ => ()
        }
    }
}

#[derive(Debug, Clone)]
pub enum SubprogramMessage {
    AddNewSubprogramStep,
    SubprogramEdit,
    SubprogramDelete,
    SubprogramTypeSelected(SubprogramTypes),
    SubprogramDescrptionChanged(String),
    SubprogramStepMessage(usize, SubprogramStepMessage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subprogram {
    pub address: usize,
    name: String,
    priority_type: SubprogramTypes,
    steps: Vec<Rc<RefCell<SubprogramStep>>>,
    current_step_edit: usize
}

impl Subprogram {
    pub fn new() -> Self {
        Subprogram {
            address: 0,
            name: String::new(),
            priority_type: SubprogramTypes::Dflt,
            steps: vec![],
            current_step_edit: 0
        }
    }

    pub fn get_data(&self) -> (usize, String, SubprogramTypes, Vec<Rc<RefCell<SubprogramStep>>>) {
        (self.address, self.name.clone(), self.priority_type, self.steps.clone())
    }

    pub fn get_current_editable_step_id(&self) -> usize {
        self.current_step_edit
    }

    pub fn get_current_editable_step(&self) -> Rc<RefCell<SubprogramStep>> {
        self.steps.get(self.current_step_edit).unwrap().clone()
    }

    pub fn get_step(&self, id: usize) -> Rc<RefCell<SubprogramStep>> {
        self.steps.get(id).unwrap().clone()
    }

    pub fn get_last_step(&self) -> Rc<RefCell<SubprogramStep>> {
        self.steps.last().unwrap().clone()
    }

    pub fn get_steps_count(&self) -> usize {
        self.steps.len()
    }

    pub fn update(
        &mut self,
        message: SubprogramMessage
    ) {
        match message {
            SubprogramMessage::SubprogramStepMessage(i, message) => {
                self.current_step_edit = i;
                match message {
                    SubprogramStepMessage::DeleteStep => {
                        self.steps.remove(i);

                        for i in 0..self.steps.len() {
                            if let Some(step) = self.steps.get_mut(i) {
                                let mut mut_step = step.borrow_mut();
                                mut_step.update(SubprogramStepMessage::ChangeId(i + 1));
                            }

                        }
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
                ));

                if let Some(step) = self.steps.clone().last_mut() {
                    let mut mut_step = step.borrow_mut();
                    mut_step.update(SubprogramStepMessage::ChangeId(self.steps.len()));
                }
            },
            SubprogramMessage::SubprogramTypeSelected(program_type) => {
                self.priority_type = program_type
            }
            SubprogramMessage::SubprogramDescrptionChanged(description) => {
                self.name = description
            }
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
pub enum SubprogramConfigStetes {
    SubprogramConfigState,
    SubprogramEditState,
    SubprogramStepConditonsPick,
    SubprogramEditDescription,
}

#[derive(Debug, Clone)]
pub struct SubprogramConfig {
    subprograms: Vec<Rc<RefCell<Subprogram>>>,
    current_subprogram_edit: usize,
    last_address: usize,
}

impl SubprogramConfig {
    pub fn new() -> Self {
        SubprogramConfig {
            current_subprogram_edit: 0,
            subprograms: vec![],
            last_address: 0,
        }
    }

    pub fn get_last_subprogram(&self) -> Rc<RefCell<Subprogram>> {
        self.subprograms.last().unwrap().clone()
    }

    pub fn get_current_editable_subprogram(&self) -> Rc<RefCell<Subprogram>> {
        self.subprograms.get(self.current_subprogram_edit).unwrap().clone()
    }

    pub fn get_current_editable_id(&self) -> usize {
        self.current_subprogram_edit
    }

    pub fn get_subprogram(&self, id: usize) -> Rc<RefCell<Subprogram>> {
        self.subprograms.get(id).unwrap().clone()
    }

    pub fn get_last_address(&self) -> usize {
        self.last_address
    }

    pub fn get_subprograms(&self) -> Vec<Rc<RefCell<Subprogram>>> {
        self.subprograms.clone()
    }

    pub fn update_addresses(&mut self) {
        self.last_address = 1;
        for i in 0..self.subprograms.len() {
            if let Some(subprogram) = self.subprograms.get_mut(i) {
                let mut mut_subprogram = subprogram.borrow_mut();
                mut_subprogram.address = self.last_address;
                self.last_address += mut_subprogram.get_steps_count();
            }
        }
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
                    SubprogramMessage::SubprogramEdit => {
                        self.current_subprogram_edit = i;
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
                ));
            },
            _ => (),
        }
        self.update_addresses();
    }

}


#[derive(Debug, Clone)]
pub enum ConditionsConfigElementMessage {
    DeleteCondition,
    AddCondition(FrameTypes),
    PickConditions(FrameTypes),
    IOElementCoditionsMessage(usize, IOElementCoditionsMessage),
    CriticalPicked(bool),
    BlockedPicked(bool),
    DescriptionChanged(String),
    AddressSelected(usize)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionsConfigElement {
    description: String,
    state_conditions: Vec<Rc<RefCell<IOElementCoditions>>>,
    control_conditions: Vec<Rc<RefCell<IOElementCoditions>>>,
    pub active_condion: FrameTypes,
    critical: bool,
    blocked: bool,
    address: usize
}

impl ConditionsConfigElement {
    pub fn new() -> Self {
        ConditionsConfigElement {
            description: String::new(),
            state_conditions: vec![],
            control_conditions: vec![],
            active_condion: FrameTypes::State,
            critical: false,
            blocked: false,
            address: 0
        }
    }

    pub fn add_new_conditon(
        &mut self,
        condition :Rc<RefCell<IOElementCoditions>>
    ) {
        let (_, _, frame_type) = condition.as_ref().borrow().get_data();

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

    pub fn get_last_condition(
        &self, frame_type: FrameTypes
    ) -> Rc<RefCell<IOElementCoditions>> {
        match frame_type {
            FrameTypes::State => {
                self.state_conditions.last().unwrap().clone()
            },
            FrameTypes::Control => {
                self.control_conditions.last().unwrap().clone()
            }
        }
    }

    pub fn get_data(&self) -> (
        String, Vec<Rc<RefCell<IOElementCoditions>>>,
        Vec<Rc<RefCell<IOElementCoditions>>>, bool, bool, usize
    ) {
        (
            self.description.clone(),
            self.state_conditions.clone(),
            self.control_conditions.clone(),
            self.blocked,
            self.critical,
            self.address
        )
    }

    pub fn update(
        &mut self,
        message: ConditionsConfigElementMessage
    ) {
        match message {
            ConditionsConfigElementMessage::AddCondition(frame_type) => {
                self.add_new_conditon(Rc::new(RefCell::new(IOElementCoditions::new(frame_type))))
            }
            ConditionsConfigElementMessage::CriticalPicked(state) => {
                self.critical = state
            },
            ConditionsConfigElementMessage::BlockedPicked(state) => {
                self.blocked = state
            },
            ConditionsConfigElementMessage::DescriptionChanged(new_descr) => {
                self.description = new_descr
            },
            ConditionsConfigElementMessage::AddressSelected(address) => {
                self.address = address
            },
            ConditionsConfigElementMessage::IOElementCoditionsMessage(i, message) => {
                match message {
                    IOElementCoditionsMessage::DeleteElement(frame_type) => {
                        match frame_type {
                            FrameTypes::State => {
                                self.state_conditions.remove(i);
                            },
                            FrameTypes::Control => {
                                self.control_conditions.remove(i);
                            }
                        }
                    },
                    _ => {
                        match self.active_condion {
                            FrameTypes::State => {
                                if let Some(condition) = self.state_conditions.get_mut(i) {
                                    let mut mut_condition = condition.borrow_mut();
                                    mut_condition.update(message);
                                }
                            },
                            FrameTypes::Control => {
                                if let Some(condition) = self.control_conditions.get_mut(i) {
                                    let mut mut_condition = condition.borrow_mut();
                                    mut_condition.update(message);
                                }
                            }
                        }
                    }
                }
            },
            _ => ()
        }
    }
}

#[derive(Debug, Clone)]
pub enum CondtionsConfigStetes {
    CondtionsConfigState,
    IOConditonsPick,
}

#[derive(Debug, Clone)]
pub enum CondtionsConfigMessage {
    AddNewConditons,
    ConditionsConfigElementMessage(usize, ConditionsConfigElementMessage),
}

#[derive(Debug, Clone)]
pub struct CondtionsConfig {
    conditions: Vec<Rc<RefCell<ConditionsConfigElement>>>,
    current_condition_edit: usize,
    pub state: CondtionsConfigStetes,
}

impl CondtionsConfig {
    pub fn new() -> Self {
        CondtionsConfig {
            current_condition_edit: 0,
            conditions: vec![],
            state: CondtionsConfigStetes::CondtionsConfigState
        }
    }

    pub fn get_last_condtions(&self) -> Rc<RefCell<ConditionsConfigElement>> {
        self.conditions.last().unwrap().clone()
    }

    pub fn get_current_editable_subprogram(&self) -> Rc<RefCell<ConditionsConfigElement>> {
        self.conditions.get(self.current_condition_edit).unwrap().clone()
    }

    pub fn get_current_editable_id(&self) -> usize {
        self.current_condition_edit
    }

    pub fn get_conditon(&self, id: usize) -> Rc<RefCell<ConditionsConfigElement>> {
        self.conditions.get(id).unwrap().clone()
    }

    pub fn sort_conditions(&mut self) {
        self.conditions.sort_by(|a, b| a.borrow().address.cmp(&b.borrow().address));
    }

    pub fn get_conditions(&self) -> Vec<Rc<RefCell<ConditionsConfigElement>>> {
        self.conditions.clone()
    }

    pub fn update(
        &mut self,
        message: CondtionsConfigMessage
    ) {
        match message {
            CondtionsConfigMessage::ConditionsConfigElementMessage(i, message) => {
                match message {
                    ConditionsConfigElementMessage::DeleteCondition => {
                        self.conditions.remove(i);
                    },
                    ConditionsConfigElementMessage::PickConditions(_) => {
                        self.current_condition_edit = i;
                    },
                    _ => {
                        if let Some(condition) = self.conditions.get_mut(i) {
                            let mut mut_condition = condition.borrow_mut();
                            mut_condition.update(message);
                        }
                    }
                }
            },
            CondtionsConfigMessage::AddNewConditons => {
                self.conditions.push(Rc::new(
                        RefCell::new(ConditionsConfigElement::new())
                ));
            },
            _ => (),
        }
    }
}

