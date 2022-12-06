use std::collections::HashSet;
use std::io;

const MARKER_SIZE: usize = 4;

fn unique_characters(input: &str) -> usize {
    let chars: HashSet<char> = HashSet::from_iter(input.chars());
    chars.len()
}

fn find_start(input: &str) -> Result<usize, &str> {
    let length = input.len();

    if length < MARKER_SIZE {
        return Err("Input too short");
    }

    for idx in MARKER_SIZE..=length {
        let frame = &input[idx - MARKER_SIZE..idx];
        if unique_characters(frame) == MARKER_SIZE {
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
            Ok(_) => println!("{}", find_start(line.trim()).unwrap()),
            Err(_) => panic!("Parse error!"),
        };

        line.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_start() {
        assert_eq!(find_start("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 5);
        assert_eq!(find_start("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 6);
        assert_eq!(find_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 10);
        assert_eq!(find_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 11);
    }
}
