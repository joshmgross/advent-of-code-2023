// https://adventofcode.com/2024/day/3
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.txt");

    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don\'t\(\))").unwrap();

    let mut enabled = true;
    let mut sum_part_one = 0;
    let mut sum_part_two = 0;
    for cap in re.captures_iter(input) {
        match &cap[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                let a = &cap[1].parse::<i32>()?;
                let b = &cap[2].parse::<i32>()?;
                sum_part_one += a * b;
                if enabled {
                    sum_part_two += a * b;
                }
            }
        }
    }

    println!("{}", sum_part_one);
    println!("{}", sum_part_two);

    Ok(())
}
