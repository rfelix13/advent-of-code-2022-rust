
pub fn main() {
    let input = include_str!("input.txt");
    let mut cycles: u32 = 0;
    let mut register: i32 = 1;
    let mut register_sum: i32 = 0;
    let mut char_arrays: Vec<Vec<char>> = vec![vec![]; 6];

    for line in input.lines().into_iter() {
        let (cycles_run, value) = parse_new_line(line, &register);
        for _ in 0..cycles_run {
            char_arrays[(cycles/40) as usize].push({
                if ((cycles % 40) as i32 - register).abs() < 2 {'#'} else {'.'}
            });
            cycles += 1;
            if cycles % 40 == 20 {
                let signal = (cycles as i32) * register;
                println!("Cycle: {}, Register: {}, Signal: {}", cycles, register, signal);
                register_sum += signal;
            }
        }
        register = value;
    }
    println!("Sum: {:?}", register_sum);
    println!("{}", char_arrays.into_iter().map(|char_array| {
        char_array.into_iter().collect::<String>()
    }).collect::<Vec<String>>().join("\n"));
}

fn parse_new_line(line: &str, old_register: &i32) -> (u8, i32) {
    let splits = line.split(" ").collect::<Vec<&str>>();
    let cycles = get_number_of_cycles_from_command(splits[0]);
    let mut register = old_register.clone();
    if splits.len() > 1 {
        register += splits[1].parse::<i32>().unwrap();
    }
    return (cycles, register)
}

fn get_number_of_cycles_from_command(command: &str) -> u8 {
    match command {
        "noop" => 1,
        "addx" => 2,
        _ => panic!("Unknown command: {}", command),
    }
}