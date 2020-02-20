use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::cell::RefCell;
use std::rc::Rc;
use serde::*;

use crate::map_as_pairs as map_as_pairs;
use crate::automata_dfa::*;
use crate::connection::*;
use crate::automata::*;
use crate::state::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AutomataNFA {
    #[serde(with = "map_as_pairs")]
    pub states_connections: BTreeMap<Rc<State>, BTreeSet<Connection>>,
    pub start_state: Option<Rc<State>>,
    pub end_states: RefCell<BTreeSet<Rc<State>>>,
    pub automata_type: AutomataType,
    pub alphabet: String,
    counter: u32,
}

impl Automata for AutomataNFA {
    fn get_type(&self) -> AutomataType {
        return self.automata_type;
    }
}

impl AutomataNFA {
    pub fn new(automata_type: AutomataType) -> AutomataNFA {
        AutomataNFA {
            states_connections: BTreeMap::new(),
            start_state: None,
            end_states: RefCell::new(BTreeSet::new()),
            automata_type: match automata_type {
                AutomataType::AutomataEClosure => AutomataType::AutomataEClosure,
                AutomataType::AutomataNFA | _ => AutomataType::AutomataNFA,
            },
            alphabet: String::new(),
            counter: 0,
        }
    }

    pub fn add_state(&mut self, name: &str) -> Rc<State> {
        let state = Rc::new(State::new(self.counter, name));
        self.counter+=1;
        self.states_connections.insert(state.clone(), BTreeSet::new());
        return state;
    }

    pub fn add_connection(&mut self, connection: (Rc<State>, Connection)){
        skip_fail!(self.states_connections.get_mut(&connection.0), Option<_>)
                                          .insert(connection.1.clone());

        let symbol = match connection.1 {
            Connection::EMove(_) => return,
            Connection::Value(_, symbol) => symbol,
        };

        if !self.alphabet.contains(symbol) {
         self.alphabet.push(symbol);
        }
    }

    pub fn set_begin_state(&mut self, state: Rc<State>) {
        self.start_state = Some(state);
    }

    pub fn set_end_states(&mut self, states: BTreeSet<Rc<State>>) {
        self.end_states = RefCell::new(states);
    }

    pub fn transition_to_nfa(&mut self) {
        //get all emoves
        let mut seen = HashSet::new();
        let start_state = skip_fail!{&self.start_state, Option<_>};
        let mut all_emoves: HashMap<Rc<State>, BTreeSet<Rc<State>>> = HashMap::new();

        self.get_all_emoves(start_state.clone(), &mut seen, &mut all_emoves);
        for (key, _) in &self.states_connections {//all others
            if !seen.contains(key) {
                 self.get_all_emoves(key.clone(), &mut seen, &mut all_emoves);     
            }
        }

        
        let mut new_values: HashMap<Rc<State>, Vec<Connection>> = HashMap::new();
        
        for (key, _) in &self.states_connections {
           
            for emove in all_emoves.get(&key.clone()).unwrap() {
                if self.end_states.borrow().contains(&emove.clone()){//promote to end_state
                    self.end_states.borrow_mut().insert(key.clone());
                }

                for value_connection in self.states_connections.get(&emove.clone()).unwrap() {
                    if let Connection::Value(destination, value) = value_connection {
                        if new_values.contains_key(&key.clone()) {
                            new_values.get_mut(&key.clone())
                                      .unwrap()
                                      .extend(
                                          all_emoves.get(&destination.clone())
                                                    .unwrap()
                                                    .clone()
                                                    .iter()
                                                    .map(|state| Connection::Value(state.clone(), *value))
                                             );                        
                        }
                        else {
                            new_values.insert(key.clone(), all_emoves.get(&destination.clone())
                                                                     .unwrap()
                                                                     .clone()
                                                                     .iter()
                                                                     .map(|state| Connection::Value(state.clone(), *value))
                                                                     .collect()                                              
                                            );
                        }
                    }
                }
            } 
        }

        self.states_connections.extend(new_values.into_iter()
                                                 .map(|(key, values)| (key, values.into_iter()
                                                                                  .collect()
                                                                       )
                                                                    )
                                      );

        self.automata_type = AutomataType::AutomataNFA;
        
    }

    pub fn get_all_emoves(&self, state: Rc<State>, mut seen: &mut HashSet<Rc<State>>, mut all_emoves: &mut HashMap<Rc<State>, BTreeSet<Rc<State>>>) -> Vec<Rc<State>> {
        let mut stuck_in_loop_previous = vec![];
        let mut stuck_in_loop_now = vec![];

        let mut current_emoves: BTreeSet<Rc<State>> = BTreeSet::new();//#self_referential care
        current_emoves.insert(state.clone());

        seen.insert(state.clone());
        for connection in self.states_connections.get(&state.clone()).unwrap() {
           match connection{
              Connection::EMove(connected_state) => {

                    if !seen.contains(&connected_state.clone()){

                        stuck_in_loop_previous = self.get_all_emoves(connected_state.clone(), &mut seen, &mut all_emoves);
                        current_emoves.extend(all_emoves.get(&connected_state.clone()).unwrap().clone());
                    }
                    else {
                        if all_emoves.contains_key(&connected_state.clone()) {
                            current_emoves.extend(all_emoves.get(&connected_state.clone()).unwrap().clone());
                        }
                        else { //we are stuck in a cycle, q0 -> EMOVE(q1), q1 -> EMOVE(q0)
                            stuck_in_loop_now.push(state.clone());
                        }
                    }                   
              },
              _ =>  ()
           }
        }
        for stuck_state in stuck_in_loop_previous {
            all_emoves.get_mut(&stuck_state).unwrap().extend(current_emoves.clone());
        }

        all_emoves.insert(state.clone(), current_emoves);
        return stuck_in_loop_now;
    }
    
    pub fn transition_to_dfa(&mut self) -> AutomataDFA{ 
        use std::iter::FromIterator;

        let mut dfa = AutomataDFA::new(AutomataType::AutomataDFA);
        dfa.alphabet = self.alphabet.clone();

        let mut end_states = BTreeSet::new();
        let mut queue = std::collections::VecDeque::new();
        let start_state = dfa.add_state(BTreeSet::from_iter(vec![skip_fail!{&self.start_state, Option<_>, dfa}.clone()].into_iter()));
        queue.push_front(start_state.clone());
        dfa.set_begin_state(start_state);

        while let Some(compound_state) = queue.pop_front(){
            let mut transitions: HashMap<char, HashSet<Rc<State>>> = HashMap::with_capacity(self.alphabet.len());
            for value in self.alphabet.chars(){
                transitions.insert(value, HashSet::new());
            }
            for state in &compound_state.states{
                for connection in skip_fail!{self.states_connections.get(&state.clone()), Option<_>, dfa}{
                    match connection {
                        Connection::Value(state, value) => { 
                            transitions.get_mut(value)
                            .unwrap()
                            .insert(state.clone());
                        },
                         _ => (),
                    };
                }
            }
            
            for (value, states) in transitions {
                if states.is_empty() {
                    continue; //dead_state
                }
                if compound_state.states.iter().any(|state| self.end_states.borrow().contains(state)) {
                    end_states.insert(compound_state.clone());
                }
                let (new_compound_state, is_new_insert) = dfa.add_state_maybe_present(states.into_iter().collect());
                dfa.add_connection((compound_state.clone(), (new_compound_state.clone(), value)));
                if is_new_insert {
                    queue.push_back(new_compound_state);
                }
            }
        }       
        dfa.set_end_states(end_states);
        dfa
    }
    
}