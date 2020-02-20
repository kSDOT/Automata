#[cfg(test)]
pub mod tests {
    use crate::automata::*;
    use crate::connection::*;
    use crate::automata_dfa::*;
    use crate::tests::transition_eclosure_to_nfa_tests::tests::*;

    pub fn create_automata_dfa_1() -> AutomataDFA {
        let mut automata = create_automata_nfa_1();
        automata.transition_to_dfa()
    }

    #[test] 
    fn test_nfa_to_dfa_transition_1() {

        let automata = create_automata_dfa_1();
        
        assert_eq!(automata, serde_json::from_str::<AutomataDFA>(r#"
        {
            "states_connections":[
                [{"states":[{"id":0,"name":"q0"}]},[
                        [{"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":2,"name":"q2"},{"id":3,"name":"q3"},{"id":4,"name":"q4"}]},"0"],
                        [{"states":[{"id":3,"name":"q3"},{"id":4,"name":"q4"}]},"1"]]
                    ],
                [{"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"}]},[
                        [{"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":2,"name":"q2"},{"id":3,"name":"q3"},{"id":4,"name":"q4"}]},"0"],
                        [{"states":[{"id":3,"name":"q3"},{"id":4,"name":"q4"}]},"1"]]
                    ],
                [{"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":2,"name":"q2"},{"id":3,"name":"q3"},{"id":4,"name":"q4"}]},[
                        [{"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":2,"name":"q2"},{"id":3,"name":"q3"},{"id":4,"name":"q4"}]},"0"],
                        [{"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":3,"name":"q3"},{"id":4,"name":"q4"}]},"1"]]
                    ],
                [{"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":3,"name":"q3"},{"id":4,"name":"q4"}]},[
                        [{"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":2,"name":"q2"},{"id":3,"name":"q3"},{"id":4,"name":"q4"}]},"0"],
                        [{"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":3,"name":"q3"},{"id":4,"name":"q4"}]},"1"]]
                    ],
                [{"states":[{"id":3,"name":"q3"},{"id":4,"name":"q4"}]},[
                        [{"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"}]},"1"],
                        [{"states":[{"id":3,"name":"q3"},{"id":4,"name":"q4"}]},"0"]]
                    ]
            ],
            "start_state":{"states":[{"id":0,"name":"q0"}]},
            "end_states":[
                    {"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":2,"name":"q2"},{"id":3,"name":"q3"},{"id":4,"name":"q4"}]},
                    {"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":3,"name":"q3"},{"id":4,"name":"q4"}]},
                    {"states":[{"id":3,"name":"q3"},{"id":4,"name":"q4"}]}
                ],
            "automata_type":"AutomataDFA",
            "alphabet":"01",
            "counter":11
        }
        "#).unwrap());
    }

    pub fn create_automata_dfa_2() -> AutomataDFA {
        let mut automata = create_automata_nfa_2();
        automata.transition_to_dfa()
    }

    #[test] 
    fn test_nfa_to_dfa_transition_2() {
        let automata = create_automata_dfa_2();

        assert_eq!(automata, serde_json::from_str::<AutomataDFA>(r#"
        {
            "states_connections":[
                [{"states":[{"id":2,"name":"q2"},{"id":1,"name":"q1"}]},[
                        [{"states":[{"id":2,"name":"q2"}]},"1"]]
                    ],
                [{"states":[{"id":2,"name":"q2"}]},[
                        [{"states":[{"id":2,"name":"q2"}]},"1"]]
                    ],
                [{"states":[{"id":0,"name":"q0"}]},[
                        [{"states":[{"id":2,"name":"q2"},{"id":1,"name":"q1"}]},"0"]]
                    ]
            ],
            "start_state":{"states":[{"id":0,"name":"q0"}]},
            "end_states":[
                    {"states":[{"id":2,"name":"q2"}]},
                    {"states":[{"id":2,"name":"q2"},{"id":1,"name":"q1"}]}
                ],
            "automata_type":"AutomataDFA",
            "alphabet":"01",
            "counter":4
        }
        "#).unwrap());
    }

    pub fn create_automata_dfa_3() -> AutomataDFA {
        let mut automata = create_automata_nfa_3();
        automata.transition_to_dfa()
    }

    #[test] 
    fn test_nfa_to_dfa_transition_3() {
        let automata = create_automata_dfa_3();
        assert_eq!(automata, serde_json::from_str::<AutomataDFA>(r#"
        {
            "states_connections":[
                [{"states":[{"id":0,"name":"q0"}]},[
                        [{"states":[{"id":1,"name":"q1"},{"id":2,"name":"q2"}]},"a"],
                        [{"states":[{"id":1,"name":"q1"},{"id":2,"name":"q2"}]},"b"]]
                    ],
                [{"states":[{"id":1,"name":"q1"},{"id":2,"name":"q2"}]},[
                        [{"states":[{"id":0,"name":"q0"}]},"a"],
                        [{"states":[{"id":0,"name":"q0"}]},"b"]]
                    ]
                ],
            "start_state":{"states":[{"id":0,"name":"q0"}]},
            "end_states":[
                    {"states":[{"id":1,"name":"q1"},{"id":2,"name":"q2"}]}
                ],
            "automata_type":"AutomataDFA","alphabet":"ab","counter":5
        }
        "#).unwrap());
    }

    pub fn create_automata_dfa_4() -> AutomataDFA {
        let mut automata = create_automata_nfa_4();
        automata.transition_to_dfa()
    }

    #[test] 
    fn test_nfa_to_dfa_transition_4() {
        let automata = create_automata_dfa_4();
        assert_eq!(automata, serde_json::from_str::<AutomataDFA>(r#"
        {
            "states_connections":[
                [{"states":[{"id":0,"name":"q0"}]},[
                        [{"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":2,"name":"q2"}]},"a"],
                        [{"states":[{"id":1,"name":"q1"},{"id":2,"name":"q2"}]},"b"],
                        [{"states":[{"id":2,"name":"q2"}]},"c"]]
                    ],
                [{"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":2,"name":"q2"}]},[
                        [{"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":2,"name":"q2"}]},"a"],
                        [{"states":[{"id":1,"name":"q1"},{"id":2,"name":"q2"}]},"b"],
                        [{"states":[{"id":2,"name":"q2"}]},"c"]]
                    ],
                [{"states":[{"id":1,"name":"q1"},{"id":2,"name":"q2"}]},[
                        [{"states":[{"id":1,"name":"q1"},{"id":2,"name":"q2"}]},"b"],
                        [{"states":[{"id":2,"name":"q2"}]},"c"]]
                    ],
                [{"states":[{"id":2,"name":"q2"}]},[
                        [{"states":[{"id":2,"name":"q2"}]},"c"]]
                    ]
            ],
            "start_state":{"states":[{"id":0,"name":"q0"}]},
            "end_states":[
                    {"states":[{"id":0,"name":"q0"}]},
                    {"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":2,"name":"q2"}]},
                    {"states":[{"id":1,"name":"q1"},{"id":2,"name":"q2"}]},
                    {"states":[{"id":2,"name":"q2"}]}
                ],
            "automata_type":"AutomataDFA",
            "alphabet":"abc",
            "counter":10
        }
        "#).unwrap());
    }

    pub fn create_automata_dfa_5() -> AutomataDFA {
        let mut automata = create_automata_nfa_5();
        automata.transition_to_dfa()
    }

    #[test] 
    fn test_nfa_to_dfa_transition_5() {
        let automata = create_automata_dfa_5();
        assert_eq!(automata, serde_json::from_str::<AutomataDFA>(r#"
        {
            "states_connections":[
                [{"states":[{"id":0,"name":"A"}]},[
                        [{"states":[{"id":0,"name":"A"},{"id":1,"name":"B"},{"id":2,"name":"C"},{"id":3,"name":"D"}]},"0"],
                        [{"states":[{"id":3,"name":"D"}]},"1"]]
                    ],
                [{"states":[{"id":0,"name":"A"},{"id":1,"name":"B"},{"id":2,"name":"C"},{"id":3,"name":"D"}]},[
                        [{"states":[{"id":0,"name":"A"},{"id":1,"name":"B"},{"id":2,"name":"C"},{"id":3,"name":"D"}]},"0"],
                        [{"states":[{"id":1,"name":"B"},{"id":3,"name":"D"}]},"1"]]
                    ],
                [{"states":[{"id":1,"name":"B"},{"id":3,"name":"D"}]},[
                        [{"states":[{"id":2,"name":"C"},{"id":3,"name":"D"}]},"0"],
                        [{"states":[{"id":3,"name":"D"}]},"1"]]
                    ],
                [{"states":[{"id":2,"name":"C"},{"id":3,"name":"D"}]},[
                        [{"states":[{"id":1,"name":"B"},{"id":3,"name":"D"}]},"1"],
                        [{"states":[{"id":3,"name":"D"}]},"0"]]
                    ],
                [{"states":[{"id":3,"name":"D"}]},[
                        [{"states":[{"id":3,"name":"D"}]},"0"],
                        [{"states":[{"id":3,"name":"D"}]},"1"]]
                    ]
                ],
            "start_state":{"states":[{"id":0,"name":"A"}]},
            "end_states":[
                    {"states":[{"id":0,"name":"A"}]},{"states":[{"id":0,"name":"A"},{"id":1,"name":"B"},{"id":2,"name":"C"},{"id":3,"name":"D"}]},
                    {"states":[{"id":1,"name":"B"},{"id":3,"name":"D"}]},
                    {"states":[{"id":2,"name":"C"},{"id":3,"name":"D"}]},
                    {"states":[{"id":3,"name":"D"}]}
                ],
            "automata_type":"AutomataDFA",
            "alphabet":"01",
            "counter":11
        }
        "#).unwrap());
    }

    pub fn create_automata_dfa_6() -> AutomataDFA {
        let mut automata = create_automata_nfa_6();
        automata.transition_to_dfa()
    }

    #[test] 
    fn test_nfa_to_dfa_transition_6() {
        let automata = create_automata_dfa_6();
        assert_eq!(automata, serde_json::from_str::<AutomataDFA>(r#"
        {
            "states_connections":[
                [{"states":[{"id":0,"name":"q0"}]},[
                        [{"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":2,"name":"q2"},{"id":4,"name":"q4"},{"id":5,"name":"q5"},{"id":6,"name":"q6"}]},"1"],
                        [{"states":[{"id":3,"name":"q3"}]},"0"]]
                    ],
                [{"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":2,"name":"q2"},{"id":4,"name":"q4"},{"id":5,"name":"q5"},{"id":6,"name":"q6"}]},[
                        [{"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":2,"name":"q2"},{"id":4,"name":"q4"},{"id":5,"name":"q5"},{"id":6,"name":"q6"}]},"1"],
                        [{"states":[{"id":3,"name":"q3"}]},"0"]]
                    ],
                [{"states":[{"id":2,"name":"q2"},{"id":4,"name":"q4"},{"id":5,"name":"q5"},{"id":6,"name":"q6"}]},[
                        [{"states":[{"id":3,"name":"q3"}]},"0"],
                        [{"states":[{"id":5,"name":"q5"},{"id":6,"name":"q6"}]},"1"]]
                    ],
                [{"states":[{"id":3,"name":"q3"}]},[
                        [{"states":[{"id":2,"name":"q2"},{"id":4,"name":"q4"},{"id":5,"name":"q5"},{"id":6,"name":"q6"}]},"1"]]
                    ],
                [{"states":[{"id":5,"name":"q5"},{"id":6,"name":"q6"}]},[
                        [{"states":[{"id":5,"name":"q5"},{"id":6,"name":"q6"}]},"1"]]
                    ]
            ],
            "start_state":{"states":[{"id":0,"name":"q0"}]},
            "end_states":[
                    {"states":[{"id":0,"name":"q0"}]},
                    {"states":[{"id":0,"name":"q0"},{"id":1,"name":"q1"},{"id":2,"name":"q2"},{"id":4,"name":"q4"},{"id":5,"name":"q5"},{"id":6,"name":"q6"}]},
                    {"states":[{"id":2,"name":"q2"},{"id":4,"name":"q4"},{"id":5,"name":"q5"},{"id":6,"name":"q6"}]},
                    {"states":[{"id":5,"name":"q5"},{"id":6,"name":"q6"}]}
                ],
            "automata_type":"AutomataDFA",
            "alphabet":"10",
            "counter":9
        }
        "#).unwrap());
    }

}