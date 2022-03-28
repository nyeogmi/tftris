// this is the ugliest code in the project
// sorry in advance!!!
use super::Term;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Group {
    settled: bool,
    n_nones: usize,
    pub(in super) terms: Vec<Option<Term>>
}

impl Group {
    pub fn new(terms: Vec<Option<Term>>) -> Group {
        let mut g = Group { 
            settled: false,
            n_nones: 0,
            terms,
        }; 
        g.recalculate_n_nones();
        return g
    }

    pub fn push(&mut self, t: Term) {
        if self.try_accept(t.clone()) { return }
        self.terms.push(Some(t));
        self.settled = false;
    }

    fn try_accept(&mut self, t: Term) -> bool {
        if self.terms.len() == 0 {
            return false
        }
        let ix = self.terms.len() - 1;
        match &mut self.terms[ix] {
            None => { 
                self.terms[ix] = Some(t); 
                self.settled = false;
                self.n_nones -= 1;
                return true 
            }
            Some(Term::Single(_)) => { return false }
            Some(Term::Group(x)) => { 
                if x.try_accept(t) {
                    self.settled = false;
                    return true
                }
                return false
            } 
        }
    }

    pub fn reduce(&mut self) -> bool {
        if self.settle() { return true }
        self.apply()
    }

    fn recalculate_n_nones(&mut self) {
        self.n_nones = self._calculate_n_nones()
    }

    fn _calculate_n_nones(&self) -> usize {
        self.terms.iter().filter(|i| i.is_none()).count()
    }

    fn settle(&mut self) -> bool {
        if self.settled { return false; }
        let did_work = self._settle();
        self.settled = !did_work;
        return did_work
    }

    fn _settle(&mut self) -> bool {
        for i in 0..self.terms.len() {
            match &mut self.terms[i] {
                Some(Term::Single(_)) => {}
                Some(Term::Group(g)) => {
                    if g.settle() { return true }
                }
                None => {}
            }

            if i + 1 < self.terms.len() {
                let slice = &mut self.terms[i..i+2];

                let n_nones_prior = slice[0].is_none() as usize + slice[1].is_none() as usize;

                let tx = slice[1].take();
                let ty = slice[0].take();

                let (changed, tx_new, ty_new) = match (tx, ty) {
                    (Some(x), None) => {
                        (true, None, Some(x))
                    }
                    (Some(x), Some(Term::Group(mut y))) => {
                        if y.try_accept(x.clone()) {
                            (true, None, Some(Term::Group(y)))
                        } else {
                            let y = Term::Group(y);
                            if x.weight().is_some() && y.weight().is_some() && x.weight() > y.weight() {
                                (true, Some(y), Some(x))
                            } else {
                                (false, Some(x), Some(y))
                            }
                        }
                    }
                    (Some(x), Some(y)) => {
                        if x.weight().is_some() && y.weight().is_some() && x.weight() > y.weight() {
                            (true, Some(y), Some(x))
                        } else {
                            (false, Some(x), Some(y))
                        }
                    }
                    (None, y) => { (false, None, y) }
                };

                slice[1] = tx_new;
                slice[0] = ty_new;

                let n_nones_post = slice[0].is_none() as usize + slice[1].is_none() as usize;
                self.n_nones += n_nones_post;
                self.n_nones -= n_nones_prior;

                if changed {
                    return true;
                }
            }
        }

        return false;
    }

    fn apply(&mut self) -> bool {
        match self.terms.pop() {
            None => { return false }
            Some(None) => { self.terms.push(None); return false }
            Some(Some(Term::Group(g))) => {
                if g.n_nones != 0 {
                    let result = self.apply();
                    self.terms.push(Some(Term::Group(g)));
                    return result; 
                }
                self.terms.extend(g.terms);
                return true
            }
            Some(Some(Term::Single(s))) => {
                if s.apply(&mut self.terms) {
                    self._calculate_n_nones();
                    return true
                } else {
                    let result = self.apply();
                    self.terms.push(Some(Term::Single(s)));
                    return result
                }
            }
        }
    }
}
