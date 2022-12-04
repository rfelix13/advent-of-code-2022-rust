
pub fn main() {
    let input = include_str!("input.txt");
    println!(
        "{}\n{}",
        input
            .split("\n")
            .map(match_one)
            .sum::<i16>(),
        input
            .split("\n")
            .map(match_two)
            .sum::<i16>(),
    );
}

fn match_one(s: &str) -> i16 {
    match s {
        "A X" => 4 as i16,
        "A Y" => 8 as i16,
        "A Z" => 3 as i16,
        "B X" => 1 as i16,
        "B Y" => 5 as i16,
        "B Z" => 9 as i16,
        "C X" => 7 as i16,
        "C Y" => 2 as i16,
        "C Z" => 6 as i16,
        _ => panic!()
    }
}

fn match_two(s: &str) -> i16 {
    match s {
        "A X" => 3 as i16,
        "A Y" => 4 as i16,
        "A Z" => 8 as i16,
        "B X" => 1 as i16,
        "B Y" => 5 as i16,
        "B Z" => 9 as i16,
        "C X" => 2 as i16,
        "C Y" => 6 as i16,
        "C Z" => 7 as i16,
        _ => panic!()
    }
}