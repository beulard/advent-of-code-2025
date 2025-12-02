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

#[cfg(test)]
#[test]
fn test_d1_1() {
    println!("d1_1={}", d1_1(include_str!("day1.txt")));
}
#[test]
fn test_d1_2() {
    println!("d1_2={}", d1_2(include_str!("day1.txt")));
}
