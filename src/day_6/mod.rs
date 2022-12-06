use std::collections::{VecDeque, HashSet};

pub fn main() {
    let input = include_str!("input.txt");
    let (char_count, packet_marker) = find_first_unique_n_char_slice(input, 4);
    println!(
        "Start of packet marker: `{}` after {} characters",
        packet_marker,
        char_count
    );

    let (char_count_two, message_marker) = find_first_unique_n_char_slice(input, 14);
    println!(
        "Message marker: `{}` after {} characters",
        message_marker,
        char_count_two
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
    (counter.to_string(), queue.iter().collect::<String>())
}

fn get_number_of_unique_chars_in_queue(queue: &VecDeque<char>) -> usize {
    let mut unique_chars = HashSet::new();
    for c in queue {
        unique_chars.insert(c);
    }
    unique_chars.len()
}
