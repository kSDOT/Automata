#[cfg(test)]
pub mod tests {
    use crate::automata::*;
    use crate::connection::*;
    use crate::automata_nfa::*;
    use crate::tests::create_automata_tests::tests::*;

    pub fn create_automata_nfa_1() -> AutomataNFA {
        let mut automata = create_automata_1();
        automata.transition_to_nfa();
        automata
    }

    #[test] 
    fn test_eclosure_to_nfa_transition_1() {
        let automata = create_automata_nfa_1();
        assert_eq!(automata, serde_json::from_str::<AutomataNFA>(r#"
        {
            "states_connections":[
                [{"id":3,"name":"q3"},[
                        {"Value":[{"id":3,"name":"q3"},"0"]},
                        {"Value":[{"id":4,"name":"q4"},"0"]},
                        {"Value":[{"id":0,"name":"q0"},"1"]},
                        {"Value":[{"id":1,"name":"q1"},"1"]}]
                    ],
                [{"id":2,"name":"q2"},[
                        {"Value":[{"id":3,"name":"q3"},"0"]},
                        {"Value":[{"id":4,"name":"q4"},"0"]},
                        {"Value":[{"id":0,"name":"q0"},"1"]},
                        {"Value":[{"id":1,"name":"q1"},"1"]}]
                    ],
                [{"id":0,"name":"q0"},[
                        {"Value":[{"id":0,"name":"q0"},"0"]},
                        {"Value":[{"id":1,"name":"q1"},"0"]},
                        {"Value":[{"id":2,"name":"q2"},"0"]},
                        {"Value":[{"id":3,"name":"q3"},"0"]},
                        {"Value":[{"id":4,"name":"q4"},"0"]},
                        {"Value":[{"id":3,"name":"q3"},"1"]},
                        {"Value":[{"id":4,"name":"q4"},"1"]}]
                    ],
                [{"id":4,"name":"q4"},[
                        {"Value":[{"id":0,"name":"q0"},"1"]},
                        {"Value":[{"id":1,"name":"q1"},"1"]}]
                    ],
                [{"id":1,"name":"q1"},[
                        {"Value":[{"id":2,"name":"q2"},"0"]},
                        {"Value":[{"id":3,"name":"q3"},"0"]},
                        {"Value":[{"id":4,"name":"q4"},"0"]},
                        {"Value":[{"id":3,"name":"q3"},"1"]},
                        {"Value":[{"id":4,"name":"q4"},"1"]}]
                    ]
            ],
            "start_state":{"id":0,"name":"q0"},
            "end_states":[
                    {"id":2,"name":"q2"},
                    {"id":3,"name":"q3"}
                ],
            "automata_type":"AutomataNFA",
            "alphabet" : "01",
            "counter": 5
            }
        "#).unwrap());
    }
    
    pub fn create_automata_nfa_2() -> AutomataNFA {
        let mut automata = create_automata_2();
        automata.transition_to_nfa();
        automata
    }
    #[test]
    fn test_eclosure_to_nfa_transition_2() {
        let automata = create_automata_nfa_2();

        assert_eq!(automata, serde_json::from_str::<AutomataNFA>(r#"
        {
           "states_connections":[
               [{"id":2,"name":"q2"},[
                       {"Value":[{"id":2,"name":"q2"},"1"]}]
                   ],
               [{"id":0,"name":"q0"},[
                       {"Value":[{"id":1,"name":"q1"},"0"]},
                       {"Value":[{"id":2,"name":"q2"},"0"]}]
                   ],
               [{"id":1,"name":"q1"},[
                       {"Value":[{"id":2,"name":"q2"},"1"]}]
                   ]
            ],
            "start_state":{"id":0,"name":"q0"},
            "end_states":[
                   {"id":2,"name":"q2"},
                   {"id":1,"name":"q1"}
               ],
            "automata_type":"AutomataNFA",
            "alphabet" : "01",
            "counter" : 3
        }
        "#).unwrap());
    }

    pub fn create_automata_nfa_3() -> AutomataNFA {
        let mut automata = create_automata_3();
        automata.transition_to_nfa();
        automata
    }

    #[test]
    fn test_eclosure_to_nfa_transition_3() {
        let automata = create_automata_nfa_3();

        assert_eq!(automata, serde_json::from_str::<AutomataNFA>(r#"
        {
            "states_connections":[
                [{"id":2,"name":"q2"},[
                        {"Value":[{"id":0,"name":"q0"},"a"]},
                        {"Value":[{"id":0,"name":"q0"},"b"]}]
                    ],
                [{"id":0,"name":"q0"},[
                        {"Value":[{"id":1,"name":"q1"},"a"]},
                        {"Value":[{"id":1,"name":"q1"},"b"]},
                        {"Value":[{"id":2,"name":"q2"},"a"]},
                        {"Value":[{"id":2,"name":"q2"},"b"]}]
                    ],
                [{"id":1,"name":"q1"},[
                        {"Value":[{"id":0,"name":"q0"},"a"]},
                        {"Value":[{"id":0,"name":"q0"},"b"]}]
                    ]
            ],
            "start_state":{"id":0,"name":"q0"},
            "end_states":[
                    {"id":2,"name":"q2"},
                    {"id":1,"name":"q1"}
                ],
            "automata_type":"AutomataNFA",
            "alphabet" : "ab",
            "counter" : 3
        }
        "#).unwrap());
    }

    pub fn create_automata_nfa_4() -> AutomataNFA {
        let mut automata = create_automata_4();
        automata.transition_to_nfa();
        automata
    }

    #[test]
    fn test_eclosure_to_nfa_transition_4() {
        let automata = create_automata_nfa_4();

        assert_eq!(automata, serde_json::from_str::<AutomataNFA>(r#"
        {
            "states_connections":[
                [{"id":2,"name":"q2"},[
                        {"Value":[{"id":2,"name":"q2"},"c"]}]
                    ],
                [{"id":1,"name":"q1"},[
                        {"Value":[{"id":1,"name":"q1"},"b"]},
                        {"Value":[{"id":2,"name":"q2"},"b"]},
                        {"Value":[{"id":2,"name":"q2"},"c"]}]
                    ],
                [{"id":0,"name":"q0"},[
                        {"Value":[{"id":0,"name":"q0"},"a"]},
                        {"Value":[{"id":1,"name":"q1"},"a"]},
                        {"Value":[{"id":2,"name":"q2"},"a"]},
                        {"Value":[{"id":1,"name":"q1"},"b"]},
                        {"Value":[{"id":2,"name":"q2"},"b"]},
                        {"Value":[{"id":2,"name":"q2"},"c"]}]
                    ]
            ],
            "start_state":{"id":0,"name":"q0"},
            "end_states":[
                {"id":1,"name":"q1"},
                {"id":2,"name":"q2"},
                {"id":0,"name":"q0"}
                ],
            "automata_type":"AutomataNFA",
            "alphabet" : "abc",
            "counter": 3
        }
        "#).unwrap());
    }

    
    pub fn create_automata_nfa_5() -> AutomataNFA {
        let mut automata = create_automata_5();
        automata.transition_to_nfa();
        automata
    }
    #[test]
    fn test_eclosure_to_nfa_transition_5() {
        let automata = create_automata_nfa_5();

        assert_eq!(automata, serde_json::from_str::<AutomataNFA>(r#"
        {
            "states_connections":[
                [{"id":2,"name":"C"},[
                        {"Value":[{"id":1,"name":"B"},"1"]},
                        {"Value":[{"id":3,"name":"D"},"1"]}]
                    ],
                [{"id":0,"name":"A"},[
                        {"Value":[{"id":0,"name":"A"},"0"]},
                        {"Value":[{"id":1,"name":"B"},"0"]},
                        {"Value":[{"id":2,"name":"C"},"0"]},
                        {"Value":[{"id":3,"name":"D"},"0"]},
                        {"Value":[{"id":3,"name":"D"},"1"]}]
                    ],
                [{"id":1,"name":"B"},[
                        {"Value":[{"id":2,"name":"C"},"0"]},
                        {"Value":[{"id":3,"name":"D"},"0"]},
                        {"Value":[{"id":3,"name":"D"},"1"]}]
                    ],
                [{"id":3,"name":"D"},[
                        {"Value":[{"id":3,"name":"D"},"0"]},
                        {"Value":[{"id":3,"name":"D"},"1"]}]
                    ]
                ],
            "start_state":{"id":0,"name":"A"},
            "end_states":[
                    {"id":0,"name":"A"},
                    {"id":3,"name":"D"},
                    {"id":1,"name":"B"}
                ],
            "automata_type": "AutomataNFA",
            "alphabet": "01",
            "counter": 4
        }
       "#).unwrap());
    }

    pub fn create_automata_nfa_6() -> AutomataNFA {
        let mut automata = create_automata_6();
        automata.transition_to_nfa();
        automata
    }

    #[test]
    fn test_eclosure_to_nfa_transition_6() {
        let automata = create_automata_nfa_6();

        assert_eq!(automata, serde_json::from_str::<AutomataNFA>(r#"
        {
            "states_connections":[
                [{"id":0,"name":"q0"},[
                        {"Value":[{"id":3,"name":"q3"},"0"]},
                        {"Value":[{"id":0,"name":"q0"},"1"]},
                        {"Value":[{"id":1,"name":"q1"},"1"]},
                        {"Value":[{"id":2,"name":"q2"},"1"]},
                        {"Value":[{"id":4,"name":"q4"},"1"]},
                        {"Value":[{"id":5,"name":"q5"},"1"]},
                        {"Value":[{"id":6,"name":"q6"},"1"]}]
                    ],

                [{"id":1,"name":"q1"},[
                        {"Value":[{"id":3,"name":"q3"},"0"]},
                        {"Value":[{"id":0,"name":"q0"},"1"]},
                        {"Value":[{"id":1,"name":"q1"},"1"]},
                        {"Value":[{"id":2,"name":"q2"},"1"]},
                        {"Value":[{"id":4,"name":"q4"},"1"]},
                        {"Value":[{"id":5,"name":"q5"},"1"]},
                        {"Value":[{"id":6,"name":"q6"},"1"]}]
                    ],
                
                [{"id":2,"name":"q2"},[
                        {"Value":[{"id":3,"name":"q3"},"0"]},
                        {"Value":[{"id":5,"name":"q5"},"1"]},
                        {"Value":[{"id":6,"name":"q6"},"1"]}]
                    ],
                [{"id":3,"name":"q3"},[
                        {"Value":[{"id":2,"name":"q2"},"1"]},
                        {"Value":[{"id":4,"name":"q4"},"1"]},
                        {"Value":[{"id":5,"name":"q5"},"1"]},
                        {"Value":[{"id":6,"name":"q6"},"1"]}]
                    ],
                [{"id":4,"name":"q4"},[
                        {"Value":[{"id":3,"name":"q3"},"0"]},
                        {"Value":[{"id":5,"name":"q5"},"1"]},
                        {"Value":[{"id":6,"name":"q6"},"1"]}]
                    ],
                [{"id":5,"name":"q5"},[
                        {"Value":[{"id":5,"name":"q5"},"1"]},
                        {"Value":[{"id":6,"name":"q6"},"1"]}]
                    ],
                [{"id":6,"name":"q6"},[    
                        {"Value":[{"id":5,"name":"q5"},"1"]},
                        {"Value":[{"id":6,"name":"q6"},"1"]}]
                    ]
            ],
            "start_state":{"id":0,"name":"q0"},
            "end_states":[
                    {"id":0,"name":"q0"}, 
                    {"id":1,"name":"q1"},
                    {"id":2,"name":"q2"},
                    {"id":4,"name":"q4"},
                    {"id":5,"name":"q5"},
                    {"id":6,"name":"q6"}

                ],
            "automata_type":"AutomataNFA",
            "alphabet" : "10",
            "counter" : 7
        }
        "#).unwrap());
    }
}