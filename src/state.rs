use serde::*;

#[derive(Debug, Hash)]
#[derive(Serialize, Deserialize)]
pub struct State {
    id: u32,
    pub name: String,
}

impl State {
    pub fn new(id: u32, name: &str) -> State {
        let name = if name.is_empty(){
                        format!("q{}", id)  
                   }
                   else {
                       name.to_owned()
                   };
        State {
            id,
            name,
        }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl std::fmt::Display for State {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, "id: \"{}\", name: \"{}\"", self.id, self.name)

    } 

}

impl PartialEq for State {
    fn eq(&self, other:  &State) -> bool {
        self.id == other.id
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other:  &State) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other:  &State) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}
