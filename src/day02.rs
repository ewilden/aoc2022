use std::iter::Iterator as _;

#[derive(Clone, Copy)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Win,
    Lose,
    Tie,
}

impl Outcome {
    fn from_me(s: &str) -> Self {
        match s {
            "X" => Lose,
            "Y" => Tie,
            "Z" => Win,
            _ => panic!("from_me: {s}"),
        }
    }
}

use Outcome::*;
use RPS::*;

impl RPS {
    fn from_opponent(s: &str) -> Self {
        match s {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => panic!("from_opponent: {s}"),
        }
    }

    fn from_me(s: &str) -> Self {
        match s {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => panic!("from_me: {s}"),
        }
    }

    fn outcome_vs_opp(self, opp: RPS) -> Outcome {
        match (self, opp) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Win,
            (Scissors, Rock) | (Paper, Scissors) | (Rock, Paper) => Lose,
            _ => Tie,
        }
    }

    fn score_vs_opp(self, opp: RPS) -> i32 {
        (match self.outcome_vs_opp(opp) {
            Lose => 0,
            Tie => 3,
            Win => 6,
        }) + (match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        })
    }

    fn pick_to_get_outcome(self, outcome: Outcome) -> RPS {
        [Rock, Paper, Scissors]
            .into_iter()
            .find(|x| x.outcome_vs_opp(self) == outcome)
            .unwrap()
    }
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<(String, RPS)> {
    input
        .lines()
        .map(|s| {
            let (opp, me) = s.split_once(" ").unwrap();
            (me.to_owned(), RPS::from_opponent(opp))
        })
        .collect()
}

#[aoc(day2, part1)]
fn pt1(inp: &Vec<(String, RPS)>) -> i32 {
    inp.iter()
        .map(|(me, opp)| RPS::from_me(me).score_vs_opp(*opp))
        .sum()
}

#[aoc(day2, part2)]
fn pt2(inp: &Vec<(String, RPS)>) -> i32 {
    inp.iter()
        .map(|(me, opp)| {
            opp.pick_to_get_outcome(Outcome::from_me(me))
                .score_vs_opp(*opp)
        })
        .sum()
}
