use crate::term::Group;

use super::{Atom, Term};

impl Atom {
    pub fn apply(&self, terms: &mut Vec<Option<Term>>) -> bool {
        fn take(terms: &mut Vec<Option<Term>>, i: usize) -> Option<Vec<Term>> {
            if terms.len() < i { return None }
            for i in terms[terms.len() - i..].iter() {
                if i.is_none() { return None }
            }
            let mut t2 = vec![];
            for i in terms.drain(terms.len() - i..) {
                t2.push(i.unwrap())
            }
            Some(t2)
        }

        match self {
            Atom::K => {
                if let Some(xy) = take(terms, 2) {
                    let x = xy[1].clone();
                    let _ = xy[0].clone();
                    terms.push(Some(x));
                    return true;
                }
            }
            Atom::S => {
                if let Some(xyz) = take(terms, 3) {
                    let x = xyz[2].clone();
                    let y = xyz[1].clone();
                    let z = xyz[0].clone();
                    terms.push(Some(Term::Group(Group::new(vec![Some(z.clone()), Some(y)]))));
                    terms.push(Some(z));
                    terms.push(Some(x));
                    return true;
                }
            }
            Atom::Y => {
                if let Some(x) = take(terms, 1) {
                    let x = x[0].clone();
                    terms.push(Some(Term::Group(Group::new(vec![
                        Some(x.clone()),
                        Some(Term::Single(Atom::Y)), 
                    ]))));
                    terms.push(Some(x));
                    return true;
                }
            }
            Atom::I => {
                if let Some(x) = take(terms, 1) {
                    let x = x[0].clone();
                    terms.push(Some(x));
                    return true;
                }
            }
        }

        return false
        // todo!()
    }
}

