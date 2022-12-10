use crate::moves::Move;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct RopeEnd(isize, isize);

impl RopeEnd {
    pub fn new() -> Self {
        RopeEnd(0, 0)
    }

    pub fn movement(&mut self, movement: Move) {
        match movement {
            Move::Right(distance) => self.0 += distance as isize,
            Move::Up(distance) => self.1 += distance as isize,
            Move::Left(distance) => self.0 -= distance as isize,
            Move::Down(distance) => self.1 -= distance as isize,
        };
    }

    fn is_stable(&self, other: &Self) -> bool {
        let (x, y) = (self.0, self.1);
        let (i, j) = (other.0, other.1);

        i >= x - 1 && i <= x + 1 && j >= y - 1 && j <= y + 1
    }

    pub fn follow(&mut self, other: &Self) -> Vec<RopeEnd> {
        let mut locations: Vec<RopeEnd> = vec![*self];

        while !self.is_stable(other) {
            let delta_x = other.0 - self.0;
            let delta_y = other.1 - self.1;

            self.0 += delta_x.signum();
            self.1 += delta_y.signum();

            locations.push(*self);
        }

        locations
    }
}
