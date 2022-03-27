use rand::prelude::*;

use super::{Term, Atom, Group, atom::Potion, Caste, Rank};

pub struct TermGenerator { }

impl TermGenerator {
    pub fn new() -> Self {
        TermGenerator {
        }
    }

    pub fn generate_term(&self) -> Term {
        // return a random term
        term()
    }
}

fn term() -> Term {
    [
        potion, potion, potion,
        meat, meat,
        meat, meat,
        critter, critter,
        tftrigger, tftrigger,
        empty_group_2,
        empty_group_3,
    ].choose(&mut thread_rng()).unwrap()()
}

fn potion() -> Term { 
    Term::Single([
        || Atom::Potion(Potion::K), 
        || Atom::Potion(Potion::K), 
        || Atom::Potion(Potion::K), 
        || Atom::Potion(Potion::S), 
        || Atom::Potion(Potion::S), 
        || Atom::Potion(Potion::I), 
        || Atom::Potion(Potion::I), 
        || Atom::Potion(Potion::Y),
    ].choose(&mut thread_rng()).unwrap()())
}

fn meat() -> Term {
    Term::Single(Atom::Meat)
}

fn caste() -> Caste {
    [
        || Caste::Bird,
        || Caste::Dog,
        || Caste::Fish,
        || Caste::Reptile,
        || Caste::Robot,
    ].choose(&mut thread_rng()).unwrap()()
}

/*
fn rank() -> Rank {
    [
        || Rank::Baby,
        || Rank::Adult,
        || Rank::Elder,
    ].choose(&mut thread_rng()).unwrap()()
}
*/

fn critter() -> Term {
    Term::Single(
        Atom::Critter(caste(), Rank::Baby),
    )
}

fn tftrigger() -> Term {
    Term::Single(
        Atom::TFTrigger(caste()),
    )
}

fn empty_group_2() -> Term {
    Term::Group(Group::new(vec![None; 2]))
}

fn empty_group_3() -> Term {
    Term::Group(Group::new(vec![None; 3]))
}