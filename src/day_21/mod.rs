use core::panic;
use std::collections::HashMap;
use std::fs;


#[derive(Clone, PartialEq, Eq)]
enum OperationEnum {
    Multiply,
    Add,
    Subtract,
    Divide,
}
impl OperationEnum {
    fn from_str(operation: &str) -> OperationEnum {
        match operation {
            "*" => OperationEnum::Multiply,
            "+" => OperationEnum::Add,
            "-" => OperationEnum::Subtract,
            "/" => OperationEnum::Divide,
            _ => panic!("Unknown operation"),
        }
    }
    fn inverse(&self) -> OperationEnum {
        match &self {
            OperationEnum::Multiply => OperationEnum::Divide,
            OperationEnum::Add => OperationEnum::Subtract,
            OperationEnum::Subtract => OperationEnum::Add,
            OperationEnum::Divide => OperationEnum::Multiply,
        }
    }
    fn compute(&self, number_1: i128, number_2: i128) -> i128 {
        match &self {
            OperationEnum::Multiply => number_1 * number_2,
            OperationEnum::Add => number_1 + number_2,
            OperationEnum::Subtract => number_1 - number_2,
            OperationEnum::Divide => number_1 / number_2,
        }
    }
}


#[derive(Clone)]
struct Dependency {
    monkey_1_name: String,
    monkey_2_name: String,
    operation: OperationEnum
}
impl Dependency {
    fn get_monkeys(&self, monkey_collection: &HashMap<String, Monkey>) -> (Monkey, Monkey) {
        let monkey_1 = monkey_collection[self.monkey_1_name.as_str()].clone();
        let monkey_2 = monkey_collection[self.monkey_2_name.as_str()].clone();
        return (monkey_1, monkey_2);
    }
}


#[derive(Clone)]
struct Monkey {
    name: String,
    number: Option<i128>,
    dependency: Option<Dependency>,
}
impl Monkey {
    fn parse_line(line: &str) -> Monkey {
        let line_split = line.split(": ").collect::<Vec<&str>>();
        let name: String = line_split[0].to_string();
        let second_split = line_split[1].split(" ").collect::<Vec<&str>>();
        if second_split.len() == 1 {
            let number = line_split[1].parse::<i128>().unwrap();
            return Monkey {name: name, number: Some(number), dependency: None};
        }
        let dependency = Dependency{
            monkey_1_name: second_split[0].to_string(),
            monkey_2_name: second_split[2].to_string(),
            operation: OperationEnum::from_str(&second_split[1]),
        };
        return Monkey {name: name, number: None, dependency: Some(dependency)};
    }
    fn get_number(&self, monkey_collection: &mut HashMap<String, Monkey>) -> i128 {
        let monkey_number = &self.number;
        if monkey_number.is_some() {
            return monkey_number.unwrap();
        }
        if self.dependency.is_none() {
            panic!("No number or dependency");
        }
        let dependency = &self.dependency.as_ref().unwrap();
        let (monkey_1, monkey_2) = dependency.get_monkeys(monkey_collection);
        let monkey_1_number = monkey_1.get_number(monkey_collection);
        let monkey_2_number = monkey_2.get_number(monkey_collection);
        let monkey_calculated_number = match dependency.operation {
            OperationEnum::Multiply => monkey_1_number * monkey_2_number,
            OperationEnum::Add => monkey_1_number + monkey_2_number,
            OperationEnum::Subtract => monkey_1_number - monkey_2_number,
            OperationEnum::Divide => monkey_1_number / monkey_2_number,
        };
        monkey_collection.insert(
            self.name.to_string(),
            Monkey {
                name: self.name.clone(),
                number: Some(monkey_calculated_number),
                dependency: None
            }
        );
        return monkey_calculated_number;
    }
    fn has_human_dependency(&self, monkey_collection: &HashMap<String, Monkey>) -> bool {
        if self.name == "humn" {
            return true;
        }
        if self.dependency.is_none() {
            return false;
        }
        let dependency = &self.dependency.as_ref().unwrap();
        let (m1, m2) = dependency.get_monkeys(monkey_collection);
        if m1.has_human_dependency(monkey_collection) || m2.has_human_dependency(monkey_collection) {
            return true
        }
        return false;
    }
    fn solve_for_value(&self, target_value: i128, monkey_collection: &mut HashMap<String, Monkey>) {
        // base case
        if &self.name == "humn" {
            monkey_collection.insert(
                self.name.to_string(),
                Monkey {
                    name: self.name.clone(),
                    number: Some(target_value),
                    dependency: None
                }
            );
            return;
        }

        // recursive case
        if self.dependency.is_none() {
            panic!("Monkey has no dependency");
        }
        let dependency = &self.dependency.as_ref().unwrap();
        let (h_monkey, h_monkey_num) = {
            let (m1, m2) = dependency.get_monkeys(monkey_collection);
            if m1.has_human_dependency(monkey_collection) {
                let o_number = m2.get_number(monkey_collection);
                (m1, dependency.operation.inverse().compute(target_value, o_number))
            } else if m2.has_human_dependency(monkey_collection) {
                let o_number = m1.get_number(monkey_collection);
                let mut n1 = target_value;
                let mut n2 = o_number;
                let op: OperationEnum = {
                    match dependency.operation {
                        OperationEnum::Add => OperationEnum::Subtract,
                        OperationEnum::Multiply => OperationEnum::Divide,
                        OperationEnum::Subtract => {n1=n2; n2=target_value; OperationEnum::Subtract},
                        OperationEnum::Divide => {n1=n2; n2=target_value; OperationEnum::Divide},
                    }
                };
                (m2, op.compute(n1, n2))
            } else {
                panic!("Neither monkey is human dependent");
            }
        };
        h_monkey.solve_for_value(h_monkey_num, monkey_collection);
    }
}

fn parse_lines(file_name: &String, monkey_collection: &mut HashMap<String, Monkey>) {
    let file_string = fs::read_to_string(file_name).unwrap();
    let lines = file_string.split("\n").collect::<Vec<&str>>();
    for line in lines {
        let monkey = Monkey::parse_line(line);
        monkey_collection.insert(monkey.name.to_string(), monkey);
    };
}


pub fn main(file_name: &String) {
    let mut monkey_collection:HashMap<String, Monkey> = HashMap::new();
    parse_lines(file_name, &mut monkey_collection);
    let root_monkey = monkey_collection["root"].clone();
    println!(
        "root: {}",
        root_monkey.get_number(&mut monkey_collection)
    );

    let mut monkey_collection_v2:HashMap<String, Monkey> = HashMap::new();
    parse_lines(file_name, &mut monkey_collection_v2);
    let root_monkey_v2 = monkey_collection_v2["root"].clone();
    let (h_dep, o_dep) = {
        let dep = root_monkey_v2.dependency.as_ref().unwrap().clone();
        let (m1, m2) = dep.get_monkeys(&mut monkey_collection_v2);
        if m1.has_human_dependency(&monkey_collection_v2) {
            (m1, m2)
        } else {
            (m2, m1)
        }
    };
    let new_number = o_dep.get_number(&mut monkey_collection_v2);
    h_dep.solve_for_value(new_number, &mut monkey_collection_v2);
    let human = monkey_collection_v2["humn"].clone();
    println!(
        "human: {}",
        human.get_number(&mut monkey_collection_v2),
    )
}
