use std::collections::VecDeque;

use rug::Integer;

use super::operation::Operation;

#[derive(Debug)]
pub struct Context {
    operations: Vec<Operation>,
    z: Integer,
    operations_length: usize,
    operations_position: usize,

    stack: VecDeque<Operation>,
    alternative_stack: VecDeque<Operation>,
}

#[derive(Debug)]
pub enum ContextError {
    NotAnElement,
    NotEnoughElementsInStack,
}

impl Context {
    pub fn new(operations: Vec<Operation>, z: Integer) -> Self {
        let operations_length = operations.len();
        let operations_position = 0;

        let stack = VecDeque::<Operation>::new();
        let alternative_stack = VecDeque::<Operation>::new();

        Context {
            operations,
            z,
            operations_length,
            operations_position,
            stack,
            alternative_stack,
        }
    }

    pub fn is_over(&self) -> bool {
        self.operations_position >= self.operations_length
    }

    pub fn next(&mut self) -> &Operation {
        let current = self.operations_position;
        self.operations_position += 1;

        &self.operations[current]
    }

    pub fn push_on_stack(&mut self, operation: Operation) {
        self.stack.push_front(operation)
    }

    pub fn pop_element_from_stack(&mut self) -> Result<Operation, ContextError> {
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

    pub fn stack_has_at_least_two_elements(&self) -> bool {
        self.stack.len() >= 2
    }

    pub fn z(&self) -> Integer {
        self.z.clone()
    }
}
