use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::Hash,
    iter::Enumerate,
    ops::DerefMut,
};

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

pub fn d3_1(input: &str) -> u32 {
    input.lines().fold(0, |sum, bank_str| {
        let joltages_except_last: BTreeMap<_, _> = bank_str[..bank_str.len() - 1]
            .chars()
            .rev()
            .enumerate()
            .map(|(idx, c)| (idx, c.to_digit(10).unwrap()))
            .collect();

        // BTreeMap::max_by() returns the last occurrence of the max value, but we
        // want the first one, so that our search for the second digit works. So
        // we reverse the map before doing the max_by(). This gives us the index
        // of the first digit FROM THE END of the string.

        let (first_idx_from_end, first_joltage) = joltages_except_last
            .iter()
            .max_by(|lhs, rhs| lhs.1.cmp(rhs.1))
            .unwrap();

        let second_joltage = bank_str
            .chars()
            .skip(bank_str.len() - first_idx_from_end - 1)
            .map(|c| c.to_digit(10).unwrap())
            .max()
            .unwrap();

        let jolts: u32 = first_joltage * 10 + second_joltage;

        return sum + jolts;
    })
}

#[test]
fn test_d3_1() {
    println!("d3_1={}", d3_1(include_str!("day3.txt")));
}

pub fn d3_2(input: &str) -> u64 {
    input.lines().fold(0, |sum, bank_str| {
        // For each digit, our search range will be from the last digit's position + 1
        // to the end of the bank minus (12 - i), where i is the current digit index
        let mut range_start = 0;
        let mut jolts = 0;
        let multipliers = [
            100000000000 as u64,
            10000000000,
            1000000000,
            100000000,
            10000000,
            1000000,
            100000,
            10000,
            1000,
            100,
            10,
            1,
        ];
        for j in 0..12 {
            let range_end = bank_str.len() - (12 - j) + 1;
            let possible_joltages: BTreeMap<_, _> = bank_str[range_start..range_end]
                .chars()
                .rev()
                .enumerate()
                .map(|(idx, c)| (idx, c.to_digit(10).unwrap() as u64))
                .collect();

            let (idx_from_end, joltage) = possible_joltages
                .iter()
                .max_by(|lhs, rhs| lhs.1.cmp(rhs.1))
                .unwrap();

            range_start = range_end - idx_from_end;

            jolts += multipliers[j] * joltage;
        }

        return sum + jolts;
    })
}

#[test]
fn test_d3_2() {
    println!("d3_2={}", d3_2(include_str!("day3.txt")));
}

pub fn d4_1(input: &str) -> u64 {
    let map: HashMap<_, _> = input
        .lines()
        .enumerate()
        .fold(vec![], |mut out, (j, line)| {
            line.chars().enumerate().for_each(|(i, c)| {
                out.push((
                    (i as i32, j as i32),
                    match c {
                        '@' => true,
                        '.' => false,
                        _ => panic!(),
                    },
                ));
            });
            out
        })
        .into_iter()
        .collect();

    let height = input.lines().count() as i32;
    let width = input.lines().last().unwrap().chars().count() as i32;

    let rolls = map.iter().filter(|&(_, value)| *value);

    let mut accessible = 0;

    for (&(x, y), _) in rolls {
        let check_positions = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];
        let mut num_neighbors = 0;
        for (cx, cy) in check_positions {
            if cx < 0 || cx >= width {
                continue;
            }
            if cy < 0 || cy >= height {
                continue;
            }
            if map[&(cx, cy)] {
                num_neighbors += 1;
            }
        }
        // println!("{} {} -> {}", x, y, num_neighbors);

        if num_neighbors < 4 {
            accessible += 1;
        }
    }

    accessible
}

#[test]
fn test_d4_1() {
    println!("d4_1={}", d4_1(include_str!("day4.txt")));
}

pub fn d4_2(input: &str) -> u64 {
    let height = input.lines().count() as i32;
    let width = input.lines().last().unwrap().chars().count() as i32;

    let mut map: HashMap<_, _> = input
        .lines()
        .enumerate()
        .fold(vec![], |mut out, (j, line)| {
            line.chars().enumerate().for_each(|(i, c)| {
                out.push((
                    (i as i32, j as i32),
                    match c {
                        '@' => true,
                        '.' => false,
                        _ => panic!(),
                    },
                ));
            });
            out
        })
        .into_iter()
        .collect();

    let mut total_accessible = 0;

    loop {
        let map_snapshot = map.clone();
        // dbg!(&map_snapshot);
        let rolls = map_snapshot.iter().filter(|&(_, value)| *value);

        let mut accessible = 0;

        for (&(x, y), _) in rolls {
            let check_positions = [
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
                (x - 1, y),
                (x + 1, y),
                (x - 1, y + 1),
                (x, y + 1),
                (x + 1, y + 1),
            ];
            let mut num_neighbors = 0;
            for (cx, cy) in check_positions {
                if cx < 0 || cx >= width {
                    continue;
                }
                if cy < 0 || cy >= height {
                    continue;
                }
                if map_snapshot[&(cx, cy)] {
                    num_neighbors += 1;
                }
            }
            if num_neighbors < 4 {
                map.insert((x, y), false);
                accessible += 1;
            }
        }
        if accessible == 0 {
            break;
        }
        total_accessible += accessible;
    }

    total_accessible
}

#[test]
fn test_d4_2() {
    println!("d4_2={}", d4_2(include_str!("day4.txt")));
}

pub fn d5_1(input: &str) -> u64 {
    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();
    let ranges = ranges_str.lines().map(|line| {
        let (lo_str, hi_str) = line.split_once("-").unwrap();
        return (
            lo_str.parse::<i64>().unwrap(),
            hi_str.parse::<i64>().unwrap(),
        );
    });

    // combine ranges ?

    let ingredients = ingredients_str.lines().map(|id| id.parse::<i64>().unwrap());

    let mut num_fresh: u64 = 0;
    'ingredient_loop: for id in ingredients {
        for range in ranges.clone() {
            if id >= range.0 && id <= range.1 {
                num_fresh += 1;
                continue 'ingredient_loop;
            }
        }
    }

    num_fresh
}

#[test]
fn test_d5_1() {
    println!("d5_1={}", d5_1(include_str!("day5.txt")));
}

pub fn d5_2(input: &str) -> u64 {
    let (ranges_str, _) = input.split_once("\n\n").unwrap();
    let mut ranges = ranges_str
        .lines()
        .map(|line| {
            let (lo_str, hi_str) = line.split_once("-").unwrap();
            return (
                lo_str.parse::<u64>().unwrap(),
                hi_str.parse::<u64>().unwrap(),
            );
        })
        .collect::<Vec<_>>();

    // sorting by lower bound makes the rest easier
    ranges.sort_by(|x, y| x.0.cmp(&y.0));

    // represent combined ranges as map of start -> end
    let mut rangemap: HashMap<u64, u64> = HashMap::new();

    // combine ranges, then simply calculate sum of end - start
    for range in ranges.clone() {
        let mut low = range.0;
        let mut high = range.1;
        for other in rangemap.clone() {
            if low <= other.1 {
                // merge this range with the other one
                low = other.0;
                high = range.1.max(other.1);
            }
        }
        rangemap.insert(low, high);
    }

    let mut num_fresh = 0;
    for range in rangemap {
        num_fresh += range.1 - range.0 + 1
    }

    num_fresh
}

#[test]
fn test_d5_2() {
    println!("d5_2={}", d5_2(include_str!("day5.txt")));
}

pub fn d6_1(input: &str) -> u64 {
    // replace intervals between elements by a single space, to make splitting
    // easier.
    let cleaned_input: Vec<_> = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        .collect();

    let num_operands = cleaned_input.len() - 1;
    let operand_lines = &cleaned_input[..num_operands];
    let operators = &cleaned_input[num_operands];

    for op_list in operand_lines {
        assert_eq!(op_list.len(), operators.len());
    }

    let result = (0..operators.len()).fold(0, |sum, i| {
        let operator = operators[i];
        let operands = operand_lines
            .iter()
            .map(|op_line| op_line[i].parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let r = match operator {
            "+" => operands
                .iter()
                .fold(0, |sum, operand_str| sum + operand_str),
            "*" => operands
                .iter()
                .fold(1, |product, operand_str| product * operand_str),
            _ => panic!(),
        };
        return sum + r;
    });

    result
}

#[test]
fn test_d6_1() {
    println!("d6_1={}", d6_1(include_str!("day6.txt")));
}

pub fn d6_2(input: &str) -> u64 {
    // Parsing the input just got harder.

    // parse the numbers from right to left in column-wise fashion
    let num_operands = input.lines().count() - 1;
    let num_columns = input.lines().nth(0).unwrap().chars().count();

    // turn the input into columns
    let columns = (0..num_columns).rev().map(|i| {
        input
            .lines()
            .take(num_operands)
            .map(move |line| line.chars().nth(i).unwrap())
            .collect::<Vec<_>>()
    });

    let operand_sets = columns.fold(vec![vec![]], |mut operand_vec: Vec<Vec<u64>>, column| {
        if column.iter().all(|&c| c == ' ') {
            // Empty column -> move to next computation
            operand_vec.push(vec![]);
        } else {
            let operand = column.iter().fold(0, |sum, &c| {
                if c == ' ' {
                    sum
                } else {
                    sum * 10 + (c.to_digit(10).unwrap() as u64)
                }
            });
            operand_vec.last_mut().unwrap().push(operand);
        }

        return operand_vec;
    });

    // Operators are still ~easy if we use split_ascii_whitespace()
    #[derive(Debug)]
    enum Operator {
        Add,
        Multiply,
    }
    let operators: Vec<Operator> = input
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .rev()
        .map(|op_str| match op_str {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!(),
        })
        .collect();

    return operators
        .iter()
        .zip(operand_sets)
        .fold(0, |sum, (operator, operands)| {
            // dbg!(&operator, &operands);
            let delta = match operator {
                Operator::Add => operands.iter().fold(0, |sum, e| sum + e),
                Operator::Multiply => operands.iter().fold(1, |product, e| product * e),
            };
            // dbg!(&delta);
            sum + delta
        });
}

#[test]
fn test_d6_2() {
    println!("d6_2={}", d6_2(include_str!("day6.txt")));
}

pub fn d7_1(input: &str) -> usize {
    // the question is how many splitters the beam hits.
    // brute force -> simulate every split and propagate each beam, eliminating
    // double counts at every vertical step.

    fn beam_step(beams_in: &HashSet<usize>, line: &str) -> (usize, HashSet<usize>) {
        let mut beams_out = HashSet::new();
        let mut num_splits = 0;

        line.chars().enumerate().for_each(|(i, c)| {
            if beams_in.contains(&i) {
                match c {
                    '.' => {
                        beams_out.insert(i);
                    }
                    '^' => {
                        num_splits += 1;
                        if i > 0 {
                            beams_out.insert(i - 1);
                        }
                        if i < line.chars().count() - 1 {
                            beams_out.insert(i + 1);
                        }
                    }
                    _ => panic!(),
                }
            }
        });

        return (num_splits, beams_out);
    }

    let start_pos = input.lines().nth(0).unwrap().find('S').unwrap();

    let mut num_splits = 0;
    let mut beams = HashSet::from([start_pos]);

    for line in input.lines().skip(1) {
        let (delta_splits, newbeams) = beam_step(&beams, line);
        beams = newbeams;
        num_splits += delta_splits;
    }

    num_splits
}

#[test]
fn test_d7_1() {
    println!("d7_1={}", d7_1(include_str!("day7.txt")));
}

pub fn d7_2(input: &str) -> usize {
    let start_pos = input.lines().nth(0).unwrap().find('S').unwrap();

    // DFS, with memoization
    fn num_splits(
        beam_pos: usize,
        depth: usize,
        lines: &[&str],
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(memoed) = memo.get(&(beam_pos, depth)) {
            return *memoed;
        }

        if lines.len() == 0 {
            return 1;
        }
        if lines[0].chars().nth(beam_pos).unwrap() == '^' {
            if beam_pos < 1 {
                panic!();
            } else if beam_pos >= lines[0].chars().count() - 1 {
                panic!();
            } else {
                let result = num_splits(beam_pos - 1, depth + 1, &lines[1..], memo)
                    + num_splits(beam_pos + 1, depth + 1, &lines[1..], memo);
                memo.insert((beam_pos, depth), result);
                return result;
            }
        } else {
            let result = num_splits(beam_pos, depth + 1, &lines[1..], memo);
            memo.insert((beam_pos, depth), result);
            return result;
        }
    }

    num_splits(
        start_pos,
        0,
        &input.lines().collect::<Vec<_>>(),
        &mut HashMap::new(),
    )
}

#[test]
fn test_d7_2() {
    println!("d7_2={}", d7_2(include_str!("day7.txt")));
}

pub fn d8_1(input: &str) -> usize {
    type Position = (i64, i64, i64);
    let positions = input
        .lines()
        .map(|line| {
            let mut elements = line.split(',');
            return (
                elements.next().unwrap().parse::<i64>().unwrap(),
                elements.next().unwrap().parse::<i64>().unwrap(),
                elements.next().unwrap().parse::<i64>().unwrap(),
            );
        })
        .collect::<Vec<_>>();

    let mut pairs: BTreeMap<i64, (Position, Position)> = BTreeMap::new();

    // first step is to order each pair of boxes by their euclidean distance
    for left in positions.clone() {
        for right in positions.clone() {
            if left != right {
                let distance = (left.0 - right.0) * (left.0 - right.0)
                    + (left.1 - right.1) * (left.1 - right.1)
                    + (left.2 - right.2) * (left.2 - right.2);
                pairs.insert(distance, (left, right));
            }
        }
    }

    let mut circuits: HashMap<Position, i32> = (0..positions.len())
        .map(|i| (positions[i], -1))
        .collect::<HashMap<_, _>>();

    let mut ngroups = 0;

    for (_, (left, right)) in pairs.iter().take(1000) {
        let lgrp = *circuits.get(left).unwrap();
        let rgrp = *circuits.get(right).unwrap();

        if lgrp == -1 && rgrp == -1 {
            circuits.insert(*left, ngroups);
            circuits.insert(*right, ngroups);
            ngroups += 1;
        } else if lgrp == -1 {
            circuits.insert(*left, rgrp);
        } else if rgrp == -1 {
            circuits.insert(*right, lgrp);
        } else if rgrp == lgrp {
        } else {
            // both have groups -> change all entries with group == rgrp to left group
            circuits
                .iter_mut()
                .filter(|(_, v)| **v == rgrp)
                .for_each(|(_, grp)| *grp = lgrp);
        }
    }

    let mut counts = vec![];
    for i in 0..ngroups {
        counts.push(circuits.values().filter(|group_id| **group_id == i).count());
    }
    // dbg!(&counts);
    counts.sort();
    counts.reverse();
    counts.iter().take(3).fold(1, |product, e| product * e)
}

#[test]
fn test_d8_1() {
    println!("d8_1={}", d8_1(include_str!("day8.txt")));
}

pub fn d8_2(input: &str) -> usize {
    type Position = (i64, i64, i64);
    let positions = input
        .lines()
        .map(|line| {
            let mut elements = line.split(',');
            return (
                elements.next().unwrap().parse::<i64>().unwrap(),
                elements.next().unwrap().parse::<i64>().unwrap(),
                elements.next().unwrap().parse::<i64>().unwrap(),
            );
        })
        .collect::<Vec<_>>();

    let mut pairs: BTreeMap<i64, (Position, Position)> = BTreeMap::new();

    // first step is to order each pair of boxes by their euclidean distance
    for left in positions.clone() {
        for right in positions.clone() {
            if left != right {
                let distance = (left.0 - right.0) * (left.0 - right.0)
                    + (left.1 - right.1) * (left.1 - right.1)
                    + (left.2 - right.2) * (left.2 - right.2);
                pairs.insert(distance, (left, right));
            }
        }
    }

    let mut circuits: HashMap<Position, i32> = (0..positions.len())
        .map(|i| (positions[i], -1))
        .collect::<HashMap<_, _>>();

    let mut ngroups = 0;

    for (_, (left, right)) in pairs.iter() {
        let lgrp = *circuits.get(left).unwrap();
        let rgrp = *circuits.get(right).unwrap();

        if lgrp == -1 && rgrp == -1 {
            circuits.insert(*left, ngroups);
            circuits.insert(*right, ngroups);
            ngroups += 1;
        } else if lgrp == -1 {
            circuits.insert(*left, rgrp);
        } else if rgrp == -1 {
            circuits.insert(*right, lgrp);
        } else if rgrp == lgrp {
        } else {
            // both have groups -> change all entries with group == rgrp to left group
            circuits
                .iter_mut()
                .filter(|(_, v)| **v == rgrp)
                .for_each(|(_, grp)| *grp = lgrp);
        }

        if circuits.values().all(|val| *val == lgrp) {
            return (left.0 * right.0).try_into().unwrap();
        }
    }

    0
}

#[test]
fn test_d8_2() {
    println!("d8_2={}", d8_2(include_str!("day8.txt")));
}
