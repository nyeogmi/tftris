pub(crate) struct Court {
    pub(crate) width: usize,
    pub(crate) rows: Vec<Row>,

    pub(crate) progress_term: Option<ProgressTerm>,
}

pub(crate) struct ProgressTerm {
    value: Term,
    glyphs: Vec<Glyph>,
    x: usize,
    y: usize,
}

pub(crate) struct Row {
    stack: Group,
    glyphs: Vec<Glyph>
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) enum Term {
    Single(Atom),
    Group(Group),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct Group(Vec<Option<Term>>);

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) enum Glyph {
    Single(Atom), LParen, RParen, Unknown,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) enum Atom {
    K, S, I, Y
}

impl Group {
    pub(crate) fn empty(len: usize) -> Group {
        Group(vec![None; len])
    }

    fn push(&mut self, t: Term) {
        if self.try_accept(t.clone()) { return }
        self.0.push(Some(t))
    }

    fn trim(&mut self) {
        while self.0.last() == Some(&None) {
            self.0.pop();
        }
    }

    fn try_accept(&mut self, t: Term) -> bool {
        if self.0.len() == 0 {
            return false
        }
        let ix = self.0.len() - 1;
        match &mut self.0[ix] {
            None => { self.0[ix] = Some(t); return true }
            Some(Term::Single(_)) => { return false }
            Some(Term::Group(x)) => { x.try_accept(t) } 
        }
    }

    fn reduce(&mut self) -> bool {
        if self.settle() { return true }
        self.apply()
    }

    fn settle(&mut self) -> bool {
        for i in 0..self.0.len() {
            match &mut self.0[i] {
                Some(Term::Single(_)) => {}
                Some(Term::Group(g)) => {
                    if g.settle() { return true }
                }
                None => {}
            }

            if i + 1 < self.0.len() {
                let slice = &mut self.0[i..i+2];
                let (t1, t2 ) = slice.split_at_mut(1);

                match &mut t1[0] {
                    Some(Term::Group(g)) => {
                        if let [Some(x)] = t2 {
                            if g.try_accept(x.clone()) {
                                t2[0] = None;
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
        match self.0.pop() {
            None => { return false}
            Some(None) => { self.0.push(None); return false }
            Some(Some(Term::Group(g))) => {
                if g.0.iter().any(|x| x.is_none()) { 
                    self.0.push(Some(Term::Group(g)));
                    return false; 
                }
                self.0.extend(g.0);
                return true
            }
            Some(Some(Term::Single(s))) => {
                if s.apply(&mut self.0) {
                    return true
                } else {
                    self.0.push(Some(Term::Single(s)));
                    return false
                }
            }
        }
    }
}

impl Group {
    fn to_glyphs(&self) -> Vec<Glyph> {
        let mut result = Vec::new();
        for t in &self.0 {
            match t {
                Some(t) => { result.append(&mut t.to_glyphs()) }
                None => { result.push(Glyph::Unknown) }
            }
        }
        result
    }
}

impl Term {
    fn to_glyphs(&self) -> Vec<Glyph> {
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

impl Row {
    fn new() -> Row {
        Row {
            stack: Group(Vec::new()),
            glyphs: Vec::new()
        }
    }

    fn clearance(&self, width: usize) -> usize {
        if self.glyphs.len() >= width { return 0 }
        width - self.glyphs.len()
    }

    fn incorporate(&mut self, term: Term) {
        self.stack.push(term.clone());
        self.recalculate_glyphs()
    }

    fn reduce(&mut self) -> bool {
        let result = self.stack.reduce();
        if result { self.recalculate_glyphs() }
        return result
    }

    fn recalculate_glyphs(&mut self) {
        self.glyphs = self.stack.to_glyphs();
    }
}

impl Court {
    pub(crate) fn new(width: usize, height: usize) -> Court {
        let mut rows = Vec::new();
        for _ in 0..height {
            rows.push(Row::new());
        }
        Court {
            width,
            rows,
            progress_term: None
        }
    }

    pub(crate) fn update(&mut self, generate_term: impl Fn() -> Term) {
        if let Some(pt) = &mut self.progress_term {
            let row = &mut self.rows[pt.y];
            let clearance = row.clearance(self.width);
            if clearance <= pt.x {
                row.incorporate(pt.value.clone());
                self.progress_term = None;
                return
            } else {
                pt.x += 1;
            }
        }

        let mut all_settled = true;
        for i in self.rows.iter_mut() {
            if i.reduce() {
                all_settled = false
            }
        }

        if all_settled && self.progress_term.is_none() {
            let value = generate_term();
            let glyphs = value.to_glyphs();
            let pt = ProgressTerm {
                value,
                glyphs,
                x: 1,
                y: self.rows.len() / 2,
            };

            self.progress_term = Some(pt);
        }
    }

    pub(crate) fn move_y(&mut self, dy: i32) {
        if dy == 0 { return; }
        let sign = dy.signum();
        let steps = dy.abs() as usize;

        if let Some(pt) = &mut self.progress_term {
            for _ in 0..steps {
                let y2: i32 = pt.y as i32 + sign;
                if y2 < 0 { return }
                if y2 >= self.rows.len() as i32 { return }
                let y2 = y2 as usize;

                let row2 = &self.rows[y2];
                let clearance2 = row2.clearance(self.width);

                if clearance2 <= pt.x {
                    return
                } else {
                    pt.y = y2;
                }
            }
        }
    }

    pub(crate) fn fallfast(&mut self) {
        if let Some(pt) = &mut self.progress_term {
            pt.x = self.rows[pt.y].clearance(self.width);
        }
    }

    pub(crate) fn draw_grid(&self, draw: impl Fn(usize, usize, &Glyph)) {
        for (y, row) in self.rows.iter().enumerate() {
            for (i, g) in row.glyphs.iter().enumerate() {
                if i >= self.width { break }
                draw(self.width - 1 - i, y, g)
            }

            if let Some(pt) = &self.progress_term {
                if y == pt.y {
                    for (i, g) in pt.glyphs.iter().enumerate() {
                        if i >= pt.x { break }
                        draw(pt.x - 1 - i, y, g)
                    }
                }
            }
        }
    }
}

impl Atom {
    pub(crate) fn apply(&self, terms: &mut Vec<Option<Term>>) -> bool {
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
                    let x = xy[0].clone();
                    let _ = xy[1].clone();
                    terms.push(Some(x));
                    return true;
                }
            }
            Atom::S => {
                if let Some(xyz) = take(terms, 3) {
                    let x = xyz[0].clone();
                    let y = xyz[1].clone();
                    let z = xyz[2].clone();
                    terms.push(Some(Term::Group(Group(vec![Some(y), Some(z.clone())]))));
                    terms.push(Some(z));
                    terms.push(Some(x));
                    return true;
                }
            }
            Atom::Y => {
                if let Some(x) = take(terms, 1) {
                    let x = x[0].clone();
                    terms.push(Some(Term::Group(Group(vec![
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