/// Advent of Code 2025 in rust 🦀 :)

pub fn d1_1(input: &str) -> u64 {
    let mut dial_pos: i32 = 50;
    let mut num_zeros: u64 = 0;
    let instructions = input
        .lines()
        .map(|x| x.split_at(1))
        .map(|(direction, distance)| match direction {
            "L" => -distance.parse::<i32>().unwrap(),
            "R" => distance.parse::<i32>().unwrap(),
            _ => panic!(),
        });

    instructions.for_each(|delta| {
        dial_pos = (dial_pos + delta).rem_euclid(100);
        if dial_pos == 0 {
            num_zeros += 1;
        }
    });

    return num_zeros;
}

#[test]
fn test_d1_1() {
    println!("d1_1={}", d1_1(include_str!("day1.txt")));
}

pub fn d1_2(input: &str) -> u64 {
    let instructions = input
        .lines()
        .map(|x| x.split_at(1))
        .map(|(direction, distance)| match direction {
            "L" => -distance.parse::<i32>().unwrap(),
            "R" => distance.parse::<i32>().unwrap(),
            _ => panic!(),
        });

    let (_, total) = instructions.fold((50, 0), |(dial_pos, num_zeros), delta| {
        let next_pos = (dial_pos + delta).rem_euclid(100);

        let inc = if delta < 0 {
            // Going left from x is like going right from (100 - x) % 100
            ((100 - dial_pos).rem_euclid(100) - delta).div_euclid(100) as u64
        } else if delta > 0 {
            (dial_pos + delta).div_euclid(100) as u64
        } else {
            panic!();
        };
        (next_pos, num_zeros + inc)
    });

    return total;
}

#[test]
fn test_d1_2() {
    println!("d1_2={}", d1_2(include_str!("day1.txt")));
}

pub fn d2_1(input: &str) -> u64 {
    fn left_is_right(x: &str) -> bool {
        let length = x.len();
        if length % 2 != 0 {
            return false;
        }
        let (left, right) = x.split_at(length / 2);
        return left == right;
    }
    let sum = input.split(",").fold(0, |sum, range| {
        let (lo_str, hi_str) = range.split_once('-').unwrap();
        let (lo, hi) = (
            lo_str.parse::<u64>().unwrap(),
            hi_str.parse::<u64>().unwrap(),
        );
        let mut total = 0;
        for x in lo..hi + 1 {
            let x_str = x.to_string();
            if left_is_right(&x_str) {
                total += x;
            }
        }
        sum + total
    });
    sum
}

#[test]
fn test_d2_1() {
    println!("d2_1={}", d2_1(include_str!("day2.txt")));
}

pub fn d2_2(input: &str) -> u64 {
    fn is_repeated(x: &str) -> bool {
        let length = x.len();
        for i in 1..length / 2 + 1 {
            if length % i != 0 {
                continue;
            }
            let mut ok = true;
            for j in 0..(length / i) {
                if x[j * i..(j + 1) * i] != x[..i] {
                    ok = false;
                    break;
                }
            }
            if ok {
                // println!("{:?} {}", i, x);
                return true;
            }
        }
        false
    }
    let sum = input.split(",").fold(0, |sum, range| {
        let (lo_str, hi_str) = range.split_once('-').unwrap();
        let (lo, hi) = (
            lo_str.parse::<u64>().unwrap(),
            hi_str.parse::<u64>().unwrap(),
        );
        let mut total = 0;
        for x in lo..hi + 1 {
            let x_str = x.to_string();
            if is_repeated(&x_str) {
                total += x;
            }
        }
        sum + total
    });
    sum
}

#[test]
fn test_d2_2() {
    println!("d2_2={}", d2_2(include_str!("day2.txt")));
}
