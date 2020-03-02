use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::cell::RefCell;
use std::rc::Rc;
use serde::*;

use crate::map_as_pairs as map_as_pairs;
use crate::compound_state::*;
use crate::connection_to_compound_state::Connection;
use crate::automata::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AutomataDFA {
    #[serde(with = "map_as_pairs")]
    pub states_connections: BTreeMap<Rc<CompoundState>, BTreeSet<Connection>>,
    pub start_state: Option<Rc<CompoundState>>,
    pub end_states: RefCell<BTreeSet<Rc<CompoundState>>>,
    pub automata_type: AutomataType,
    pub alphabet: String,
    counter: u32,
}

impl Automata for AutomataDFA {
    fn get_type(&self) -> AutomataType {
        return self.automata_type;
    }
}

impl AutomataDFA {
    pub fn new(automata_type: AutomataType) -> AutomataDFA {
        AutomataDFA{
            states_connections: BTreeMap::new(),
            start_state: None,
            end_states: RefCell::new(BTreeSet::new()),
            automata_type:  match automata_type {
                AutomataType::AutomataMinimizedDFA => AutomataType::AutomataMinimizedDFA,
                AutomataType::AutomataDFA | _ => AutomataType::AutomataDFA,
            },
            alphabet: String::new(),
            counter: 0,
        }
    }

    pub fn add_state(&mut self, states: BTreeSet<Rc<crate::state::State>>) -> Rc<CompoundState> {
        return self.add_state_maybe_present(states).0
    }

    pub fn add_state_maybe_present(&mut self, states: BTreeSet<Rc<crate::state::State>>) -> (Rc<CompoundState>, bool) {
        let state = Rc::new(CompoundState::new_init(states));
        self.counter+=1;

        if self.states_connections.contains_key(&state.clone()) {
            return (state, false);
        }
        else {
            self.states_connections.insert(state.clone(), BTreeSet::new());
            return (state, true);
        }
    }

    pub fn add_connection(&mut self, connection: (Rc<CompoundState>, Connection)){
        skip_fail!(self.states_connections.get_mut(&connection.0), Option<_>)
                                          .insert(connection.1.clone());    
    }

    pub fn set_begin_state(&mut self, state: Rc<CompoundState>) {
        self.start_state = Some(state);
    }

    pub fn set_end_states(&mut self, states: BTreeSet<Rc<CompoundState>>) {
        self.end_states = RefCell::new(states);
    }

    pub fn transition_to_minimized(&mut self) {
        //assume all states are connected to start state

        let mut groups = vec![];
        let (mut first_group, mut end_group) = (
                                        Vec::with_capacity(self.states_connections.len()), 
                                        Vec::with_capacity(self.states_connections.keys().len() - self.end_states.borrow().len())
                                       );
         
        self.states_connections.keys().for_each(|compound_state|
                                                    if self.end_states.borrow().contains(&compound_state.clone()) {
                                                        end_group.push(compound_state.clone());
                                                    }
                                                    else {
                                                        first_group.push(compound_state.clone());
                                                    }
                                                );
        if !first_group.is_empty(){                                     
            groups.push(first_group);
        }
        groups.push(end_group);

        while self.split(&mut groups){}

        self.states_connections.values_mut().for_each(
            |set|{
                let mut values_to_remove = vec![];

                for tuple in set.iter(){
                    for group in groups.iter() {
                        if group.len() > 1{
                            if group.iter().any(|compound_state| 
                                compound_state.states == (tuple.0).states 
                            ) {//value needs update if true
                                values_to_remove.push((tuple.clone(), group.to_vec()));
                            }
                        }
                    }
                }

                for tuple in values_to_remove {
                    set.remove(&tuple.0);
                    set.insert((Rc::new(CompoundState::new_from_compound(tuple.1)), (tuple.0).1));
                }
            }
        );

        for group in groups.iter() {//insert the new state
            if group.len() > 1 {
                let mut connections = BTreeSet::new();
                let mut is_end = false;
                let mut is_start = false;
                for compound_state in group{
                    connections = skip_fail_loop!{self.states_connections.remove(&compound_state.clone()), Option<_>};      
                    
                    if let Some(start_state) = &self.start_state { 
                        if start_state.states == compound_state.states {
                            is_start = true;
                        }
                    }
                    if self.end_states.borrow().iter().any(|cstate| cstate.states == compound_state.states){
                        is_end = true;
                    } 
                }

                let new_state = Rc::new(CompoundState::new_from_compound(group.to_vec()));
                self.states_connections.insert(new_state.clone(), connections);
                if is_start{
                    self.start_state = Some(new_state.clone());
                }
                if is_end{
                    self.end_states.borrow_mut().insert(new_state.clone());
                }
            }
        }

        self.automata_type = AutomataType::AutomataMinimizedDFA;
    }

    fn split (&mut self, groups_arg:&mut Vec<Vec<Rc<CompoundState>>>) -> bool {
       
        let mut new_groups: Vec<Vec<Rc<CompoundState>>> = Vec::with_capacity(groups_arg.len());

        for group in groups_arg.iter(){
            new_groups.push(Vec::with_capacity(group.len()));
        }

        let mut changes =  false;
        for (group_index, group) in groups_arg.iter().enumerate() {    
            if group.len() > 0 {
                new_groups[group_index].push(group[0].clone());
            }
            for state in group.iter().skip(1) {
                if !self.is_same_group(group[0].clone(), state.clone(), groups_arg){//needs to be put in a new group
                    changes = true;
                    
                    for vec_of_states in new_groups.iter_mut() {//found group for this state
                        if vec_of_states.len() > 0  && self.is_same_group(vec_of_states[0].clone(), state.clone(), groups_arg){
                            if !(self.end_states.borrow().contains(&state.clone()) ^ 
                                 self.end_states.borrow().contains(&vec_of_states[0].clone())) {//only if both are in end state or are not in end state
                                    vec_of_states.push(state.clone());
                                    break;
                            }
                        }
                    }

                   //not found group for temp_state
                    new_groups.push(vec![state.clone()]);                   
                    
                }
                else {
                    new_groups[group_index].push(state.clone());
                }
            }
        }
        *groups_arg = new_groups;

        changes
    }
    
    fn is_same_group<'a>(&self, state: Rc<CompoundState>, original_state: Rc<CompoundState>, groups: &'a Vec<Vec<Rc<CompoundState>>>) -> bool{
        return self.states_connections.get(&state.clone())
                                      .unwrap()
                                      .iter()
                                      .all(|connection| 
                                        self.states_connections.get(&original_state.clone())
                                        .unwrap()
                                        .iter()
                                        .any(|connection_original_state|{
                                            let a = self.get_group(connection_original_state.0.clone(), groups);
                                            let b = self.get_group(connection.0.clone(), groups);
                                                connection_original_state.1 == connection.1 && a == b
                                        }                                         
                                        )
                                      )
    }
    
    fn get_group<'a>(&self, state: Rc<CompoundState>, groups: &'a Vec<Vec<Rc<CompoundState>>>) -> Option<usize>{
        for index in 0..groups.len(){
            if groups[index].contains(&state.clone()) {
                return Some(index);
            }
        }
        None
    }

}