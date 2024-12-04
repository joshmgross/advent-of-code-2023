// https://adventofcode.com/2024/day/2

fn parse_levels(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[derive(PartialEq)]
enum SafeResult {
    Ok,
    InvalidIndex(usize),
}

fn safe(report: &Vec<i32>) -> SafeResult {
    if report.len() < 2 {
        return SafeResult::Ok;
    }

    let first = report[0];
    let second = report[1];
    if first == second {
        return SafeResult::InvalidIndex(1);
    }

    let increasing = if second > first { true } else { false };

    let mut prev = first;
    for i in 1..report.len() {
        let level = report[i];
        if increasing {
            if level < prev {
                return SafeResult::InvalidIndex(i);
            }
        } else {
            if level > prev {
                return SafeResult::InvalidIndex(i);
            }
        }

        if level.abs_diff(prev) < 1 || level.abs_diff(prev) > 3 {
            return SafeResult::InvalidIndex(i);
        }

        prev = level;
    }

    return SafeResult::Ok;
}

fn solve(reports: &Vec<Vec<i32>>, dampener: bool) -> usize {
    let mut num_safe = 0;
    let mut num_dampened_safe = 0;
    for report in reports {
        match safe(&report) {
            SafeResult::Ok => num_safe += 1,
            SafeResult::InvalidIndex(i) => {
                if dampener {
                    let mut new_report = report.clone();
                    new_report.remove(i);

                    let mut new_report_two = report.clone();
                    new_report_two.remove(i - 1);
                    if safe(&new_report) == SafeResult::Ok
                        || safe(&new_report_two) == SafeResult::Ok
                    {
                        num_dampened_safe += 1;
                        continue;
                    }

                    // The first level may have given us the wrong direction
                    if i == 2 {
                        let mut new_report_three = report.clone();
                        new_report_three.remove(0);
                        if safe(&new_report_three) == SafeResult::Ok {
                            num_dampened_safe += 1;
                        }
                    }
                }
            }
        }
    }

    return num_safe + num_dampened_safe;
}

fn main() {
    let input = include_str!("../input.txt");

    let reports = input.lines().map(parse_levels).collect::<Vec<Vec<i32>>>();

    println!("part 1: {}", solve(&reports, false));
    println!("part 2: {}", solve(&reports, true));
}
