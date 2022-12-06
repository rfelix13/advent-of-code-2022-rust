use std::collections::{VecDeque, HashSet};

pub fn main() {
    println!(
        "{}\n{}",
        find_first_unique_n_char_slice(include_str!("input.txt"), 4).0,
        find_first_unique_n_char_slice(include_str!("input.txt"), 14).0
    );
}

fn find_first_unique_n_char_slice(input: &str, n: u16) -> (String, String) {
    let mut queue = VecDeque::new();
    let mut counter: u16 = n;
    for c in input.chars() {
        if queue.len() < (n-1) as usize {
            queue.push_back(c);
            continue;
        }
        queue.push_back(c);
        if get_number_of_unique_chars_in_queue(&queue) == queue.len() {
            break;
        }
        queue.pop_front();
        counter += 1;
    }
    print!("{:?} ", queue);
    (counter.to_string(), queue.iter().collect::<String>())
}

fn get_number_of_unique_chars_in_queue(queue: &VecDeque<char>) -> usize {
    let mut unique_chars = HashSet::new();
    for c in queue {
        unique_chars.insert(c);
    }
    unique_chars.len()
}
