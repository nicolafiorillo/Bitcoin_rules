use std::collections::VecDeque;

use rug::Integer;

use super::{condition_stack::ConditionStack, operation::Operation};

#[derive(Debug)]
pub struct Context {
    operations: Vec<Operation>,
    pub z: Integer,
    operations_length: usize,
    operations_position: usize,

    stack: VecDeque<Operation>,
    alternative_stack: VecDeque<Operation>,

    condition_stack: ConditionStack,
}

#[derive(Debug, PartialEq)]
pub enum ContextError {
    InvalidOpCode,
    NotAnElement,
    NotEnoughElementsInStack,
    DerError,
    UnexpectedEndIf,
    UnexpectedElse,
    ExitByReturn,
    ExitByFailedVerify,
}

impl Context {
    pub fn new(operations: Vec<Operation>, z: Integer) -> Self {
        let operations_length = operations.len();
        let operations_position = 0;

        let stack = VecDeque::<Operation>::new();
        let alternative_stack = VecDeque::<Operation>::new();
        let condition_stack = ConditionStack::new();

        Context {
            operations,
            z,
            operations_length,
            operations_position,
            stack,
            alternative_stack,
            condition_stack,
        }
    }

    pub fn is_over(&self) -> bool {
        self.operations_position >= self.operations_length
    }

    pub fn pop_next(&mut self) -> &Operation {
        assert!(self.operations_position < self.operations_length);

        let current = self.operations_position;
        self.operations_position += 1;

        &self.operations[current]
    }

    pub fn push_element(&mut self, operation: Operation) {
        self.stack.push_front(operation)
    }

    pub fn top_stack(&self) -> &Operation {
        assert!(!self.stack.is_empty());

        self.stack.front().unwrap()
    }

    pub fn pop_element(&mut self) -> Result<Operation, ContextError> {
        // TODO: assert stack is not empty
        match self.stack.pop_front().unwrap() {
            Operation::Element(element) => Ok(Operation::Element(element)),
            op => {
                log::error!("Expected element, found {:?}", op);
                Err(ContextError::NotAnElement)
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        self.stack.len() == 1 && self.stack[0] == Operation::Element(vec![1])
    }

    pub fn has_enough_elements(&self, num: usize) -> bool {
        self.stack.len() >= num
    }

    pub fn has_elements(&self, num: usize) -> bool {
        self.stack.len() == num
    }

    pub fn executing(&self) -> bool {
        self.condition_stack.executing()
    }

    pub fn set_execute(&mut self, value: bool) {
        self.condition_stack.set_execute(value)
    }

    pub fn unset_execute(&mut self) {
        self.condition_stack.unset_execute()
    }

    pub fn toggle_execute(&mut self) {
        self.condition_stack.toggle_execute()
    }

    pub fn in_condition(&self) -> bool {
        self.condition_stack.in_condition()
    }
}
