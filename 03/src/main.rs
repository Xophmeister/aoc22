mod test;

use std::collections::HashSet;
use std::io;

fn rucksack_compartments(rucksack: &str) -> (&str, &str) {
    // Split the rucksack string into its equal compartments
    let len = rucksack.len();
    assert!(len % 2 == 0);

    (&rucksack[..len / 2], &rucksack[len / 2..])
}

fn common_item(collections: Vec<&str>) -> Option<char> {
    // Find the single common item (if any) amongst N collections of items
    let mut items: Vec<HashSet<char>> = collections
        .iter()
        .map(|collection| HashSet::from_iter(collection.chars()))
        .collect();

    // From https://stackoverflow.com/a/65175232/876937
    let mut common = items.pop().unwrap();
    common.retain(|item| items.iter().all(|set| set.contains(item)));

    let item = Vec::from_iter(common);
    match item.len() {
        1 => Some(item[0]),
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

fn group_priority(collections: Vec<&str>) -> u32 {
    if let Some(item) = common_item(collections) {
        item_priority(item)
    } else {
        panic!("No common item")
    }
}

fn rucksack_priority(rucksack: &str) -> u32 {
    let (left, right) = rucksack_compartments(rucksack);
    group_priority(vec![left, right])
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    let mut sum_a = 0; // Answer for part 1
    let mut sum_b = 0; // Answer for part 2

    let mut group: Vec<String> = Vec::new();

    loop {
        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Err(e) => panic!("{e}"),

            Ok(_) => {
                let record = line.trim();

                // Part 1
                sum_a += rucksack_priority(record);

                // Part 2
                // TODO There's probably a better way of doing this!
                group.push(String::from(record));
                if group.len() == 3 {
                    let group_ref: Vec<&str> = group.iter().map(|elf| elf.as_str()).collect();
                    sum_b += group_priority(group_ref);
                    group.clear();
                }
            }
        };

        line.clear();
    }

    println!("Part 1: {sum_a}");
    println!("Part 2: {sum_b}");
}
