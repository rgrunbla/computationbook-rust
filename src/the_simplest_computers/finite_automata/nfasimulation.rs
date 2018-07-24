use std::collections::HashSet;
use std::hash::Hash;

use helper::{to_hashset};

use super::farule::{FARule};
use super::nfadesign::{NFADesign};

pub struct NFASimulation<T> {
    nfa_design: NFADesign<T>
}

impl<T: Eq + Clone + Hash> NFASimulation<T> {
    pub fn new(nfa_design: &NFADesign<T>) -> NFASimulation<T> {
        NFASimulation {
            nfa_design: nfa_design.clone()
        }
    }

    pub fn next_state (&self, states: &HashSet<T>, c: char) -> HashSet<T> {
        let mut nfa = self.nfa_design.to_nfa_with_state(states);
        nfa.read_character(c);
        nfa.current_state()
    }

    pub fn rule_for(&self, states: &HashSet<T>) -> Vec<FARule<HashSet<T>>> {
        let alphabet = self.nfa_design.rulebook().alphabet();
        alphabet.iter().map(|&c| FARule::new(states, c, &self.next_state(states, c))).collect()
    }
}

