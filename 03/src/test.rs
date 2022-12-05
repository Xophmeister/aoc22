#[cfg(test)]
use crate::*;

#[test]
fn test_rucksack_compartments() {
    assert_eq!(rucksack_compartments("a1"), ("a", "1"));
    assert_eq!(rucksack_compartments("abc123"), ("abc", "123"));
    assert_eq!(
        rucksack_compartments("vJrwpWtwJgWrhcsFMMfFFhFp"),
        ("vJrwpWtwJgWr", "hcsFMMfFFhFp")
    );
}

#[test]
fn test_common_item() {
    assert_eq!(common_item(("a", "a")), Some('a'));
    assert_eq!(common_item(("vJrwpWtwJgWr", "hcsFMMfFFhFp")), Some('p'));
    assert_eq!(common_item(("abc", "ABC")), None);
    assert_eq!(common_item(("abc", "abc")), None);
}

#[test]
fn test_item_priority() {
    assert_eq!(item_priority('a'), 1);
    assert_eq!(item_priority('z'), 26);
    assert_eq!(item_priority('A'), 27);
    assert_eq!(item_priority('Z'), 52);
}

#[test]
fn test_rucksack_priority() {
    assert_eq!(rucksack_priority("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
    assert_eq!(rucksack_priority("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 38);
    assert_eq!(rucksack_priority("PmmdzqPrVvPwwTWBwg"), 42);
    assert_eq!(rucksack_priority("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 22);
    assert_eq!(rucksack_priority("ttgJtRGJQctTZtZT"), 20);
    assert_eq!(rucksack_priority("CrZsJsPPZsGzwwsLwLmpwMDw"), 19);
}
