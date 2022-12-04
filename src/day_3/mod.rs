use std::str;
use std::collections::{HashSet, HashMap};

pub fn main() {
    let input = include_str!("input.txt");
    let char_map = get_char_map();
    println!(
        "{}\n{}",
        input.split("\n").map(|s| get_score(s, &char_map)).sum::<u16>(),
        input.split("\n")
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(|grp| get_group_score(grp, &char_map))
            .sum::<u16>()
    );

}

fn get_char_map() -> HashMap<char, u16> {
    let mut char_map = HashMap::new();
    ('a'..='z')
        .into_iter()
        .enumerate()
        .for_each(|(i, char)| {char_map.insert(char, (i+1) as u16);});
    ('A'..='Z')
        .into_iter()
        .enumerate()
        .for_each(|(i, char)| {char_map.insert(char, (i+27) as u16);});
    return char_map
}

fn get_hash_set(s: &str) -> HashSet<char> {
    s.chars().collect()
}

fn get_score(s: &str, char_map: &HashMap<char, u16>) -> u16{
    let chunk_size: usize = s.chars().count() / 2 as usize;
    let string_array = s.as_bytes()
        .chunks(chunk_size)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();
    let s1 = get_hash_set(string_array[0]);
    let s2 = get_hash_set(string_array[1]);
    let intersection: Vec<&char> = s1.intersection(&s2).collect();
    intersection.iter().map(|c| char_map.get(c).cloned().unwrap_or(0 as u16)).sum::<u16>()
}

fn get_group_score(group: &[&str], char_map: &HashMap<char, u16>) -> u16 {
    group
        .iter()
        .map(|s| get_hash_set(s))
        .reduce(
            |a, b| {
            HashSet::from_iter(a.intersection(&b).into_iter().cloned())
            })
        .unwrap()
        .iter()
        .map(|c| char_map.get(c).cloned().unwrap_or(0 as u16)).sum::<u16>()
}
