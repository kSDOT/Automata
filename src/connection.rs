use crate::state::State as State;
use std::rc::Rc;
use serde::*;

#[derive(Debug, Clone)]
#[derive(Hash, Serialize, Deserialize)]
pub enum Connection {
    EMove(Rc<State>),
    Value(Rc<State>, char),
}

use Connection::*;

impl PartialEq for Connection {
    fn eq(&self, other:  &Connection) -> bool {
        match (self, other) {
            (Value(state, value), Value(other_state, other_value)) => state.eq(other_state) && value.eq(other_value),
            (EMove(state), EMove(other_state)) => state.eq(other_state), 
            _ => false,
        }
    }
}

impl Eq for Connection {}

impl Ord for Connection {
    fn cmp(&self, other:  &Connection) -> std::cmp::Ordering {
        match (self, other) { //order: compare by state, then by value; if value is lacking it takes priority over emove
            (Value(state, value), Value(other_state, other_value)) => {
                state.cmp(other_state).then_with(||value.cmp(other_value))
            },
            (EMove(state), EMove(other_state)) => state.cmp(other_state),
            (Value(state, _), EMove(other_state)) => state.cmp(other_state).then(std::cmp::Ordering::Greater), 
            (EMove(state), Value(other_state, _)) => state.cmp(other_state).then(std::cmp::Ordering::Less), 
        }
    }
}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other:  &Connection) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
