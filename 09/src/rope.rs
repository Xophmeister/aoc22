use std::ops::Index;

use crate::moves::Move;

#[derive(Copy, Clone, Default, Eq, Hash, PartialEq)]
pub struct Knot(isize, isize);

impl Knot {
    fn travel(&mut self, movement: Move) {
        match movement {
            Move::Right => self.0 += 1,
            Move::Up => self.1 += 1,
            Move::Left => self.0 -= 1,
            Move::Down => self.1 -= 1,
        };
    }

    fn is_stable(&self, other: &Self) -> bool {
        let (x, y) = (self.0, self.1);
        let (i, j) = (other.0, other.1);

        (x - i).abs() <= 1 && (y - j).abs() <= 1
    }

    fn follow(&mut self, other: &Self) {
        while !self.is_stable(other) {
            let delta_x = other.0 - self.0;
            let delta_y = other.1 - self.1;

            self.0 += delta_x.signum();
            self.1 += delta_y.signum();
        }
    }
}

pub struct Rope(Vec<Knot>, usize);

impl Rope {
    pub fn new(knots: usize) -> Self {
        Rope(vec![Knot::default(); knots], knots)
    }

    pub fn simulate(&mut self, movement: Move) {
        self.0[0].travel(movement);

        for i in 1..self.1 {
            if let ([.., prior], [this, ..]) = self.0.split_at_mut(i) {
                this.follow(prior);
            }
        }
    }
}

impl Index<usize> for Rope {
    type Output = Knot;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}
