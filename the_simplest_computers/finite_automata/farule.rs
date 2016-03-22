use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub struct FARule {
    state: u32,
    character: char,
    next_state: u32,
}

impl FARule {
    pub fn new(state: u32, character: char, next_state: u32) -> Self {
        FARule{state: state, character: character, next_state: next_state}
    }

    pub fn applies_to(&self, state: u32, character: char) -> bool {
        self.state == state && self.character == character
    }

    pub fn follow(&self) -> u32 {
        self.next_state
    }
}

impl Display for FARule {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "FARule {} --{}--> {}", self.state, self.character, self.next_state)
    }
}
