use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// modulo with -22 % 100 == 78
// lord forgive me for I have sinned with these type casts.
// I do know they're problematic but not for this problem.
fn modulo(rhs: isize, mod_: usize) -> usize {
    let res = rhs % mod_ as isize;
    if res < 0 {
        (res + mod_ as isize) as usize
    } else {
        res as usize
    }
}

fn main() {
    let mut args = env::args();

    if args.len() != 2 {
        eprintln!("usage: {} <file>", args.next().unwrap());
        return;
    }

    // skip argv[0]
    _ = args.next();

    let file_name = args.next().unwrap();
    let Ok(file) = File::open(file_name) else {
        eprintln!("does this file even exist?? couldnt open!");
        return;
    };

    let reader = BufReader::new(file);

    let mut dial: u8 = 50;
    let mut password: usize = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.chars();

        let direction = match line.next() {
            Some('L') => -1,
            Some('R') => 1,
            _ => panic!("invalid direction"),
        };

        let mut number: usize = 0;

        for (idx, digit) in line.rev().enumerate() {
            let digit = digit.to_digit(10).expect("invalid digit");

            number += usize::try_from(digit).expect("should be run on at least 32 bits")
                * 10usize.pow(u32::try_from(idx).expect("no 32 bits???"));
        }

        let dial_ = usize::try_from(dial).expect("need 32 bit!");

        dial = u8::try_from(modulo(
            isize::from(dial)
                + isize::try_from(direction).expect("32 bit invariant")
                    * isize::try_from(number).unwrap(),
            100,
        ))
        .expect("should be less than 100!!");

        if dial_ + number > 100 && direction == 1 {
            password += (dial_ + number - 1) / 100;
        } else if dial_ < number && direction == -1 {
            password += number / 100
                + if number % 100 > dial_ && dial_ != 0 {
                    1
                } else {
                    0
                };
        }

        if dial == 0 {
            password += 1;
        }
    }

    println!("your password for day 1 act 2 is: {}", password);
}
