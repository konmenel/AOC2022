use aoc::read_inputs;
use lazy_static::lazy_static;
use std::collections::HashMap;

type InputT = Vec<String>;

lazy_static! {
    static ref DIGITS: HashMap<char, i64> = [('=', -2), ('-', -1), ('0', 0), ('1', 1), ('2', 2),]
        .iter()
        .copied()
        .collect();
}

lazy_static! {
    static ref DIGITS_REV: HashMap<i64, char> =
        [(-2, '='), (-1, '-'), (0, '0'), (1, '1'), (2, '2'),]
            .iter()
            .copied()
            .collect();
}

#[inline]
fn pow5(pow: u32) -> i64 {
    match pow {
        0 => 1,
        1 => 5,
        2 => 25,
        3 => 125,
        4 => 625,
        5 => 3125,
        6 => 15625,
        _ => 5i64.pow(pow),
    }
}

fn snafu(num: &str) -> i64 {
    // SNAFU to decimal
    let mut total = 0;
    for (i, d) in num.chars().rev().enumerate() {
        total += DIGITS[&d] * pow5(i as u32);
    }
    total
}

fn ufans(num: i64) -> String {
    // decimal to SNAFU (or SNAFU in reverse)

    let mut digits = vec![]; // digits in reverse order (highest power last)
    let mut n = 0;
    digits.push(0);

    loop {
        if let Some(i) = digits.iter().position(|&x| x > 2) {
            n = i;
            if n == digits.len() - 1 {
                digits.push(1);
            } else {
                digits[n + 1] += 1;
            }
            digits[n] += -5;
            n = 0;
            continue;
        }

        let x: i64 = digits
            .iter()
            .enumerate()
            .map(|(n, &x)| x * pow5(n as u32))
            .sum();

        if x == num {
            break;
        }
        let rem = num - x;

        while rem / pow5(n as u32) > 5 {
            n += 1;
            if n > digits.len() - 1 {
                digits.push(0);
            }
        }

        digits[n] += rem / pow5(n as u32);

        if n != 0 {
            n -= 1;
        }
    }
    let digits: String = digits.into_iter().rev().map(|d| DIGITS_REV[&d]).collect();

    digits
}

fn part1(input: &InputT) {
    let mut sum = 0;

    #[cfg(debug_assertions)]
    println!("SNAFU\tDecimal");

    for num in input {
        #[cfg(debug_assertions)]
        println!("{}\t{}", num, snafu(num));

        sum += snafu(num);
    }

    let ufans_sum = ufans(sum);

    #[cfg(debug_assertions)]
    println!("\nsum = {sum}");

    println!("{ufans_sum}");
}

fn main() {
    let day: u32 = 25;
    #[cfg(debug_assertions)]
    let file_path = format!("data/examples/{:02}.txt", day);

    #[cfg(not(debug_assertions))]
    let file_path = format!("data/{:02}.txt", day);

    let input: InputT = read_inputs(&file_path).unwrap();
    println!("FINAL RESULT:");
    part1(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(dead_code)]
    fn ufans_test() {
        const NUMBERS: [i64; 8] = [
          6545, 138571, 9532, 412, 935172, i64::MAX - 1000, 12498, 123
        ];

        for n in NUMBERS {
            let forward = ufans(n);
            let backward = snafu(&forward);
            assert_eq!(n, backward);
        }
    }
}
