mod test;

use std::collections::HashSet;
use std::io;

fn rucksack_compartments(rucksack: &str) -> (&str, &str) {
    // Split the rucksack string into its equal compartments
    let len = rucksack.len();
    assert!(len % 2 == 0);

    (&rucksack[..len / 2], &rucksack[len / 2..])
}

fn common_item(compartments: (&str, &str)) -> Option<char> {
    // Find the single common item (if any) contained in each compartment
    let (left, right) = compartments;
    let a: HashSet<char> = HashSet::from_iter(left.chars());
    let b: HashSet<char> = HashSet::from_iter(right.chars());

    // common.first() returns Option<&&char>; applying .cloned() twice gets us the correct type,
    // but doesn't have the correct behaviour when there are multiple common items.
    let common: Vec<&char> = a.intersection(&b).collect();
    match common.len() {
        1 => Some(common[0].clone()),
        _ => None,
    }
}

fn item_priority(item: char) -> u32 {
    // Convert item characters into their priority values
    // NOTE We rely on codepoints and the Roman alphabet being ordered sequentially
    match item {
        'a'..='z' => (item as u32) - ('a' as u32) + 1,
        'A'..='Z' => (item as u32) - ('A' as u32) + 27,

        _ => panic!("Unknown item"),
    }
}

fn rucksack_priority(rucksack: &str) -> u32 {
    if let Some(item) = common_item(rucksack_compartments(rucksack)) {
        item_priority(item)
    } else {
        panic!("No common item")
    }
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    let mut sum = 0;

    loop {
        sum += match stdin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => rucksack_priority(&line.trim()),

            Err(e) => panic!("{e}"),
        };

        line.clear();
    }

    println!("{sum}");
}
