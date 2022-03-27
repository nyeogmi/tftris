use crate::term::Group;

use super::{Atom, Term, atom::Potion};

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

fn transform(terms: &mut Vec<Option<Term>>, alter: impl Fn(&Term) -> Option<Term>) -> bool {
    if terms.len() == 0 { return false; }

    let ix = terms.len() - 1;
    if terms[ix].is_none() { return false; }
    let subj = terms[ix].as_ref().unwrap();

    let subj2 = 
    if let Some(subj2) = alter(subj) {
        if &subj2 != subj { 
            subj2
        } else {
            return false;
        }
    } else {
        return false;
    };
    terms[ix] = Some(subj2);
    return true
}

impl Atom {
    pub fn apply(&self, terms: &mut Vec<Option<Term>>) -> bool {
        match self {
            Atom::Potion(Potion::K) => {
                if let Some(xy) = take(terms, 2) {
                    let x = xy[1].clone();
                    let _ = xy[0].clone();
                    terms.push(Some(x));
                    return true;
                }
            }
            Atom::Potion(Potion::S) => {
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
            Atom::Potion(Potion::Y) => {
                if let Some(x) = take(terms, 1) {
                    let x = x[0].clone();
                    terms.push(Some(Term::Group(Group::new(vec![
                        Some(x.clone()),
                        Some(Term::Single(Atom::Potion(Potion::Y))), 
                    ]))));
                    terms.push(Some(x));
                    return true;
                }
            }
            Atom::Potion(Potion::I) => {
                if let Some(x) = take(terms, 1) {
                    let x = x[0].clone();
                    terms.push(Some(x));
                    return true;
                }
            }

            Atom::Meat => {
                return transform(terms, |x| {
                    match x {
                        Term::Single(Atom::Critter(caste, rank)) => {
                            Some(Term::Single(Atom::Critter(caste.clone(), rank.next())))
                        },
                        _ => None,
                    }
                });
            }

            Atom::Critter(_, _) => {
                return false;
            }

            Atom::TFTrigger(new_caste) => {
                return transform(terms, |x| {
                    match x {
                        Term::Single(Atom::Critter(_, rank)) => {
                            Some(Term::Single(Atom::Critter(new_caste.clone(), rank.clone())))
                        },
                        _ => None,
                    }
                });
            }
        }

        return false
        // todo!()
    }
}

