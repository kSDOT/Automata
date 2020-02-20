#[cfg(test)]
pub mod tests {
    use crate::automata_nfa::*;
    use crate::connection::*;
    use crate::automata::*;
    use std::fs::File;
    use std::io::Read;
    
    pub fn create_automata_1() -> AutomataNFA {
        let mut automata = AutomataNFA::new(AutomataType::AutomataEClosure);

        let state0 = automata.add_state("");
        let state1 = automata.add_state("");
        let state2 = automata.add_state("");
        let state3 = automata.add_state("");
        let state4 = automata.add_state("");

        automata.add_connection((state0.clone(), Connection::Value(state0.clone(), '0')));//q0
        automata.add_connection((state0.clone(), Connection::EMove(state1.clone())));

        automata.add_connection((state1.clone(), Connection::Value(state2.clone(), '0')));//q1
        automata.add_connection((state1.clone(), Connection::Value(state3.clone(), '1')));

        automata.add_connection((state2.clone(), Connection::EMove(state3.clone())));//q2

        automata.add_connection((state3.clone(), Connection::Value(state3.clone(), '0')));//q3
        automata.add_connection((state3.clone(), Connection::Value(state1.clone(), '1')));
        automata.add_connection((state3.clone(), Connection::EMove(state4.clone())));

        automata.add_connection((state4.clone(), Connection::Value(state0.clone(), '1')));//q4

        automata.set_begin_state(state0.clone());
        automata.set_end_states(vec![state3.clone()].into_iter().collect());

        automata
    }

    #[test]
    fn test_automata_creation_1() {
        let automata =  create_automata_1();
        let mut file = File::open("www/tests/automata_1.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        assert_eq!(automata, serde_json::from_str::<AutomataNFA>(&data).unwrap());
    }

    pub fn create_automata_2() -> AutomataNFA {
        let mut automata = AutomataNFA::new(AutomataType::AutomataEClosure);
        let state0 = automata.add_state("");
        let state1 = automata.add_state("");
        let state2 = automata.add_state("");

        automata.add_connection((state0.clone(), Connection::Value(state1.clone(), '0')));//q0

        automata.add_connection((state1.clone(), Connection::EMove(state2.clone())));//q1

        automata.add_connection((state2.clone(), Connection::Value(state2.clone(), '1')));//q2

        automata.set_begin_state(state0.clone());
        automata.set_end_states(vec![state2.clone()].into_iter().collect());
        
        automata
    }
    #[test]
    fn test_automata_creation_2() {
        let automata = create_automata_2();
        let mut file = File::open("www/tests/automata_2.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        assert_eq!(automata, serde_json::from_str::<AutomataNFA>(&data).unwrap());
    }
    
    pub fn create_automata_3() -> AutomataNFA {
          
        let mut automata = AutomataNFA::new(AutomataType::AutomataEClosure);
        let state0 = automata.add_state("");
        let state1 = automata.add_state("");
        let state2 = automata.add_state("");

        automata.add_connection((state0.clone(), Connection::Value(state2.clone(), 'a')));
        automata.add_connection((state0.clone(), Connection::Value(state1.clone(), 'b')));//q0

        automata.add_connection((state1.clone(), Connection::Value(state0.clone(), 'a')));//q1
        automata.add_connection((state1.clone(), Connection::EMove(state2.clone())));//q1


        automata.add_connection((state2.clone(), Connection::EMove(state1.clone())));//q2
        automata.add_connection((state2.clone(), Connection::Value(state0.clone(), 'b')));//q2


        automata.set_begin_state(state0.clone());
        automata.set_end_states(vec![state2.clone()].into_iter().collect());

        automata
    }

    #[test]
    fn test_automata_creation_3(){
        let automata = create_automata_3();
        let mut file = File::open("www/tests/automata_3.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        assert_eq!(automata, serde_json::from_str::<AutomataNFA>(&data).unwrap());
    }

    pub fn create_automata_4() -> AutomataNFA {
          
        let mut automata = AutomataNFA::new(AutomataType::AutomataEClosure);
            
        let state0 = automata.add_state("");
        let state1 = automata.add_state("");
        let state2 = automata.add_state("");

        automata.add_connection((state0.clone(), Connection::Value(state0.clone(), 'a')));//q0
        automata.add_connection((state0.clone(), Connection::EMove(state1.clone())));

        automata.add_connection((state1.clone(), Connection::Value(state1.clone(), 'b')));//q1
        automata.add_connection((state1.clone(), Connection::EMove(state2.clone())));//q1

        automata.add_connection((state2.clone(), Connection::Value(state2.clone(), 'c')));//q2

        automata.set_begin_state(state0.clone());
        automata.set_end_states(vec![state2.clone()].into_iter().collect());

        automata
    }

    #[test]
    fn test_automata_creation_4(){
        let automata = create_automata_4();
        let mut file = File::open("www/tests/automata_4.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        assert_eq!(automata, serde_json::from_str::<AutomataNFA>(&data).unwrap());
    }

    pub fn create_automata_5() -> AutomataNFA {
          
        let mut automata = AutomataNFA::new(AutomataType::AutomataEClosure);
        let state_a = automata.add_state("A");
        let state_b = automata.add_state("B");
        let state_c = automata.add_state("C");
        let state_d = automata.add_state("D");

        automata.add_connection((state_a.clone(), Connection::Value(state_a.clone(), '0')));//A
        automata.add_connection((state_a.clone(), Connection::EMove(state_b.clone())));

        automata.add_connection((state_b.clone(), Connection::Value(state_c.clone(), '0')));//B
        automata.add_connection((state_b.clone(), Connection::EMove(state_d.clone())));

        automata.add_connection((state_c.clone(), Connection::Value(state_b.clone(), '1')));//C

        automata.add_connection((state_d.clone(), Connection::Value(state_d.clone(), '0')));//D
        automata.add_connection((state_d.clone(), Connection::Value(state_d.clone(), '1')));

        automata.set_begin_state(state_a.clone());
        automata.set_end_states(vec![state_d.clone()].into_iter().collect());

        automata
    }

    #[test]
    fn test_automata_creation_5(){
        let automata = create_automata_5();
        let mut file = File::open("www/tests/automata_5.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        assert_eq!(automata, serde_json::from_str::<AutomataNFA>(&data).unwrap());
    }

    pub fn create_automata_6() -> AutomataNFA {
          
        let mut automata = AutomataNFA::new(AutomataType::AutomataEClosure);

        let state0 = automata.add_state("");
        let state1 = automata.add_state("");
        let state2 = automata.add_state("");
        let state3 = automata.add_state("");
        let state4 = automata.add_state("");
        let state5 = automata.add_state("");
        let state6 = automata.add_state("");

        automata.add_connection((state0.clone(), Connection::Value(state1.clone(), '1')));//q0
        automata.add_connection((state0.clone(), Connection::EMove(state1.clone())));

        automata.add_connection((state1.clone(), Connection::EMove(state0.clone())));//q1
        automata.add_connection((state1.clone(), Connection::EMove(state2.clone())));

        automata.add_connection((state2.clone(), Connection::Value(state3.clone(), '0')));//q2
        automata.add_connection((state2.clone(), Connection::EMove(state4.clone())));
       
        automata.add_connection((state3.clone(), Connection::Value(state4.clone(), '1')));//q3

        automata.add_connection((state4.clone(),  Connection::EMove(state2.clone())));//q4
        automata.add_connection((state4.clone(), Connection::EMove(state5.clone())));

        automata.add_connection((state5.clone(),  Connection::Value(state5.clone(), '1')));//q5
        automata.add_connection((state5.clone(),  Connection::EMove(state6.clone())));

        automata.add_connection((state6.clone(), Connection::EMove(state5.clone())));//q6
        

        automata.set_begin_state(state0.clone());
        automata.set_end_states(vec![state6.clone()].into_iter().collect());
        automata
    }

    #[test]
    fn test_automata_creation_6(){
        let automata = create_automata_6();
        let mut file = File::open("www/tests/automata_6.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        assert_eq!(automata, serde_json::from_str::<AutomataNFA>(&data).unwrap());
    }
}