#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
struct Operation {
    operator: Operator,
    value: u64,
    power: bool,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_by: u64,
    monkey_true: u64,
    monkey_false: u64,
}

impl Operation {
    fn calculate(&self, item: u64) -> u64 {
        let value = if self.power { item } else { self.value };
        match self.operator {
            Operator::Add => value + item,
            Operator::Multiply => value * item,
        }
    }
}
impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: Vec::new(),
            operation: Operation {
                operator: Operator::Add,
                value: 0,
                power: false,
            },
            test_by: 0,
            monkey_true: 0,
            monkey_false: 0,
        }
    }

    fn calculate(&self, item: u64) -> u64 {
        self.operation.calculate(item)
    }

    fn test(&self, result: u64) -> u64 {
        if result % self.test_by == 0 {
            self.monkey_true
        } else {
            self.monkey_false
        }
    }
}

fn main() {
    let input = include_str!("../input.txt").trim();
    let now = std::time::Instant::now();

    let mut monkeys = get_monkeys(input);
    let mut counters = process_monkeys(&mut monkeys);
    counters.sort();
    let counter_len = counters.len();

    let get_product = &counters[counter_len - 2..];

    println!("{:?}", get_product[0] * get_product[1]);
    println!("elapsed: {}ms", now.elapsed().as_millis());
}

fn get_gcf(monkeys: &Vec<Monkey>) -> u64 {
    monkeys.iter().fold(1, |acc, m| m.test_by * acc)
}
fn process_monkeys(monkeys: &mut Vec<Monkey>) -> Vec<u64> {
    let mut counter_item = vec![0; monkeys.len()];
    let gcf = get_gcf(monkeys);

    for _counter in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i].clone();
            counter_item[i] += monkey.items.len() as u64;
            for item in 0..monkey.items.len() {
                let result = monkey.calculate(monkey.items[item]);
                let result_div = result % gcf;
                let pass_to = monkey.test(result_div);
                monkeys[pass_to as usize].items.push(result_div);
            }
            monkey.items.clear();
            monkeys[i] = monkey.clone();
        }
    }

    counter_item
}

fn get_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for line in input.split("\n\n").collect::<Vec<&str>>() {
        let mut monkey = Monkey::new();
        for (i, val) in line.split('\n').enumerate() {
            match i {
                0 => continue,

                1 => {
                    let numbers = val.split(':').map(|s| s.trim()).collect::<Vec<&str>>()[1];
                    let number_list = numbers
                        .split(',')
                        .map(|s| s.trim().parse().unwrap())
                        .collect::<Vec<u64>>();

                    monkey.items = number_list;
                }
                2 => {
                    let operation = val.split("old").map(|s| s.trim()).collect::<Vec<&str>>()[1];
                    let operation_list = operation
                        .split(' ')
                        .map(|s| s.trim())
                        .collect::<Vec<&str>>();

                    let operator = match operation_list[0] {
                        "*" => Operator::Multiply,
                        "+" => Operator::Add,
                        _ => Operator::Add,
                    };
                    let mut is_power = false;
                    let operator_value: u64 = if operation_list.len() == 1 {
                        is_power = true;
                        0
                    } else {
                        operation_list[1].parse().unwrap()
                    };

                    monkey.operation = Operation {
                        operator,
                        value: operator_value,
                        power: is_power,
                    }
                }
                3 => {
                    let test_value = val.split("by").map(|s| s.trim()).collect::<Vec<&str>>()[1];
                    monkey.test_by = test_value.trim().parse().unwrap();
                }
                4 => {
                    let monkey_true =
                        val.split("monkey").map(|s| s.trim()).collect::<Vec<&str>>()[1];
                    monkey.monkey_true = monkey_true.trim().parse().unwrap();
                }
                5 => {
                    let monkey_false =
                        val.split("monkey").map(|s| s.trim()).collect::<Vec<&str>>()[1];
                    monkey.monkey_false = monkey_false.trim().parse().unwrap();
                }
                _ => continue,
            };
        }
        monkeys.push(monkey);
    }

    monkeys
}
