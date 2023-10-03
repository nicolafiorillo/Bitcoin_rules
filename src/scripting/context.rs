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
    alt_stack: VecDeque<Token>,

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
            alt_stack: alternative_stack,
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

    pub fn alt_stack_pop(&mut self) -> Token {
        assert!(!self.alt_stack.is_empty());

        self.alt_stack.pop_front().unwrap()
    }

    pub fn alt_stack_push(&mut self, token: Token) {
        self.alt_stack.push_front(token)
    }

    // See https://github.com/bitcoin/bitcoin/blob/d096743150fd35578b7ed71ef6bced2341927d43/src/script/interpreter.cpp#L1956C1-L1956C1
    // for reference validation
    /*
       Transactions valid:
           if the top result on the stack is TRUE (noted as {0x01}) or
           any other non-zero value or
           if the stack is empty
       Transactions are invalid:
           if the top value on the stack is FALSE (a zero-length empty value, noted as {}) or
           if script execution is halted explicitly by an operator (such as OP_VERIFY, OP_RETURN, or a conditional terminator such as OP_ENDIF)
    */
    pub fn is_valid(&self) -> bool {
        if self.stack.len() == 0 {
            return true;
        }

        let last = self.stack.len() - 1;
        self.stack[last].as_bool()
    }

    pub fn stack_has_enough_items(&self, num: usize) -> bool {
        self.stack.len() >= num
    }

    pub fn stack_has_items(&self, num: usize) -> bool {
        self.stack.len() == num
    }

    pub fn alt_stack_has_enough_items(&self, num: usize) -> bool {
        self.alt_stack.len() >= num
    }

    pub fn alt_stack_has_items(&self, num: usize) -> bool {
        self.alt_stack.len() == num
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
