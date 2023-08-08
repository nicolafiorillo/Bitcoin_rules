use std::collections::VecDeque;

use rug::Integer;

use super::{condition_stack::ConditionStack, token::Token};

#[derive(Debug)]
pub struct Context {
    script_tokens: Vec<Token>,
    pub z: Integer,
    script_tokens_length: usize,
    script_tokens_position: usize,

    stack: VecDeque<Token>,
    alternative_stack: VecDeque<Token>,

    condition_stack: ConditionStack,
}

#[derive(Debug, PartialEq)]
pub enum ContextError {
    InvalidOpCode,
    NotAnElement,
    NotEnoughItemsInStack,
    DerError,
    UnexpectedEndIf,
    UnexpectedElse,
    ExitByReturn,
    ExitByFailedVerify,
    DeprecatedOpCode,
    ExitByReserved,
}

impl Context {
    pub fn new(script_tokens: Vec<Token>, z: Integer) -> Self {
        let script_tokens_length = script_tokens.len();
        let script_tokens_position = 0;

        let stack = VecDeque::<Token>::new();
        let alternative_stack = VecDeque::<Token>::new();
        let condition_stack = ConditionStack::new();

        Context {
            script_tokens,
            z,
            script_tokens_length,
            script_tokens_position,
            stack,
            alternative_stack,
            condition_stack,
        }
    }

    pub fn tokens_are_over(&self) -> bool {
        self.script_tokens_position >= self.script_tokens_length
    }

    pub fn next_token(&mut self) -> &Token {
        assert!(self.script_tokens_position < self.script_tokens_length);

        let current = self.script_tokens_position;
        self.script_tokens_position += 1;

        &self.script_tokens[current]
    }

    pub fn stack_push(&mut self, token: Token) {
        self.stack.push_front(token)
    }

    pub fn top_stack(&self) -> &Token {
        assert!(!self.stack.is_empty());

        self.stack.front().unwrap()
    }

    pub fn stack_pop(&mut self) -> Token {
        assert!(!self.stack.is_empty());

        self.stack.pop_front().unwrap()
    }

    pub fn stack_pop_as_element(&mut self) -> Result<Token, ContextError> {
        assert!(!self.stack.is_empty());

        match self.stack.pop_front().unwrap() {
            Token::Element(element) => Ok(Token::Element(element)),
            op => {
                log::error!("Expected element, found {:?}", op);
                Err(ContextError::NotAnElement)
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        self.stack.len() == 1 && self.stack[0] == Token::Element(vec![1])
    }

    pub fn stack_has_enough_items(&self, num: usize) -> bool {
        self.stack.len() >= num
    }

    pub fn stack_has_items(&self, num: usize) -> bool {
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
