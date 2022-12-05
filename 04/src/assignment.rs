use std::str::FromStr;

pub struct Assignment(u32, u32);

impl FromStr for Assignment {
    type Err = std::num::ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (from, to) = input.split_once("-").unwrap();
        Ok(Assignment(from.parse::<u32>()?, to.parse::<u32>()?))
    }
}

impl Assignment {
    pub fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let assignment = Assignment::from_str("1-2").unwrap();
        assert_eq!(assignment.0, 1);
        assert_eq!(assignment.1, 2);
    }

    #[test]
    fn containment() {
        let a = Assignment(0, 10);
        let b = Assignment(3, 7);
        let c = Assignment(8, 15);

        assert!(a.contains(&b));
        assert!(!a.contains(&c));
    }
}
