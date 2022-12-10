#[derive(Clone, Copy)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn deltas(&self) -> Vec<i32> {
        match self {
            Instruction::Noop => vec![0],
            Instruction::Addx(n) => vec![0, *n],
        }
    }
}

#[aoc_generator(day10)]
pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| match line {
            "noop" => Instruction::Noop,
            line => {
                let (_, num) = line.split_once(" ").unwrap();
                let num = num.parse::<i32>().unwrap();
                Instruction::Addx(num)
            }
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &[Instruction]) -> i32 {
    let signal_strengths = input
        .iter()
        .flat_map(Instruction::deltas)
        .scan(1, |register, delta| {
            let value = *register;
            *register += delta;
            Some(value)
        })
        .enumerate()
        .map(|(i, register)| (i + 1, register))
        .map(|(cycle_num, register)| i32::try_from(cycle_num).unwrap() * register)
        .collect::<Vec<_>>();
    [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|cycle_num| signal_strengths[cycle_num - 1])
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &[Instruction]) -> i32 {
    for (cycle_num, position) in input
        .iter()
        .flat_map(Instruction::deltas)
        .scan(1, |register, delta| {
            let value = *register;
            *register += delta;
            Some(value)
        })
        // .collect::<Vec<i32>>();
        .enumerate()
        .map(|(i, register)| (i + 1, register))
    {
        let horizontal_cursor = cycle_num % 40;
        let horizontal_cursor = if horizontal_cursor == 0 {
            40
        } else {
            horizontal_cursor
        };
        let horizontal_cursor = horizontal_cursor - 1;

        if horizontal_cursor == 0 {
            println!();
        }
        if (i32::try_from(horizontal_cursor).unwrap() - position).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }
    }

    // [20, 60, 100, 140, 180, 220]
    //     .into_iter()
    //     .map(|cycle_num| signal_strengths[cycle_num - 1])
    //     .sum()
    // RGLRBZAU
    0
}
