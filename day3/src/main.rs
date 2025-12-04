use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

fn part_one(lines: Lines<BufReader<File>>) -> u32 {
    let mut total_joltage = 0;

    for line in lines {
        let line = line.unwrap();
        let mut first_digit = 0;
        let mut first_digit_idx = 0;
        let mut second_digit = 0;

        let first_pass = line.chars();
        let second_pass = line.chars();

        for (idx, chr) in first_pass.rev().skip(1).enumerate() {
            let digit = chr.to_digit(10).expect("invalid character (not a digit!)");

            if digit >= first_digit {
                first_digit = digit;
                first_digit_idx = idx;
            }
        }

        first_digit_idx = line.len() - 1 - first_digit_idx;

        for chr in second_pass.skip(first_digit_idx) {
            let digit = chr.to_digit(10).expect("invalid character (not a digit!)");

            if digit > second_digit {
                second_digit = digit;
            }
        }

        total_joltage += first_digit * 10 + second_digit;
    }

    total_joltage
}

/// computes the answer to the second stage, which is
/// "what is the best joltage I can get by choosing exactly 12 batteries in the array?"
///
/// it does so by greedily choosing the largest digit for the current decimal place of the
/// max joltage per line. the largest digit to be considered is the digit from the index
/// of the last digit chosen until a specific index in the remaining battery array, making
/// sure to always leave enough batteries for all of the other to-be-filled decimal places
/// in the resulting configuration. sounds cryptic, I know. as an example:
///
/// assuming the battery array is 95831 and we're choosing the best 3. obv. the configuration
/// would be '983'. this algorithm would choose this number thusly:
///
/// - consider 9, 5, 8 (once we have the first digit chosen, we need two more; 3 and 1 can't be
///   correct and are thus not considered for this place.)
/// - the biggest of those is 9, so the next time we start with 5.
/// - consider 5, 8, 3 (same thing here, we can't choose 1 because that would leave us with no
///   numbers for the last digit)
/// - the biggest of those is 8, so the next time we start looking at 3 (we simply skip 5)
/// - consider 3, 1 for the last digit
/// - the biggest is 3. we're done choosing digits.
///
/// the result is 9 * 100 + 8 * 10 + 3 = 983.
fn part_two(lines: Lines<BufReader<File>>) -> u64 {
    const MAX_BATTERY_COUNT: usize = 12;
    let mut total_joltage = 0;

    for line in lines {
        let line = line.expect("invalid line");
        let bytes = line.as_bytes();

        let mut choosing_index_idx = 0;
        let mut smolest_index = 0;
        let line_length = line.len();
        let mut max_joltage = 0;

        while choosing_index_idx < MAX_BATTERY_COUNT {
            let mut max_digit = 0;
            let range_beginning = smolest_index;
            let range_end = line_length - (MAX_BATTERY_COUNT - choosing_index_idx) + 1;
            let considered_range = range_beginning..range_end;

            for (idx, chr) in bytes[considered_range].iter().enumerate() {
                if (*chr as u32 - b'0' as u32) > max_digit {
                    max_digit = *chr as u32 - b'0' as u32;
                    smolest_index = range_beginning + idx + 1;
                }
            }

            max_joltage +=
                10u64.pow((MAX_BATTERY_COUNT - choosing_index_idx) as u32 - 1) * max_digit as u64;

            choosing_index_idx += 1;
        }

        total_joltage += max_joltage;
    }

    total_joltage
}

fn main() {
    let mut args = env::args();

    if args.len() != 2 {
        eprintln!("usage: {} <file>", args.next().unwrap());
        return;
    }

    _ = args.next();

    let file_name = args.next().unwrap();
    let Ok(file) = File::open(file_name) else {
        eprintln!("could not open file!");
        return;
    };

    let reader = BufReader::new(file);
    let total_joltage = part_two(reader.lines());

    println!("here's your answer: {}", total_joltage);
}
