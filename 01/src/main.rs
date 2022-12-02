use std::io;

#[derive(PartialEq)]
enum LineInput {
    Value(u32),
    EndOfRecord,
    EndOfFile,
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    let mut calories = Vec::new(); // Calories for each elf
    let mut current = 0; // Sum of calories for the current elf

    loop {
        // Read the line from stdin and determine the record type
        let record = match stdin.read_line(&mut line) {
            Ok(0) => LineInput::EndOfFile,

            Ok(_) => match line.as_str() {
                "\n" => LineInput::EndOfRecord,
                value => match value.trim().parse::<u32>() {
                    Ok(x) => LineInput::Value(x),
                    Err(_) => LineInput::Value(0),
                },
            },

            Err(e) => panic!("{e}"),
        };

        // Process the record
        match record {
            LineInput::Value(x) => current += x,

            _ => {
                calories.push(current);
                current = 0;
            }
        };

        // Break the loop when we reach the EOF
        if record == LineInput::EndOfFile {
            break;
        }

        line.clear();
    }

    // Sum the top three elves' calories
    calories.sort();
    match &calories[..] {
        [.., a, b, c] => println!("{}", a + b + c),
        _ => panic!("Not enough elves"),
    };
}
