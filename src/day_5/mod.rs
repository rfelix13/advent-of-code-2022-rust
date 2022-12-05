use std::collections::{VecDeque, HashSet};

pub fn main() {
    let input = include_str!("input.txt");
    let (config, instructions) = split_input(input);
    
    // part one
    let mut queue_array: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];
    config.split("\n").for_each(|line| {
        parse_config_line(line, &mut queue_array);
    });
    queue_array.iter().for_each(|queue| {
        println!("{:?}", queue);
    });
    instructions.split("\n").for_each(|line| {
        parse_instruction_line(line, &mut queue_array);
    });
    println!("{}", queue_array.iter().map(get_front).collect::<String>());
    
    // part two
    let mut queue_array_v2: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];
    config.split("\n").for_each(|line| {
        parse_config_line(line, &mut queue_array_v2);
    });
    instructions.split("\n").for_each(|line| {
        parse_instruction_line_v2(line, &mut queue_array_v2);
    });
    println!("{}", queue_array_v2.iter().map(get_front).collect::<String>());
}

//split string by double new line and store into two variables, first is config, second is instructions
fn split_input(input: &str) -> (&str, &str) {
    let mut split_input = input.split("\n\n");
    let config = split_input.next().unwrap();
    let instructions = split_input.next().unwrap();
    (config, instructions)
}

// parse every 4th character out of each line starting with the second character
fn parse_config_line(line: &str, queue_array: &mut Vec<VecDeque<char>>) {
    let mut string_integers = (0..=9).map(|i| i.to_string()).collect::<HashSet<String>>();
    string_integers.insert(" ".to_string());
    for (i, c) in line.chars().enumerate() {
        if i % 4 == 1 && c != ' ' && !string_integers.contains(&c.to_string()){
            // println!("{} {}", i, c);
            queue_array[i/4].push_back(c);
        }
    }
}

fn parse_instruction_line(line: &str, queue_array: &mut Vec<VecDeque<char>>) {
    let mut split_line = line.split(" ");
    let num_boxes = split_line.nth(1).unwrap().parse::<usize>().unwrap();
    let source_queue_int = split_line.nth(1).unwrap().parse::<usize>().unwrap();
    let target_queue_int: usize = split_line.nth(1).unwrap().parse::<usize>().unwrap();
    for _ in 0..num_boxes {
        let source_queue_front = queue_array[(source_queue_int - 1) as usize].pop_front().unwrap_or('ß');
        if source_queue_front != 'ß' {
            queue_array[(target_queue_int - 1) as usize].push_front(source_queue_front);
        }
    }
}

fn parse_instruction_line_v2(line: &str, queue_array: &mut Vec<VecDeque<char>>) {
    let mut split_line = line.split(" ");
    let num_boxes = split_line.nth(1).unwrap().parse::<usize>().unwrap();
    let source_queue_int = split_line.nth(1).unwrap().parse::<usize>().unwrap();
    let target_queue_int: usize = split_line.nth(1).unwrap().parse::<usize>().unwrap();
    let mut temp_queue: VecDeque<char> = VecDeque::new();
    for _ in 0..num_boxes {
        let source_queue_front = queue_array[(source_queue_int - 1) as usize].pop_front().unwrap_or('ß');
        if source_queue_front != 'ß' {
            temp_queue.push_front(source_queue_front);
        }
    }
    temp_queue.iter().for_each(|c| {
        queue_array[(target_queue_int - 1) as usize].push_front(*c);
    });
}

fn get_front(queue: &VecDeque<char>) -> char {
    queue.front().unwrap_or(&' ').clone()
}