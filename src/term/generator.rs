use rand::prelude::*;

use super::{Term, Atom, Group};

pub struct TermGenerator { }

impl TermGenerator {
    pub fn new() -> Self {
        TermGenerator {
        }
    }

    pub fn generate_term(&self) -> Term {
        // return a random term
        let single: fn() -> Term = || Term::Single([
            || Atom::K, 
            || Atom::S, 
            || Atom::I, 
            || Atom::Y
        ].choose(&mut thread_rng()).unwrap()());
        [
            single, single, single,
            single, single, single,
            single, single, single,
            || Term::Group(Group::new(vec![None; 2])),
            || Term::Group(Group::new(vec![None; 3])),
        ].choose(&mut thread_rng()).unwrap()()
    }
}