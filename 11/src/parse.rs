use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline, space1};
use nom::combinator::map_res;
use nom::multi::{many1, separated_list0};
use nom::sequence::{pair, preceded, terminated, tuple};
use nom::IResult;

use crate::monkey::{Monkey, Operation, Value};

/// ITEMS := "  Starting items: " ( DIGIT+ | DIGIT+ ( ", " DIGIT+ )+ )
fn items(input: &str) -> IResult<&str, Vec<Value>> {
    let (remaining, _) = tag("  Starting items: ")(input)?;
    separated_list0(tag(", "), map_res(digit1, str::parse))(remaining)
}

/// OPERATION := "  Operation: new = old " ( "* old" | "* " DIGIT+ | "+ " DIGIT+ )
fn operation(input: &str) -> IResult<&str, Operation> {
    let (remaining, _) = tag("  Operation: new = old ")(input)?;
    let (remaining, parsed) = alt((
        pair(tag("*"), preceded(space1, tag("old"))),
        pair(tag("*"), preceded(space1, digit1)),
        pair(tag("+"), preceded(space1, digit1)),
    ))(remaining)?;

    Ok((
        remaining,
        match parsed {
            ("*", "old") => Operation::Square,
            ("*", value) => Operation::Multiply(value.parse().unwrap()),
            ("+", value) => Operation::Sum(value.parse().unwrap()),

            _ => unreachable!(),
        },
    ))
}

/// DIVISOR := "  Test: divisible by " DIGIT+
fn divisor(input: &str) -> IResult<&str, Value> {
    let (remaining, _) = tag("  Test: divisible by ")(input)?;
    map_res(digit1, str::parse)(remaining)
}

/// RESULT<T> := "    If " T ": throw to monkey " DIGIT+
fn result(truthy: bool) -> impl Fn(&str) -> IResult<&str, usize> {
    move |input: &str| {
        let (remaining, _) = tag("    If ")(input)?;
        let (remaining, _) = tag(truthy.to_string().as_str())(remaining)?;
        let (remaining, _) = tag(": throw to monkey ")(remaining)?;

        map_res(digit1, str::parse)(remaining)
    }
}

/// MONKEY := "Monkey " DIGIT+ ":" LF ITEMS LF OPERATION LF DIVISOR LF RESULT<"true"> LF RESULT<"false">
fn monkey(input: &str) -> IResult<&str, Monkey> {
    // Header
    let (remaining, _) = tag("Monkey ")(input)?;
    let (remaining, _) = digit1(remaining)?;
    let (remaining, _) = tag(":")(remaining)?;
    let (remaining, _) = newline(remaining)?;

    let (remaining, (items, operation, divisor, if_true, if_false)) = tuple((
        terminated(items, newline),
        terminated(operation, newline),
        terminated(divisor, newline),
        terminated(result(true), newline),
        result(false),
    ))(remaining)?;

    Ok((
        remaining,
        Monkey::new(items, operation, divisor, if_true, if_false),
    ))
}

/// TROOP := MONKEY | ( MONKEY ( LF+ MONKEY )+ )
pub fn troop(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list0(many1(newline), monkey)(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn items() {
        assert_eq!(super::items("  Starting items: "), Ok(("", vec![])));
        assert_eq!(super::items("  Starting items: 1"), Ok(("", vec![1])));
        assert_eq!(super::items("  Starting items: 1, 2"), Ok(("", vec![1, 2])));
    }

    #[test]
    fn operation() {
        assert_eq!(
            super::operation("  Operation: new = old * old"),
            Ok(("", super::Operation::Square))
        );

        assert_eq!(
            super::operation("  Operation: new = old * 123"),
            Ok(("", super::Operation::Multiply(123)))
        );

        assert_eq!(
            super::operation("  Operation: new = old + 123"),
            Ok(("", super::Operation::Sum(123)))
        );
    }

    #[test]
    fn divisor() {
        assert_eq!(super::divisor("  Test: divisible by 123"), Ok(("", 123)));
    }

    #[test]
    fn result() {
        assert_eq!(
            super::result(true)("    If true: throw to monkey 123"),
            Ok(("", 123))
        );

        assert_eq!(
            super::result(false)("    If false: throw to monkey 123"),
            Ok(("", 123))
        );
    }

    #[test]
    fn monkey() {
        let monkey: &str = concat!(
            "Monkey 123:\n",
            "  Starting items: 1, 2, 3\n",
            "  Operation: new = old + 1\n",
            "  Test: divisible by 3\n",
            "    If true: throw to monkey 123\n",
            "    If false: throw to monkey 456"
        );

        assert_eq!(
            super::monkey(monkey),
            Ok((
                "",
                super::Monkey::new(vec![1, 2, 3], super::Operation::Sum(1), 3, 123, 456)
            ))
        );
    }
}
