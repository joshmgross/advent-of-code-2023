// https://adventofcode.com/2024/day/5

use std::collections::{HashMap, HashSet};

fn is_ordered(update: &Vec<usize>, rules: &HashMap<usize, HashSet<usize>>) -> bool {
    let mut seen = HashSet::new();
    for page in update {
        seen.insert(page);

        let after_pages = rules.get(&page);
        if after_pages.is_none() {
            continue;
        }

        for after_page in after_pages.unwrap().iter() {
            if seen.contains(after_page) {
                return false;
            }
        }
    }

    return true;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.txt");

    // X|Y
    let mut rules = HashMap::new();

    // X,Y,Z
    let mut updates = vec![];

    let mut reading_rules = true;
    for line in input.lines() {
        if line == "" {
            reading_rules = false;
            continue;
        }

        if reading_rules {
            let rule: Vec<&str> = line.split("|").collect();

            let set = rules
                .entry(rule[0].parse::<usize>()?)
                .or_insert(HashSet::new());
            set.insert(rule[1].parse::<usize>()?);

            continue;
        }

        let mut update = vec![];
        for page in line.split(",") {
            update.push(page.parse::<usize>()?);
        }
        updates.push(update);
    }
    // println!("{:?}", rules);

    let mut part_one_sum = 0;
    for update in updates {
        let ordered = is_ordered(&update, &rules);
        // println!("{:?} is ordered {:?}", update, ordered);

        if ordered {
            let middle = update[update.len() / 2];
            // println!("Middle: {}", middle);

            part_one_sum += middle;
        }
    }

    println!("{}", part_one_sum);

    Ok(())
}
