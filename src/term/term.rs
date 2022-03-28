use super::{Atom, Group};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Term {
    Single(Atom),
    Group(Group),
}

impl Term {
    pub fn weight(&self) -> Option<usize> {
        match self {
            Term::Single(a) => Some(a.weight()),
            Term::Group(_) => None,
        }
    }
}