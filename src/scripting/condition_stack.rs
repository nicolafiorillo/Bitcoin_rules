#[derive(Debug)]
pub struct ConditionStack {
    size: usize,
    false_pos: usize,
}

static EMPTY: usize = usize::MAX;

impl ConditionStack {
    pub fn new() -> Self {
        Self {
            size: 0,
            false_pos: EMPTY,
        }
    }

    pub fn in_condition(&self) -> bool {
        self.size > 0
    }

    pub fn executing(&self) -> bool {
        self.false_pos == EMPTY
    }

    pub fn set_execute(&mut self, value: bool) {
        if self.false_pos == EMPTY && !value {
            self.false_pos = self.size;
        }

        self.size += 1;
    }

    pub fn unset_execute(&mut self) {
        if self.size == 0 {
            panic!("Attempted to pop from empty condition stack");
        }

        self.size -= 1;

        if self.size == self.false_pos {
            self.false_pos = EMPTY;
        }
    }

    pub fn toggle_execute(&mut self) {
        if self.size == 0 {
            panic!("Attempted to toggle empty condition stack");
        }

        if self.false_pos == EMPTY {
            self.false_pos = self.size - 1;
            return;
        }

        if self.size - 1 == self.false_pos {
            self.false_pos = EMPTY;
        }
    }
}
