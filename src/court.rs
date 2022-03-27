use crate::term::{Group, Term, Glyph};

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

impl Row {
    fn new() -> Row {
        Row {
            stack: Group::new(vec![]),
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
