use std::{fs, collections::HashSet};

pub fn main() {
    let pairings = parse_file_into_pairings("src/day_4/input.txt");
    println!(
        "Containment count: {}\nIntersection count: {}",
        pairings.iter().map(|pairing: &Vec<String>| {
            let set_one = get_section_hash_set(&pairing[0]);
            let set_two = get_section_hash_set(&pairing[1]);
            if set_one.is_subset(&set_two) || set_two.is_subset(&set_one) {
                1 as u16
            } else {0 as u16}
        }).sum::<u16>(),
        pairings.iter().map(|pairing: &Vec<String>| {
            let set_one = get_section_hash_set(&pairing[0]);
            let set_two = get_section_hash_set(&pairing[1]);
            if !set_one.is_disjoint(&set_two) {
                1 as u16
            } else {0 as u16}
        }).sum::<u16>()

    )
}

// Load the input file, split lines into strings by newline
fn parse_file_into_pairings(file_name: &str) -> Vec<Vec<String>> {
    let input: String = fs::read_to_string(file_name).expect("Could not read file");
    let lines = split_by_newline(&input);
    lines.iter().map(split_by_comma).collect::<Vec<Vec<String>>>()
}

fn split_by_newline(line: &str) -> Vec<String> {
    line.split("\n").map(|s| s.to_string()).collect()
}

fn split_by_comma(line: &String) -> Vec<String> {
    line.split(",").map(|s| s.to_string()).collect()
}


fn get_section_hash_set(line: &String) -> HashSet<i16> {
    let vals: Vec<i16> = line.split("-").map(|s| s.parse::<i16>().unwrap()).collect();
    let start = vals[0];
    let end = vals[1];
    HashSet::from_iter(start..=end)
}
