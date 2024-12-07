// https://adventofcode.com/2024/day/4

use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let mut word_search: Vec<Vec<char>> = vec![vec![]];
    for line in input.lines() {
        word_search.push(line.chars().collect())
    }

    let mut total = 0;
    for i in 0..word_search.len() {
        for j in 0..word_search[i].len() {
            total += find_xmas(&word_search, i, j)
        }
    }

    print!("{}", total);
}

// Returns the number of times "XMAS" appears at the given location
fn find_xmas(word_search: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    if word_search[i][j] != 'X' {
        return 0;
    }

    let mut found = 0;
    for found_m in find_adjacent_char(word_search, i, j, 'M') {
        for found_a in find_adjacent_char(word_search, found_m.0, found_m.1, 'A') {
            for found_s in find_adjacent_char(word_search, found_a.0, found_a.1, 'S') {
                let points = vec![(i, j), found_m, found_a, found_s];
                println!("{:?} {:?}", points, word(word_search, &points));
                found += 1;
            }
        }
    }
    return found;
}

// find_adjacent_char determines if search_char is in any adjacent squares to Point
// Returns an array of Points where character was found
fn find_adjacent_char(
    word_search: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    search_char: char,
) -> HashSet<(usize, usize)> {
    let mut found_chars: HashSet<(usize, usize)> = HashSet::new();

    for x_diff in -1..1 {
        for y_diff in -1..1 {
            let i: isize = (i as isize) + x_diff;
            let j: isize = (j as isize) + y_diff;
            if i.is_negative() || j.is_negative() {
                continue;
            }

            if i == 0 && j == 0 {
                continue;
            }

            let i = i as usize;
            let j = j as usize;
            if i >= word_search.len() || j >= word_search[i].len() {
                continue;
            }

            if word_search[i][j] == search_char {
                found_chars.insert((i, j));
            }
        }
    }
    return found_chars;
}

fn word(word_search: &Vec<Vec<char>>, points: &Vec<(usize, usize)>) -> String {
    let mut chars = vec![];
    for (i, j) in points {
        chars.push(word_search[i.clone()][j.clone()]);
    }

    return chars.iter().collect();
}
