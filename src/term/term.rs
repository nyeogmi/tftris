use super::{Atom, Group};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Term {
    Single(Atom),
    Group(Group),
}