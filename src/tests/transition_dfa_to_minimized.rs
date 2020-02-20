#[cfg(test)]
pub mod tests {
    use crate::automata_nfa::*;
    use crate::automata_dfa::*;
    use crate::connection::*;
    use crate::automata::*;
    use std::fs::File;
    use std::io::Read;
    #[test]
    fn test_automata_minimized_1() {

        let mut file = File::open("www/tests/automata_minimized_1.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        
        let mut automata = serde_json::from_str::<AutomataNFA>(&data).unwrap();
        assert_eq!(automata, serde_json::from_str::<AutomataNFA>(&data).unwrap());

        automata.transition_to_nfa();
        let mut automata = automata.transition_to_dfa();
        automata.transition_to_minimized();

        assert_eq!(automata, serde_json::from_str::<AutomataDFA>(r#"
        {
            "states_connections":[
                [{"states":[{"id":0,"name":"q0"},{"id":2,"name":"q2"}]},[
                        [{"states":[{"id":0,"name":"q0"},{"id":2,"name":"q2"}]},"b"],
                        [{"states":[{"id":1,"name":"q1"}]},"a"]]
                    ],
                [{"states":[{"id":1,"name":"q1"}]},[
                        [{"states":[{"id":1,"name":"q1"}]},"a"],
                        [{"states":[{"id":3,"name":"q3"}]},"b"]]
                    ],
                [{"states":[{"id":3,"name":"q3"}]},[
                        [{"states":[{"id":1,"name":"q1"}]},"a"],
                        [{"states":[{"id":4,"name":"q4"}]},"b"]]
                    ],
                [{"states":[{"id":4,"name":"q4"}]},[
                        [{"states":[{"id":0,"name":"q0"},{"id":2,"name":"q2"}]},"b"],
                        [{"states":[{"id":1,"name":"q1"}]},"a"]]
                    ]
            ],
            "start_state":{"states":[{"id":0,"name":"q0"}]},
            "end_states":[{"states":[{"id":4,"name":"q4"}]}],
            "automata_type":"AutomataMinimizedDFA",
            "alphabet":"ab",
            "counter":11
        }
        "#).unwrap());   
    }

    #[test]
    fn test_automata_minimized_2() {

        let mut file = File::open("www/tests/automata_minimized_2.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        
        let mut automata = serde_json::from_str::<AutomataNFA>(&data).unwrap();
        assert_eq!(automata, serde_json::from_str::<AutomataNFA>(&data).unwrap());

        automata.transition_to_nfa();
        let mut automata = automata.transition_to_dfa();
        automata.transition_to_minimized();

        assert_eq!(automata, serde_json::from_str::<AutomataDFA>(r#"
        {
            "states_connections":[
                [{"states":[{"id":0,"name":"q0"}]},[
                        [{"states":[{"id":1,"name":"q1"}]},"0"],
                        [{"states":[{"id":4,"name":"q4"}]},"1"]]
                    ],
                [{"states":[{"id":1,"name":"q1"}]},[
                        [{"states":[{"id":2,"name":"q2"}]},"1"],
                        [{"states":[{"id":5,"name":"q5"}]},"0"]]
                    ],
                [{"states":[{"id":2,"name":"q2"}]},[
                        [{"states":[{"id":0,"name":"q0"}]},"0"],
                        [{"states":[{"id":2,"name":"q2"}]},"1"]]
                    ],
                [{"states":[{"id":3,"name":"q3"}]},[
                        [{"states":[{"id":4,"name":"q4"}]},"1"],
                        [{"states":[{"id":6,"name":"q6"}]},"0"]]
                    ],
                [{"states":[{"id":4,"name":"q4"}]},[
                        [{"states":[{"id":2,"name":"q2"}]},"0"],
                        [{"states":[{"id":5,"name":"q5"}]},"1"]]
                    ],
                [{"states":[{"id":5,"name":"q5"}]},[
                        [{"states":[{"id":3,"name":"q3"}]},"1"],
                        [{"states":[{"id":5,"name":"q5"}]},"0"]]
                    ],
                [{"states":[{"id":6,"name":"q6"}]},[
                        [{"states":[{"id":2,"name":"q2"}]},"1"],
                        [{"states":[{"id":5,"name":"q5"}]},"0"]]
                    ]
            ],
            "start_state":{"states":[{"id":0,"name":"q0"}]},
            "end_states":[{"states":[{"id":2,"name":"q2"}]}],
            "automata_type":"AutomataMinimizedDFA",
            "alphabet":"01",
            "counter":15
        }
        "#).unwrap());   
    }
    #[test]
    fn test_automata_minimized_3() {
        
        {
            let mut automata = AutomataNFA::new(AutomataType::AutomataEClosure);
        let state0 = automata.add_state("");
        let state1 = automata.add_state("");
        let state2 = automata.add_state("");
        let state3 = automata.add_state("");
        let state4 = automata.add_state("");
        let state5 = automata.add_state("");
        let state6 = automata.add_state("");
        let state7 = automata.add_state("");


        automata.add_connection((state0.clone(), Connection::Value(state1.clone(), '0')));
        automata.add_connection((state0.clone(), Connection::Value(state5.clone(), '1')));//q0

        automata.add_connection((state1.clone(), Connection::Value(state6.clone(), '0')));//q1
        automata.add_connection((state1.clone(), Connection::Value(state2.clone(), '1')));//q1

        automata.add_connection((state2.clone(), Connection::Value(state0.clone(), '0')));//q1
        automata.add_connection((state2.clone(), Connection::Value(state2.clone(), '1')));//q2

        automata.add_connection((state3.clone(), Connection::Value(state2.clone(), '0')));//q1
        automata.add_connection((state3.clone(), Connection::Value(state6.clone(), '1')));//q2

        automata.add_connection((state4.clone(), Connection::Value(state7.clone(), '0')));//q1
        automata.add_connection((state4.clone(), Connection::Value(state5.clone(), '1')));//q2

        automata.add_connection((state5.clone(), Connection::Value(state2.clone(), '0')));//q1
        automata.add_connection((state5.clone(), Connection::Value(state6.clone(), '1')));//q2

        automata.add_connection((state6.clone(), Connection::Value(state6.clone(), '0')));//q1
        automata.add_connection((state6.clone(), Connection::Value(state4.clone(), '1')));//q2

        automata.add_connection((state7.clone(), Connection::Value(state6.clone(), '0')));//q1
        automata.add_connection((state7.clone(), Connection::Value(state2.clone(), '1')));//q2

        automata.set_begin_state(state0.clone());
        automata.set_end_states(vec![state2.clone()].into_iter().collect());
            println!("{}", serde_json::to_string(&automata).unwrap());
        }

        let mut file = File::open("www/tests/automata_minimized_3.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        
        let mut automata = serde_json::from_str::<AutomataNFA>(&data).unwrap();
        assert_eq!(automata, serde_json::from_str::<AutomataNFA>(&data).unwrap());

        automata.transition_to_nfa();
        let mut automata = automata.transition_to_dfa();
        automata.transition_to_minimized();

        assert_eq!(automata, serde_json::from_str::<AutomataDFA>(r#"
        {
            "states_connections":[
                [{"states":[{"id":0,"name":"q0"}]},[
                        [{"states":[{"id":1,"name":"q1"}]},"0"],
                        [{"states":[{"id":4,"name":"q4"}]},"1"]]
                    ],
                [{"states":[{"id":1,"name":"q1"}]},[
                        [{"states":[{"id":2,"name":"q2"}]},"1"],
                        [{"states":[{"id":5,"name":"q5"}]},"0"]]
                    ],
                [{"states":[{"id":2,"name":"q2"}]},[
                        [{"states":[{"id":0,"name":"q0"}]},"0"],
                        [{"states":[{"id":2,"name":"q2"}]},"1"]]
                    ],
                [{"states":[{"id":3,"name":"q3"}]},[
                        [{"states":[{"id":4,"name":"q4"}]},"1"],
                        [{"states":[{"id":6,"name":"q6"}]},"0"]]
                    ],
                [{"states":[{"id":4,"name":"q4"}]},[
                        [{"states":[{"id":2,"name":"q2"}]},"0"],
                        [{"states":[{"id":5,"name":"q5"}]},"1"]]
                    ],
                [{"states":[{"id":5,"name":"q5"}]},[
                        [{"states":[{"id":3,"name":"q3"}]},"1"],
                        [{"states":[{"id":5,"name":"q5"}]},"0"]]
                    ],
                [{"states":[{"id":6,"name":"q6"}]},[
                        [{"states":[{"id":2,"name":"q2"}]},"1"],
                        [{"states":[{"id":5,"name":"q5"}]},"0"]]
                    ]
            ],
            "start_state":{"states":[{"id":0,"name":"q0"}]},
            "end_states":[{"states":[{"id":2,"name":"q2"}]}],
            "automata_type":"AutomataMinimizedDFA",
            "alphabet":"01",
            "counter":15
        }
        "#).unwrap());   
    }
}