use std::env;

#[derive(Debug)]
struct Divisors {
    num: usize,
    pivot: usize,
    cache: usize, // this is really ugly and hacky
}

impl Divisors {
    fn new(num: usize) -> Self {
        let pivot = num.isqrt();

        Divisors {
            num,
            pivot,
            cache: 0,
        }
    }
}

/// returns all integer divisors, no order guarantees.
/// at the moment, order is (d, num/d) pairs with d descending
impl Iterator for Divisors {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // lord have mercy on my soul
        if self.cache != 0 {
            let cache = self.cache;
            self.cache = 0;

            return Some(cache);
        }

        if self.pivot == 0 {
            return None;
        }

        while self.num % self.pivot != 0 {
            if self.pivot == 0 {
                return None;
            }
            self.pivot -= 1;
        }

        let num = self.pivot;
        self.cache = self.num / num;

        self.pivot -= 1;

        Some(num)
    }
}

#[cfg(feature = "second_stage")]
#[cfg(not(feature = "first_stage"))]
fn is_valid(id: usize) -> bool {
    if id <= 10 {
        return true;
    }

    let digit_count = id.ilog10() + 1;
    let divisors = Divisors::new(digit_count as usize);

    for divisor in divisors {
        if divisor == digit_count as usize {
            continue;
        }

        let repetition_count = digit_count / divisor as u32;
        let cut_id = id / 10usize.pow(digit_count - divisor as u32) as usize;
        let mut reconstructed_by_repetition = 0;

        for idx in 0..repetition_count {
            let space_power = idx * divisor as u32;
            let space = 10usize.pow(space_power as u32);

            reconstructed_by_repetition += space * cut_id;
        }

        if id == reconstructed_by_repetition {
            return false;
        }
    }
    return true;
}

#[cfg(feature = "first_stage")]
#[cfg(not(feature = "second_stage"))]
fn is_valid(id: usize) -> bool {
    let log = id.ilog10() + 1;
    if log % 2 != 0 {
        return false;
    }

    let half_digit_count_base_10 = log / 2;
    let id_half = id / (10u32.pow(half_digit_count_base_10)) as usize;
    let base = (10u32.pow(half_digit_count_base_10) as usize * id_half) as usize;

    if cfg!(debug_assertions) {
        dbg!(id, id_half, half_digit_count_base_10, base, id_half + base);
    }

    return id_half + base != id;
}

fn main() {
    let mut args = env::args();

    if args.len() != 2 {
        eprintln!("usage: {} <file>", args.next().unwrap());
        return;
    }

    _ = args.next();

    let ranges = args.next().unwrap();
    let mut invalid_ids = 0;

    for range_spec in ranges.split_terminator(",") {
        let (start, end) = range_spec.split_once("-").expect("invalid range spec!");
        let (start, end) = (
            start.parse::<usize>().expect("not a valid usize!"),
            end.parse::<usize>().expect("not a valid usize!"),
        );

        let range = start..=end;

        for num in range {
            if !is_valid(num) {
                if cfg!(debug_assertions) {
                    println!("invalid id: {num}");
                }
                invalid_ids += num;
            }
        }
    }

    println!("here's your answer: {}", invalid_ids);
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "first_stage")]
    #[cfg(not(feature = "second_stage"))]
    mod first_stage {
        use super::super::is_valid;

        #[test]
        fn test_basic_is_not_valid() {
            let id = 11;
            assert!(!is_valid(id));
        }

        #[test]
        fn test_basic_is_valid() {
            let id = 12;
            assert!(is_valid(id));
        }

        #[test]
        fn test_weird_is_invalid() {
            let id = 1010;
            assert!(!is_valid(id));
        }

        #[test]
        fn test_bigger_is_valid() {
            let id = 1234567729320302;
            assert!(is_valid(id));
        }

        #[test]
        fn test_bigger_is_invalid() {
            let id = 12345671234567;
            assert!(!is_valid(id));
        }
    }
}
