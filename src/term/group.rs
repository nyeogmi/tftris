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
        g.recalculate_settled();
        g.recalculate_n_nones();
        return g
    }

    pub fn push(&mut self, t: Term) {
        if self.try_accept(t.clone()) { return }
        self.terms.push(Some(t))
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

    fn recalculate_settled(&mut self) {
        self.settled = self._calculate_settled()
    }

    fn recalculate_n_nones(&mut self) {
        self.n_nones = self._calculate_n_nones()
    }

    fn _calculate_settled(&self) -> bool {
        let mut saw_none = false;
        for t in self.terms.iter() {
            match t {
                None => { saw_none = true }
                Some(t) => { 
                    if saw_none { return false }
                    match t {
                        Term::Single(_) => {}
                        Term::Group(g) => {
                            if !g.settled { return false }
                        }
                    }
                }
            }
        }
        return true
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
                let (t1, t2 ) = slice.split_at_mut(1);

                match &mut t1[0] {
                    Some(Term::Group(g)) => {
                        if let [Some(x)] = t2 {
                            if g.try_accept(x.clone()) {
                                t2[0] = None;
                                self.n_nones += 1;
                                return true
                            }
                        }
                    }
                    Some(Term::Single(_)) => {}
                    None => {
                        if t2[0].is_some() {
                            t1[0] = t2[0].take();
                            return true
                        }
                    }
                }
            }
        }
        return false
    }

    fn apply(&mut self) -> bool {
        match self.terms.pop() {
            None => { return false }
            Some(None) => { self.terms.push(None); return false }
            Some(Some(Term::Group(g))) => {
                if g.n_nones != 0 {
                    self.terms.push(Some(Term::Group(g)));
                    return false; 
                }
                self.terms.extend(g.terms);
                return true
            }
            Some(Some(Term::Single(s))) => {
                if s.apply(&mut self.terms) {
                    self._calculate_n_nones();
                    return true
                } else {
                    self.terms.push(Some(Term::Single(s)));
                    return false
                }
            }
        }
    }
}
