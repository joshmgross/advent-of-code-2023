// https://adventofcode.com/2024/day/3
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.txt");

    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // let mut fields: Vec<(i64, i64)> = vec![];
    let mut sum = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|caps| caps.extract()) {
        sum += a.parse::<i32>()? * b.parse::<i32>()?;
    }

    println!("{}", sum);

    Ok(())
}
