use super::{Term, Group, Atom};


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Glyph {
    Single(Atom), LParen, RParen, Unknown,
}

impl Group {
    pub fn to_glyphs(&self) -> Vec<Glyph> {
        let mut result = Vec::new();
        for t in &self.terms {
            match t {
                Some(t) => { result.append(&mut t.to_glyphs()) }
                None => { result.push(Glyph::Unknown) }
            }
        }
        result
    }
}

impl Term {
    pub fn to_glyphs(&self) -> Vec<Glyph> {
        match self {
            Term::Single(s) => { vec![Glyph::Single(s.clone())] }
            Term::Group(g) => { 
                let mut result = vec![];
                result.push(Glyph::RParen);
                result.append(&mut g.to_glyphs());
                result.push(Glyph::LParen);
                result
            }
        }
    }
}