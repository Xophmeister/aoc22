use crate::assignment::Assignment;
use std::str::FromStr;

pub struct Record(Assignment, Assignment);

impl FromStr for Record {
    type Err = std::num::ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (a, b) = input.split_once(",").unwrap();
        Ok(Record(a.parse::<Assignment>()?, b.parse::<Assignment>()?))
    }
}

impl Record {
    pub fn redundant(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    pub fn overlaps(&self) -> bool {
        self.0.overlaps(&self.1) || self.1.overlaps(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn containment() {
        let a = Record::from_str("1-10,3-8").unwrap();
        let b = Record::from_str("1-10,5-15").unwrap();
        assert!(a.redundant());
        assert!(!b.redundant());
    }
}
