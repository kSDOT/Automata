use crate::compound_state::*;
use std::rc::Rc;

pub type Connection = (Rc<CompoundState>, char);
