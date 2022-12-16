use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map_res,
    multi::separated_list0, IResult, Parser,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Packet {
    Num(i32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        match (self, other) {
            (Packet::Num(a), Packet::Num(b)) => a.partial_cmp(b),
            (a @ Packet::Num(_), b @ Packet::List(_)) => {
                Packet::List(vec![a.clone()]).partial_cmp(b)
            }
            (a @ Packet::List(_), b @ Packet::Num(_)) => {
                a.partial_cmp(&Packet::List(vec![b.clone()]))
            }
            (Packet::List(a), Packet::List(b)) => {
                for i in 0..b.len() {
                    if i >= a.len() {
                        return Some(Ordering::Less);
                    }

                    let item_cmp = a[i].partial_cmp(&b[i]).unwrap();
                    if item_cmp != Ordering::Equal {
                        return Some(item_cmp);
                    }
                }
                if a.len() == b.len() {
                    Some(Ordering::Equal)
                } else {
                    Some(Ordering::Greater)
                }
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn packet<'a>(input: &'a str) -> IResult<&'a str, Packet> {
    alt((
        map_res(digit1, |s: &str| s.parse::<i32>().map(Packet::Num)),
        list.map(Packet::List),
    ))(input)
}

fn list<'a>(input: &'a str) -> IResult<&'a str, Vec<Packet>> {
    let (input, _) = tag("[")(input)?;
    let (input, packets) = separated_list0(tag(","), |input: &'a str| packet(input))(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, packets))
}

#[aoc_generator(day13)]
pub fn parse(input: &str) -> Vec<(Packet, Packet)> {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let mut packets = chunk.take(2).map(|line| {
                let (_, packet) = packet(line).unwrap();
                packet
            });
            (packets.next().unwrap(), packets.next().unwrap())
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &[(Packet, Packet)]) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a <= b)
        .map(|(i, _)| i + 1)
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &[(Packet, Packet)]) -> usize {
    let mut packets = input
        .iter()
        .cloned()
        .chain(std::iter::once((
            Packet::List(vec![Packet::List(vec![Packet::Num(2)])]),
            Packet::List(vec![Packet::List(vec![Packet::Num(6)])]),
        )))
        .flat_map(|(a, b)| [a, b])
        .collect::<Vec<_>>();
    packets.sort();
    packets
        .into_iter()
        .enumerate()
        .filter(|(_, packet)| {
            packet == &Packet::List(vec![Packet::List(vec![Packet::Num(2)])])
                || packet == &Packet::List(vec![Packet::List(vec![Packet::Num(6)])])
        })
        .map(|(i, _)| i + 1)
        .product()
}
