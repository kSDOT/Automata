use std::collections::BTreeSet;

use std::rc::Rc;
use serde::*;
use crate::state::*;

#[derive(Debug, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
pub struct CompoundState {
    pub states: BTreeSet<Rc<State>>,
}

impl CompoundState {
    pub fn new() -> CompoundState {
        CompoundState{
            states: BTreeSet::new(),
        }
    }

    pub fn new_init(collection: BTreeSet<Rc<State>>) -> CompoundState {
        CompoundState{
            states: collection,
        }
    }

    pub fn new_from_compound(collection: Vec<Rc<CompoundState>>) -> CompoundState {
        CompoundState{
            states: collection.into_iter().map(|compound_state| compound_state.states.clone()).flatten().collect(),
        }
    }


    pub fn add_state(&mut self, state: Rc<State>){
        self.states.insert(state);
    }

    pub fn name(&self) -> String {
        let mut name = String::from("[");
        for state in &self.states{
            name += &state.name;
            name += ", ";
        }
        name.pop();
        name.pop();
        name.push(']');
        name
    }

    pub fn id(&self) -> String {
        let mut id = String::from("[");
        for state in &self.states{
            id += &state.id().to_string();
        } 
        id.push(']');
        id
    }
} 

impl std::fmt::Display for CompoundState {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for state in &self.states {
            write!(f, "id: \"{}\", name: \"{}\"", state.id(), state.name)?;
        }
        Ok(())
    } 

}
