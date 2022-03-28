#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Atom {
    Potion(Potion),
    Meat,
    Critter(Caste, Rank),
    TFTrigger(Caste),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Potion {
    K, S, I, Y
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Caste {
    Bird, Dog, Fish, Reptile, Robot
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Rank {
    Baby, Adult, Elder
}

impl Rank {
    pub(crate) fn next(&self) -> Rank {
        match self {
            Rank::Baby => Rank::Adult,
            Rank::Adult => Rank::Elder,
            Rank::Elder => Rank::Baby,
        }
    }
}
impl Atom {
    pub(crate) fn weight(&self) -> usize {
        match self {
            // potions are very light
            Atom::Potion(_) => 1,  

            // things that go on top of critters
            Atom::Meat => 2,
            Atom::TFTrigger(_) => 2,

            // heavy things
            Atom::Critter(_, _) => 3,  
        }
    }
}