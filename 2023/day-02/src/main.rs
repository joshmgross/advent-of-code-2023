// https://adventofcode.com/2023/day/2

// Pulling cubes from a bag
// Given a list of games
// Each game is listed with an ID number (Game 11: ...) followed by a semi-colon separated
// list of subsets of cubes
// A game is "possible" if it only contained 12 red cubes, 13 green cubes, and 14 blue cubes
// What is the sum of the IDs of the possible games?

// Part 2: For each game, find the fewest number of cubes of each color that could have been in the bag?
// The power of a set of cubes is equal to the number of red, green, and blue cubes multiplied together.
// What is the sum of the power of these sets?
use std::fs;

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Input not found");
    let games = input.lines();

    // Part 1: Sum of possible game IDs
    let mut possible_game_id_sum = 0;

    // Part 2: Sum of power of fewest cubes per game
    let mut min_cubes_power_sum = 0;

    for game in games {
        let (game_id, sets) = game.split_once(": ").unwrap();

        // Ignore the "Game " prefix
        let id: u32 = game_id
            .chars()
            .skip(5)
            .collect::<String>()
            .parse()
            .expect("Not a number");

        // println!("{}", id);

        let cube_sets = parse_sets(sets);
        if possible_game(&cube_sets) {
            possible_game_id_sum += id;
        }

        let min_cube_set = min_cubes(&cube_sets);
        let power = min_cube_set.red * min_cube_set.green * min_cube_set.blue;
        min_cubes_power_sum += power
    }

    println!("{}", possible_game_id_sum);
    println!("{}", min_cubes_power_sum);
}

fn parse_cube_set(raw_cube_set: &str) -> CubeSet {
    let mut ret = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };

    let color_num_pairs: Vec<&str> = raw_cube_set.split(",").map(|s| s.trim()).collect();
    for color_num_pair in color_num_pairs {
        let (number_string, color) = color_num_pair.split_once(" ").unwrap();

        let number: u32 = number_string.trim().parse().expect("Not a number");

        match color {
            "red" => ret.red = number,
            "green" => ret.green = number,
            "blue" => ret.blue = number,
            _ => panic!("Unexpected color {}", color),
        }
    }

    return ret;
}

fn parse_sets(sets: &str) -> Vec<CubeSet> {
    let mut cube_sets = Vec::new();
    let raw_cube_sets: Vec<&str> = sets.split(';').map(|s| s.trim()).collect();
    for raw_cube_set in raw_cube_sets {
        cube_sets.push(parse_cube_set(raw_cube_set));
    }

    return cube_sets;
}

fn possible_game(cube_sets: &Vec<CubeSet>) -> bool {
    for cube_set in cube_sets {
        if cube_set.red > RED_MAX || cube_set.green > GREEN_MAX || cube_set.blue > BLUE_MAX {
            return false;
        }
    }
    return true;
}

fn min_cubes(cube_sets: &Vec<CubeSet>) -> CubeSet {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for cube_set in cube_sets {
        if red < cube_set.red {
            red = cube_set.red;
        }
        if green < cube_set.green {
            green = cube_set.green
        }
        if blue < cube_set.blue {
            blue = cube_set.blue
        }
    }

    return CubeSet { red, blue, green };
}
