// https://adventofcode.com/2023/day/1
// Given an input string, return a number that represents the first and last digits that appear in
// the string.
// A digit could be a numerical digit or the digit spelled out ("one", "two", etc).
// two1nine => 29
// eightwothree => 83
use std::fs;

fn main() {
    let calibration_values = fs::read_to_string("./input.txt").expect("Input not found");
    let inputs = calibration_values.lines();

    let mut sum = 0;
    for input in inputs {
        let num = calibration_value(input);
        sum = sum + num;
    }

    println!("{}", calibration_value("eightwothree"));
    println!("{}", sum);
}

fn calibration_value(v: &str) -> u32 {
    let mut first_digit = 0;
    let mut last_digit = 0;

    let mut last_three = String::from("");
    let mut last_four = String::from("");
    let mut last_five = String::from("");

    for c in v.chars() {
        last_three.push(c);
        if last_three.len() > 3 {
            last_three.remove(0);
        }

        last_four.push(c);
        if last_four.len() > 4 {
            last_four.remove(0);
        }

        last_five.push(c);
        if last_five.len() > 5 {
            last_five.remove(0);
        }

        let d = digit(c, &last_three, &last_four, &last_five);
        if d.is_none() {
            continue;
        }

        if first_digit == 0 {
            first_digit = d.unwrap();
        }

        last_digit = d.unwrap();
    }

    return first_digit * 10 + last_digit;
}

// I'm sure there's a better way to do this
fn digit(c: char, last_three: &String, last_four: &String, last_five: &String) -> Option<u32> {
    if c.is_numeric() {
        return c.to_digit(10);
    }

    match last_three.as_str() {
        "one" => return Some(1),
        "two" => return Some(2),
        "six" => return Some(6),
        _ => {}
    }

    match last_four.as_str() {
        "four" => return Some(4),
        "five" => return Some(5),
        "nine" => return Some(9),
        _ => {}
    }

    match last_five.as_str() {
        "three" => return Some(3),
        "seven" => return Some(7),
        "eight" => return Some(8),
        _ => {}
    }

    None
}
