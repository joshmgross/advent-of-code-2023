// https://adventofcode.com/2024/day/2

fn parse_levels(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn safe(report: Vec<i32>) -> bool {
    if report.len() < 2 {
        return true;
    }

    let first = report[0];
    let second = report[1];
    if first == second {
        return false;
    }

    let increasing = if second > first { true } else { false };

    let mut prev = first;
    for level in report.into_iter().skip(1) {
        if increasing {
            if level < prev {
                return false;
            }
        } else {
            if level > prev {
                return false;
            }
        }

        if level.abs_diff(prev) < 1 || level.abs_diff(prev) > 3 {
            return false;
        }

        prev = level;
    }

    return true;
}

fn main() {
    let input = include_str!("../input.txt");

    let reports = input.lines().map(parse_levels).collect::<Vec<Vec<i32>>>();

    let mut num_safe = 0;
    for report in reports {
        if safe(report) {
            num_safe += 1
        }
    }
    println!("{}", num_safe);
}
