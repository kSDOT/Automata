#![feature(drain_filter)]
#![feature(vec_remove_item)]
#[macro_use]
pub mod skip_macro;
pub mod automata;
pub mod connection;
pub mod connection_to_compound_state;
pub mod state;
pub mod compound_state;
pub mod tests;
pub mod map_as_pairs;
pub mod automata_nfa;
pub mod automata_dfa;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


fn nfa_to_json(automata_nfa: automata_nfa::AutomataNFA) -> JsValue {
    return JsValue::from_serde(
        &serde_json::to_string(&
            automata_nfa
        ).unwrap()
    ).unwrap();
}

fn dfa_to_json(automata_nfa: automata_dfa::AutomataDFA) -> JsValue {
    return JsValue::from_serde(
        &serde_json::to_string(&
            automata_nfa
        ).unwrap()
    ).unwrap();
}

#[wasm_bindgen]
pub fn init() -> JsValue{
    return nfa_to_json(automata_nfa::AutomataNFA::new(automata::AutomataType::AutomataEClosure));
}

fn json_to_nfa(automata: JsValue) -> automata_nfa::AutomataNFA{
    return automata.into_serde().unwrap();
}

fn json_to_dfa(automata: JsValue) -> automata_dfa::AutomataDFA{
    return automata.into_serde().unwrap();
}

fn json_to_array_string(array: JsValue) -> String{
   array.into_serde().unwrap()
}

#[wasm_bindgen]
pub fn add_state(automata: JsValue, name: String)  -> JsValue{
    let mut automata = json_to_nfa(automata);
    automata.add_state(&name);
    nfa_to_json(automata)
}

#[wasm_bindgen]
pub fn init_state(automata: JsValue, name: JsValue)  -> JsValue{
    let name = json_to_array_string(name);
    let mut automata = json_to_nfa(automata);
    
    if automata.states_connections.keys().any(|state| return state.name == name) == false{    
       automata.add_state(&name);   
    }
    nfa_to_json(automata)
}

fn name_to_state(automata: &mut automata_nfa::AutomataNFA, name: &str) -> Option<std::rc::Rc<state::State>>{
    for state in automata.states_connections.keys(){
        if state.name == name {
            return Some(state.clone());
        }
    }
    None
}

#[wasm_bindgen]
pub fn add_connection(automata: JsValue, first_state: String, other_state: String, symbol: char, emove: bool)  -> JsValue {
    let mut automata = json_to_nfa(automata);
    let first_state = match name_to_state(&mut automata, &first_state){
        Some(state)=> state,
        None=> return nfa_to_json(automata) 
    };
    
    let other_state = match name_to_state(&mut automata, &other_state){
        Some(state)=> state,
        None=> return nfa_to_json(automata) 
    };

    automata.add_connection((first_state, match emove {
        true => connection::Connection::EMove(other_state),
        false => connection::Connection::Value(other_state, symbol),
    }));
    nfa_to_json(automata)
}

#[wasm_bindgen]
pub fn set_alphabet(automata: JsValue, name: String)  -> JsValue{
    let mut automata = json_to_nfa(automata);
    automata.alphabet = name;
    nfa_to_json(automata)
}

#[wasm_bindgen]
pub fn transition_to_nfa(automata: JsValue) -> JsValue {
    let mut automata = json_to_nfa(automata);
    automata.transition_to_nfa();
    nfa_to_json(automata)
}

#[wasm_bindgen]
pub fn transition_to_dfa(automata: JsValue) -> JsValue {
    let mut automata = json_to_nfa(automata);
    dfa_to_json(automata.transition_to_dfa())
}

#[wasm_bindgen]
pub fn transition_to_dfa_minimized(automata: JsValue) -> JsValue {
    let mut automata = json_to_dfa(automata);
    automata.transition_to_minimized();
    dfa_to_json(automata)
}
#[wasm_bindgen]
pub fn set_start_state(automata: JsValue, name: String) -> JsValue {
    let mut automata = json_to_nfa(automata);
    if let Some(state) = name_to_state(&mut automata, &name){
        automata.set_begin_state(state);
    }
    nfa_to_json(automata)
}

#[wasm_bindgen]
pub fn set_end_state(automata: JsValue, name: String) -> JsValue {
    let mut automata = json_to_nfa(automata);
    if let Some(state) = name_to_state(&mut automata, &name){
        automata.end_states.borrow_mut().insert(state);
    }
    nfa_to_json(automata)
}
