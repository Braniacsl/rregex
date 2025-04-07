use std::collections::HashMap;


pub(crate) type StateID = usize;

pub(crate) enum Transition {
    Epsilon,
    Literal(char),
}

pub struct NFA {
    pub(crate) start_state: StateID,
    pub(crate) end_states: Vec<StateID>,
    pub(crate) transitions: HashMap<StateID, Vec<(Transition, StateID)>>,
}

impl NFA {
    pub fn new() -> Self {
        NFA {
            start_state: 0,
            end_states: Vec::new(),
            transitions: HashMap::new(),
        }
    }

    pub(crate) fn add_transition(
        &mut self, 
        from: StateID, 
        transition: Transition,
        to: StateID,
    ){
        self.transitions
            .entry(from)
            .or_insert_with(Vec::new)
            .push((transition, to));
    }

    pub fn literal(c: char) -> Self {
        let mut nfa = NFA::new();
        let start = 0;
        let end = 1;

        nfa.add_transition(
            start, 
            Transition::Literal(c),
            end);
        nfa.start_state = start;
        nfa.end_states.push(end);

        nfa
    }

    pub fn epsilon() -> Self {
        let mut nfa = NFA::new();
        let start = 0;
        let end = 1;

        nfa.add_transition(
            start,
            Transition::Epsilon,
            end
        );
        nfa.start_state = start;
        nfa.end_states.push(end);

        nfa
    }

    pub fn union(nfa1: Self, nfa2: Self) -> Self {
        let mut nfa = NFA::new();
        let start = 0;
        let end = 1;

        nfa.add_transition(
            start,
            Transition::Epsilon,
            nfa1.start_state,
        );

        nfa.add_transition(
            start,
            Transition::Epsilon,
            nfa2.start_state,
        );

        for &end_state in &nfa1.end_states {
            nfa.add_transition(
                end_state,
                Transition::Epsilon,
                end,
            )
        }

        for &end_state in &nfa2.end_states {
            nfa.add_transition(
                end_state,
                Transition::Epsilon,
                end
            )
        }

        nfa.transitions.extend(nfa1.transitions);
        nfa.transitions.extend(nfa2.transitions);

        nfa.start_state = start;
        nfa.end_states.push(end);

        nfa
    }

    pub fn concatenate(nfa1: Self, nfa2: Self) -> Self{
        let mut nfa = NFA::new();
        let start = nfa1.start_state;
        let end = nfa2.end_states;

        for &end_state in &nfa1.end_states {
            nfa.add_transition(
                end_state,
                Transition::Epsilon,
                nfa2.start_state,
            )
        }

        nfa.transitions.extend(nfa1.transitions);
        nfa.transitions.extend(nfa2.transitions);

        nfa.start_state = start;
        nfa.end_states = end;

        nfa
    }

    pub fn kleene_star(nfa1: Self) -> Self {
        let mut nfa = NFA::new();
        let start = 0;
        let end = 1;

        nfa.add_transition(start, Transition::Epsilon, nfa1.start_state);
        nfa.add_transition(start, Transition::Epsilon, end);

        for &end_state in &nfa1.end_states {
            nfa.add_transition(end_state, Transition::Epsilon, nfa1.start_state);
            nfa.add_transition(end_state, Transition::Epsilon, end);
        }

        nfa.transitions.extend(nfa1.transitions);

        nfa.start_state = start;
        nfa.end_states.push(end);

        nfa
    }
}