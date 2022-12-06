use std::collections::HashSet;
use std::io;

const PKT_MARKER_SIZE: usize = 4;
const MSG_MARKER_SIZE: usize = 14;

fn unique_characters(input: &str) -> usize {
    let chars: HashSet<char> = HashSet::from_iter(input.chars());
    chars.len()
}

fn find_start(input: &str, marker_size: usize) -> Result<usize, &str> {
    let length = input.len();

    if length < marker_size {
        return Err("Input too short");
    }

    for idx in marker_size..=length {
        let frame = &input[idx - marker_size..idx];
        if unique_characters(frame) == marker_size {
            return Ok(idx);
        }
    }

    Err("Marker not found")
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Err(_) => panic!("Parse error!"),

            Ok(_) => {
                println!(
                    "Part 1: {}",
                    find_start(line.trim(), PKT_MARKER_SIZE).unwrap()
                );

                println!(
                    "Part 2: {}",
                    find_start(line.trim(), MSG_MARKER_SIZE).unwrap()
                );
            }
        };

        line.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_start() {
        let cases = vec![
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (input, expected) in cases {
            assert_eq!(find_start(input, PKT_MARKER_SIZE).unwrap(), expected);
        }
    }

    #[test]
    fn message_start() {
        let cases = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (input, expected) in cases {
            assert_eq!(find_start(input, MSG_MARKER_SIZE).unwrap(), expected);
        }
    }
}
