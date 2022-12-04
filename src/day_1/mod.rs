use std::fs;

pub fn main() {
    let max_cals: u64 = get_max_calories();
    let top_three_cals: u64 = get_top_three_max_calories();
    println!("{}", max_cals);
    println!("{}", top_three_cals);
    
}

fn get_max_calories() -> u64 {
    let input: String = fs::read_to_string("src/day_1/input.txt").expect("Couldn't read file!");
    let splits: Vec<&str> = input.split("\n\n").collect();
    let mut max_cals: u64 = 0;
    for chunk in splits {
        let num_strings: Vec<&str> = chunk.split("\n").collect();
        let total_cals: u64 = num_strings.iter().map(|x| x.parse::<u64>().unwrap()).sum();
        max_cals = max_cals.max(total_cals)
    }
    return max_cals;
}

fn get_top_three_max_calories() -> u64 {
    let input: String = fs::read_to_string("src/day_1/input.txt").expect("Couldn't read file!");
    let splits: Vec<&str> = input.split("\n\n").collect();
    let mut cals: Vec<u64> = vec![];
    for chunk in splits {
        let num_strings: Vec<&str> = chunk.split("\n").collect();
        let total_cals: u64 = num_strings.iter().map(|x| x.parse::<u64>().unwrap()).sum();
        cals.push(total_cals)
    }
    cals.sort_unstable();
    cals.reverse();
    let sum: u64 = cals[0..3].iter().sum();
    return sum
}
