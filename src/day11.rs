use itertools::Itertools;

pub struct Monkey {
    pub items: Vec<i64>,
    pub operation: Box<dyn Fn(i64) -> i64>,
    pub test_divisor: i64,
    pub iftrue: usize,
    pub iffalse: usize,
    pub inspection_count: i64,
}

#[aoc_generator(day11)]
pub fn parse(input: &str) -> String {
    input.to_owned()
}

const PRINT: bool = false;

fn real_parse(input: &str) -> Vec<Monkey> {
    input
        .lines()
        .chunks(7)
        .into_iter()
        .map(|mut iter| {
            let _ = iter.next().unwrap();

            let (_, items) = iter.next().unwrap().split_once(": ").unwrap();
            let items = items
                .split(", ")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            if PRINT {
                println!("items: {items:?}");
            }

            let (_, operation) = iter.next().unwrap().split_once("new = old ").unwrap();
            let operand = operation[2..].parse::<i64>().ok();
            let operation = operation[0..1].to_owned();

            if PRINT {
                println!("operand: {operand:?}, operation: {operation:?}");
            }
            let operation = Box::new(move |x: i64| match operation.as_str() {
                "+" => x + operand.unwrap_or(x),
                "*" => x * operand.unwrap_or(x),
                s => panic!("unrecognized operation {s}"),
            });

            let (_, test_divisor) = iter.next().unwrap().split_once("divisible by ").unwrap();
            let test_divisor = test_divisor.parse::<i64>().unwrap();
            if PRINT {
                println!("test_divisor: {test_divisor:?}");
            }

            let (_, iftrue) = iter
                .next()
                .unwrap()
                .split_once("true: throw to monkey ")
                .unwrap();
            let iftrue = iftrue.parse::<usize>().unwrap();
            if PRINT {
                println!("iftrue: {iftrue:?}");
            }

            let (_, iffalse) = iter
                .next()
                .unwrap()
                .split_once("false: throw to monkey ")
                .unwrap();
            let iffalse = iffalse.parse::<usize>().unwrap();
            if PRINT {
                println!("iffalse: {iffalse:?}");
            }

            Monkey {
                items,
                operation,
                test_divisor,
                iftrue,
                iffalse,
                inspection_count: 0,
            }
        })
        .collect()
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> i64 {
    let mut monkeys = real_parse(input);

    for _ in 1..=20 {
        for i in 0..monkeys.len() {
            let items = std::mem::replace(&mut monkeys[i].items, Vec::new());
            for worry_level in items {
                monkeys[i].inspection_count += 1;
                let worry_level = (monkeys[i].operation)(worry_level);
                let worry_level = worry_level / 3;
                let test_result = worry_level % monkeys[i].test_divisor == 0;
                let next_monkey_index = if test_result {
                    monkeys[i].iftrue
                } else {
                    monkeys[i].iffalse
                };
                monkeys[next_monkey_index].items.push(worry_level);
            }
        }
    }

    monkeys
        .into_iter()
        .map(|monkey| -monkey.inspection_count)
        .k_smallest(2)
        .product()
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> i64 {
    let mut monkeys = real_parse(input);
    let test_divisor_product: i64 = monkeys.iter().map(|monkey| monkey.test_divisor).product();

    for _ in 1..=10000 {
        for i in 0..monkeys.len() {
            let items = std::mem::replace(&mut monkeys[i].items, Vec::new());
            for worry_level in items {
                monkeys[i].inspection_count += 1;
                let worry_level = (monkeys[i].operation)(worry_level) % test_divisor_product;
                let test_result = worry_level % monkeys[i].test_divisor == 0;
                let next_monkey_index = if test_result {
                    monkeys[i].iftrue
                } else {
                    monkeys[i].iffalse
                };
                monkeys[next_monkey_index].items.push(worry_level);
            }
        }
    }

    monkeys
        .into_iter()
        .map(|monkey| -monkey.inspection_count)
        .k_smallest(2)
        .product()
}
