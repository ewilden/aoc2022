pub enum Dir {
  Forward,
  Down,
  Up,
}

pub struct Command {
  dir: Dir,
  dist: i32,
}

use anyhow::{Result, anyhow};
use combine::{
    choice,
    error::ParseError,
    parser::{
        char::{char, digit, space, string},
        repeat::many1,
    },
    skip_count,
    stream::position,
    ParseResult,
    Parser, Stream,
};
// use combine::parser::range::{range, take_while1};
// use combine::skip_count;
// use combine::parser::repeat::{sep_by};
// use combine::parser::{
//     choice::choice,
//     char,
//     char::space,
//     char::string,
// };
// use combine::parser::Parser;
// use combine::stream::{RangeStream, state::Stream};
// use combine::error::ParseError;

fn command<I>() -> impl Parser<I, Output = Command>
    where I: Stream<Token = char>,
          I::Error: ParseError<I::Token, I::Range, I::Position> {
    use Dir::*;
    let dir = choice((
        string("forward").map(|_| Forward),
        string("down").map(|_| Down),
        string("up").map(|_| Up),
    ));
    (dir, skip_count(1, space()), many1(digit()).map(|s: String| s.parse::<i32>().unwrap()))
        .map(|(dir, (), dist)| Command {dir, dist})
}

fn parse_line(line: &str) -> Result<Command> {
    command().parse(line).map(|x| x.0).map_err(|e| e.into())
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Command> {
    input.lines().map(|line| {
        parse_line(line).unwrap()
    }).collect()
}

#[aoc(day2, part1)]
pub fn part1(inp: &Vec<Command>) -> i32 {
    let (x, y) = inp.iter().fold((0, 0), |(x,y), Command {dir, dist}| {
        let (dx, dy) = match dir {
            Dir::Forward => (*dist, 0),
            Dir::Down => (0, *dist),
            Dir::Up => (0, -dist),
        };
        (x + dx, y + dy)
    });
    x * y
}

#[aoc(day2, part2)]
pub fn part2(inp: &Vec<Command>) -> i32 {
    let (x, y, _) = inp.iter().fold((0, 0, 0), |(x,y, aim), Command {dir, dist}| {
        let (dx, dy, daim) = match dir {
            Dir::Forward => (*dist, dist * aim, 0),
            Dir::Down => (0, 0, *dist),
            Dir::Up => (0, 0, -dist),
        };
        (x + dx, y + dy, aim + daim)
    });
    x * y
}