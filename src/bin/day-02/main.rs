fn main() {
    part1();
    part2();
}

fn part1_is_report_valid(values: &Vec<u32>) -> bool {
    let len = values.len();
    if len == 1 {
        return true;
    }

    let difference = values[1] as i64 - values[0] as i64;
    let distance = difference.abs();
    if distance < 1 || distance > 3 {
        return false;
    }

    let mut old_sign = difference.signum();

    for i in 1..(len - 1) {
        let difference = values[i + 1] as i64 - values[i] as i64;
        if difference.signum() != old_sign {
            return false;
        }
        let distance = difference.abs();
        if distance < 1 || distance > 3 {
            return false;
        }
        old_sign = difference.signum();
    }

    true
}

fn part1() {
    println!("=== Part 1 ===");

    let input = std::fs::read_to_string("src/bin/day-02/input.txt").expect("Unable to read file");

    let safe_reports = input
        .trim()
        .split('\n')
        .map(|line| {
            line.split(" ")
                .map(|v| v.parse::<u32>().expect("Unable to parse number"))
                .collect::<Vec<u32>>()
        })
        .filter(|report| part1_is_report_valid(report))
        .count();

    println!("Number of safe reports: {safe_reports}");
}

fn part2_is_report_valid(values: &Vec<u32>) -> bool {
    let len = values.len();
    if len == 1 {
        return true;
    }

    if part1_is_report_valid(values) {
        return true;
    }

    for i in 0..len {
        let mut new_values = values.clone();
        new_values.remove(i);
        if part1_is_report_valid(&new_values) {
            return true;
        }
    }

    return false;
}

fn part2() {
    println!("=== Part 2 ===");

    let input = std::fs::read_to_string("src/bin/day-02/input.txt").expect("Unable to read file");

    let safe_reports = input
        .trim()
        .split('\n')
        .map(|line| {
            line.split(" ")
                .map(|v| v.parse::<u32>().expect("Unable to parse number"))
                .collect::<Vec<u32>>()
        })
        .filter(|report| part2_is_report_valid(report))
        .count();

    println!("Number of safe reports: {safe_reports}");
}
