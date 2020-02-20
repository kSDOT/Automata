use serde::*;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum AutomataType {
    AutomataEClosure,
    AutomataNFA,
    AutomataDFA,
    AutomataMinimizedDFA,
}

pub trait Automata {
    fn get_type(&self) -> AutomataType;
}
