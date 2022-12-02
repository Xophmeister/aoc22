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

    let mut sum = 0; // Local sum
    let mut max = 0; // Current maximum

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
            LineInput::Value(x) => sum += x,

            _ => {
                if sum > max {
                    max = sum;
                }
                sum = 0;
            }
        };

        // Break the loop when we reach the EOF
        if record == LineInput::EndOfFile {
            break;
        }

        line.clear();
    }

    println!("{max}");
}
