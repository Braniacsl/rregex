use std::{collections::{HashSet, VecDeque}};
use crate::nfa::{NFA, StateID, Transition};

pub struct Matcher {
    nfa: NFA
}

impl Matcher {
    pub fn new(nfa: NFA) -> Self {
        Matcher { nfa }
    }

    pub fn matches(&self, input: &str) -> bool {
        let mut current_states = self.epsilon_closure(
            &HashSet::from([self.nfa.start_state]));

        for ch in input.chars() {
            let mut next_states = HashSet::new();

            for &state in &current_states {
                if let Some(transitions) = self.nfa.transitions.get(&state) {
                    for (transition, next_state) in transitions {
                        match transition {
                            Transition::Literal(c) if *c == ch => {
                                next_states.insert(*next_state);
                            },
                            _ => {},
                        }
                    }
                }
            }

            current_states = self.epsilon_closure(&next_states);

            if current_states.is_empty() {
                return false;
            }
        }

        current_states.iter().any(|state| self.nfa.end_states.contains(state))
    }

    pub fn epsilon_closure(&self, states: &HashSet<StateID>) -> HashSet<StateID> {
        let mut closure = states.clone();
        let mut stack = VecDeque::from_iter(states.iter().clone());

        while let Some(state) = stack.pop_front() {
            if let Some(transitions) = self.nfa.transitions.get(&state) {
                for (transition, next_state) in transitions {
                    if matches!(transition, Transition::Epsilon) && !closure.contains(next_state) {
                        closure.insert(*next_state);
                        stack.push_back(next_state);
                    }
                }
            }
        }

        closure
    }
}