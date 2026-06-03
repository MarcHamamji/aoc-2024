use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("=== Part 1 ===");

    let input = std::fs::read_to_string("src/bin/day-01/input.txt").expect("Unable to read file");

    let mut left_list: Vec<u32> = vec![];
    let mut right_list: Vec<u32> = vec![];

    input.trim().split('\n').for_each(|line| {
        let mut values = line
            .split("   ")
            .map(|value| value.parse::<u32>().expect("Unable to parse number"));

        let left_num: u32 = values.next().expect("Unable to get first value of line");
        let right_num: u32 = values.next().expect("Unable to get first value of line");

        left_list.push(left_num);
        right_list.push(right_num);
    });

    left_list.sort();
    right_list.sort();

    let zip = std::iter::zip(left_list, right_list);
    let differences = zip.map(|values| (values.1 as i32 - values.0 as i32).abs());
    let sum: i32 = differences.sum();

    println!("Sum: {sum}");
}

fn part2() {
    println!("=== Part 2 ===");

    let input = std::fs::read_to_string("src/bin/day-01/input.txt").expect("Unable to read file");

    let mut left_list: Vec<u32> = vec![];
    let mut right_list: Vec<u32> = vec![];

    input.trim().split('\n').for_each(|line| {
        let mut values = line
            .split("   ")
            .map(|value| value.parse::<u32>().expect("Unable to parse number"));

        let left_num: u32 = values.next().expect("Unable to get first value of line");
        let right_num: u32 = values.next().expect("Unable to get first value of line");

        left_list.push(left_num);
        right_list.push(right_num);
    });

    let mut map: HashMap<u32, u32> = HashMap::new();
    let mut score = 0;

    left_list.iter().for_each(|value| {
        if let Some(value_score) = map.get(&value) {
            score += value_score;
        } else {
            let mut value_score = 0;
            right_list.iter().for_each(|v| {
                if *v == *value {
                    value_score += 1
                }
            });
            value_score *= value;
            map.insert(*value, value_score);

            score += value_score;
        }
    });

    println!("Score: {score}");
}
