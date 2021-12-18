use bitvec::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools as _;

type bitindex = usize;

fn digit_to_bitmask(digit: i32) -> BitVec {
    match digit {
        0 => bitvec![1, 1, 1, 0, 1, 1, 1],
        1 => bitvec![0, 0, 1, 0, 0, 1, 0],
        2 => bitvec![1, 0, 1, 1, 1, 0, 1],
        3 => bitvec![1, 0, 1, 1, 0, 1, 1],
        4 => bitvec![0, 1, 1, 1, 0, 1, 0],
        5 => bitvec![1, 1, 0, 1, 0, 1, 1],
        6 => bitvec![1, 1, 0, 1, 1, 1, 1],
        7 => bitvec![1, 0, 1, 0, 0, 1, 0],
        8 => bitvec![1, 1, 1, 1, 1, 1, 1],
        9 => bitvec![1, 1, 1, 1, 0, 1, 1],
        _ => panic!(),
    }
}

fn zeroes() -> BitVec { bitvec![0, 0, 0, 0, 0, 0, 0] }
fn ones() -> BitVec { bitvec![1, 1, 1, 1, 1, 1, 1] }
fn singleton(ind: usize) -> BitVec {
    let mut z = zeroes();
    z.set(ind, true);
    z
}

fn bitmask_to_digit(bm: &BitVec) -> Option<i32> {
    for d in 0..=9 {
        if digit_to_bitmask(d) == *bm {
            return Some(d as i32)
        }
        if digit_to_bitmask(d).len() != bm.len() {
            panic!()
        }
    }
    None
}

fn num_segments(digit: i32) -> usize {
    digit_to_bitmask(digit)
        .as_bitslice()
        .count_ones()
        .try_into()
        .unwrap()
}

fn unique_segment_count(seg_count: usize) -> Option<i32> {
    match seg_count {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        0 | 1 | 5 | 6 => None,
        _ => panic!(),
    }
}

#[derive(Clone)]
struct InputLine {
    signals: Vec<Vec<char>>,
    output: (Vec<char>, Vec<char>, Vec<char>, Vec<char>),
}

impl InputLine {
    fn all_entries(&self) -> Vec<Vec<char>> {
        self.signals.clone()
            .into_iter()
            .chain(from_quad(self.output.clone()))
            .collect()
    }
}

fn from_quad<T>(quad: (Vec<T>, Vec<T>, Vec<T>, Vec<T>)) -> Vec<Vec<T>> {
    let (a, b, c, d) = quad;
    vec![a, b, c, d]
}

#[aoc_generator(day8)]
fn parse(inp: &str) -> Vec<InputLine> {
    inp.lines()
        .map(|line| {
            let splitted = line.split(" | ").collect::<Vec<_>>();
            let raw_fst = splitted[0];
            let raw_snd = splitted[1];
            let v: Vec<Vec<char>> = raw_snd.split(" ").map(|s| s.chars().collect()).collect();
            InputLine {
                signals: raw_fst.split(" ").map(|s| s.chars().collect()).collect(),
                output: (v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()),
            }
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(inp: &Vec<InputLine>) -> usize {
    inp.iter()
        .map(|input| {
            let uniq_digit_count: usize = from_quad(input.output.clone())
                .iter()
                .map(|v| v.len())
                .map(|n| {
                    let may_uniq_digit: Option<i32> = unique_segment_count(n);
                    may_uniq_digit.iter().len()
                })
                .sum();
            uniq_digit_count
        })
        .sum()
}

fn assign_chars(input: &InputLine) -> HashMap<char, BitVec> {
    let mut char_to_bitmask: HashMap<char, BitVec> = ('a'..='g').map(|c| (c, ones())).collect();
    let mut locked_assignments: HashMap<char, BitVec> = HashMap::new();
    let mut times_since_moved_prev: usize = 0;
    loop {
        println!("{:?}", char_to_bitmask);
        // move over assignments
        let new_assignments: Vec<_> = char_to_bitmask.clone().into_iter().filter(|(_,bm)| bm.count_ones() == 1).collect();
        if new_assignments.len() == 0 {
            if times_since_moved_prev > 10 {
                println!("{:?}", locked_assignments);
                panic!("I give up.")
            }
            times_since_moved_prev += 1;
        } else {
            println!("some success at least");
            times_since_moved_prev = 0;
        }
        locked_assignments.extend(new_assignments);

        if locked_assignments.len() == 7 {
            return locked_assignments;
        }

        let union_of_locked_assignments = locked_assignments.values().cloned().fold(zeroes(), |a,b| a | b);

        let mut poss_digits = input.signals.clone().into_iter().map(|signal| {
            let num_segs = signal.len();
            // A superset of the possible digits this signal represents can be found
            // simply by counting the number of segmetns in the signal.
            let (possible, impossible): (Vec<i32>, Vec<i32>) = 
                DIGITS.into_iter().partition_map(|d| 
                    if num_segments(d) == num_segs {
                        itertools::Either::Left(d)
                    } else {
                        itertools::Either::Right(d)
                    });
            // But we can do better by filtering out any digits with a segment that
            // we know for sure doesn't match any of the characters in the signal.
            let possible: Vec<i32> = possible.into_iter()
                            .filter(|dig| {
                                let union_of_charmasks = signal.iter().flat_map(|c| char_to_bitmask.get(c).into_iter().chain(locked_assignments.get(c)))
                                    .cloned()                                    
                                    .fold(zeroes(), |a,b| a | b);
                                // need this to be superset of possible's mask
                                let poss_mask = digit_to_bitmask(*dig);
                                for (cbit, possbit) in union_of_charmasks.into_iter().zip(poss_mask) {
                                    if !cbit && possbit {
                                        return false
                                    }
                                }
                                true
                            }).collect();
            possible
        }).collect::<Vec<_>>();
        for _ in 1..=10 {
            let possdig_to_signal_indices: multimap::MultiMap<i32, usize> = poss_digits.iter().cloned().enumerate().flat_map(
                    |(ind, possdigs)| possdigs.into_iter().map(move |dig| (dig, ind))
            ).collect();
            let uniq_digs: HashMap<usize, i32> = possdig_to_signal_indices.into_iter().filter_map(|(dig, inds)| {
                if inds.len() == 1 {
                    Some((inds[0], dig))
                } else {
                    None
                }
            }).collect();
            for (i, dig_vec) in poss_digits.iter_mut().enumerate() {
                if let Some(dig) = uniq_digs.get(&i) {
                    // println!("tripped {:?} {} at {}", *dig_vec, dig, i);
                    *dig_vec = vec![*dig]
                } else {
                    *dig_vec = (*dig_vec).clone().into_iter().filter(|x| !uniq_digs.values().contains(&x)).collect();
                }
            }
        }

        for (signal, possible) in input.signals.iter().zip(poss_digits.iter()) {
            let union_of_possible = possible.iter().cloned().map(digit_to_bitmask).fold(zeroes(), |a,b| a | b);
            // let union_of_impossible = impossible.iter().cloned().map(digit_to_bitmask).fold(zeroes(), |a,b| a | b);
            for c in signal {
                let bitmask = char_to_bitmask.entry(*c).or_insert_with(ones);
                *bitmask &= union_of_possible.clone();
                *bitmask &= !union_of_locked_assignments.clone();
            }
        }
        // Next step: what if we know for some signal that it is the only candidate for a digit?
        // We already encode the reverse (that a signal might have only one candidate digit)

        let mut clear_except: Vec<([char; 2], BitVec)> = Vec::new();

        // search for 2-sets. for any 2-set, none of the other characters
        // can map to either segment in the 2-set
        for c in 'a'..='g' {
            for c_ in 'a'..='g' {
                if c == c_ {
                    continue
                }
                if char_to_bitmask[&c].count_ones() != 2 {
                    continue
                }
                if char_to_bitmask[&c] == char_to_bitmask[&c_] {
                    // panic!("Found equal");
                    clear_except.push(([c, c_], char_to_bitmask[&c].clone()))
                }
            }
        }
        for (excepts, clearmask) in clear_except {
            for (chr, bm) in char_to_bitmask.iter_mut() {
                if !excepts.contains(chr) {
                    *bm &= !clearmask.clone();
                }
            }
        }
    }
}




struct Assigns(HashMap<char, bitindex>);

impl Assigns {
    fn decode(&self, chrs: Vec<char>) -> i32 {
        bitmask_to_digit(&chrs.iter()
            .map(|c| {
                let bit_index: bitindex = *self.0.get(c).unwrap();
                let mut mask = zeroes();
                mask.set(bit_index, true);
                mask
            }).fold(zeroes(), |a, b| a.clone() | b.clone())).unwrap()
    }
    fn decode_quad(&self, (a, b, c, d): (Vec<char>, Vec<char>, Vec<char>, Vec<char>)) -> i32 {
        self.decode(a) * 1000
        + self.decode(b) * 100
        + self.decode(c) * 10
        + self.decode(d)
    }
}

#[aoc(day8, part2)]
fn part2(inp: &Vec<InputLine>) -> String {
    format!("{:?}", assign_chars(&inp[0]))
}

fn reverse_mapping(char_to_mask: &HashMap<char, BitVec>) -> HashMap<bitindex, HashSet<char>> {
    let mut mapping = HashMap::new();
    for (chr, bv) in char_to_mask.iter() {
        for (ind, val) in bv.iter().enumerate() {
            if *val == true {
                let hs = mapping.entry(ind).or_insert(HashSet::new());
                hs.insert(*chr);
            }
        }
    }
    mapping
}

fn deduce(input_line: InputLine) -> Assigns {
    let mut char_to_mask: HashMap<char, BitVec> = HashMap::new();
    for charvec in input_line.all_entries().into_iter() {
        let relevant_bitmasks: Vec<BitVec> = (0..=9).map(digit_to_bitmask).filter(|bm| bm.as_bitslice().count_ones() == charvec.len()).collect();
        let relevant_union: BitVec = relevant_bitmasks.into_iter().fold(zeroes(), |x, y| (x | y));
        for c in charvec.iter() {
            let entry = char_to_mask.entry(*c).or_insert(ones());
            *entry = entry.clone() & relevant_union.clone();
        }
    }
    println!("{:#?}", char_to_mask);

    let mut assignments: HashMap<char, bitindex> = HashMap::new();
    loop {
        if assignments.len() == 9 {
            break
        }
        let new_assignments_r: Vec<(char, BitVec)> = char_to_mask.clone().into_iter().filter(|(k,v)| {
            v.count_ones() == 1
        }).collect();
        let new_assignments_l: Vec<(char, BitVec)> = reverse_mapping(&char_to_mask).into_iter().filter(|(ind, chrset)| {
            chrset.len() == 1
        }).map(|(ind, chrset)| (chrset.into_iter().next().unwrap(), singleton(ind))).collect();
        let new_assignments = [&new_assignments_r[..], &new_assignments_l[..]].concat();
        if new_assignments.len() == 0 {
            panic!("{:?}\n{:?}", assignments, char_to_mask)
        }
        for (k, v) in new_assignments_r.into_iter() {
            for (ind, val) in v.iter().enumerate() {
                if *val == true {
                    assignments.insert(k, ind);
                    char_to_mask.remove(&k);
                    for (chr, msk) in char_to_mask.iter_mut() {
                        msk.set(ind, false);
                    }
                }
            }
        }
    }
    
    Assigns(assignments)
}

// #[aoc(day8, part2)]
// fn part2(inp: &Vec<InputLine>) -> i32 {
//     inp.clone().into_iter().map(|line| {
//         let assigns = deduce(line.clone());
//         assigns.decode_quad(line.output)
//     }).sum()
// }

// use good_lp::{constraint, default_solver, Solution, SolverModel, variables};

// struct SegmentVars {
//     a: good_lp::Variable,
//     b: good_lp::Variable,
//     c: good_lp::Variable,
//     d: good_lp::Variable,
//     e: good_lp::Variable,
//     f: good_lp::Variable,
//     g: good_lp::Variable,
// }

const CHARS: std::ops::RangeInclusive<char> = 'a'..='g';
const CLOCK_SEGMENTS: std::ops::RangeInclusive<usize> = 0..=6;
const DIGITS: std::ops::RangeInclusive<i32> = 0..=9;
const GROUPING_INDICES: std::ops::RangeInclusive<usize> = 0..=13;

// fn solve_line(InputLine { signals, output }: InputLine) -> i32 {
//     use good_lp::{Expression, Variable, constraint, variable};
//     let mut vars: good_lp::ProblemVariables = variables!();
//         // let char_vars: HashMap<char, good_lp::Variable> = 
//     //     ('a'..='g').zip(std::iter::repeat_with(|| vars.add(variable().clamp(1, 1)))).collect();
//     // let dig_vars: Vec<good_lp::Variable> = DIGITS.map(|_| vars.add(variable().clamp(1, 1))).collect();
//     // let char_to_dig_vars: HashMap<(char, usize), Variable> = {
//     //     let mut hm = HashMap::new();
//     //     for c in CHARS {
//     //         for d in CLOCK_SEGMENTS {
//     //             hm.insert((c, d), vars.add(variable().binary()));
//     //         }
//     //     }
//     //     hm
//     // };
//     let mut constraints: Vec<good_lp::Constraint> = Vec::new();

//     for grouping in signals.iter().chain(from_quad(output).iter()) {
//         let relevant_digits: Vec<i32> = 
//             DIGITS.filter(|dig| digit_to_bitmask(*dig).as_bitslice().count_ones() == grouping.len()).collect();
//         // this grouping is exactly one of these digits.
//         let grouping_to_dig_vars: Vec<Variable> = DIGITS.map(|_| vars.add(variable().binary())).collect();
//         constraints.push(grouping_to_dig_vars
//             .iter()
//             .map(Expression::from_other_affine)
//             .reduce(|a,b| a + b)
//             .unwrap()
//             .eq(1));
//         constraints.push(grouping_to_dig_vars
//             .iter()
//             .enumerate()
//             .filter(|(i,_)| relevant_digits.contains(&((*i) as i32)))
//             .map(|(_,b)| b)
//             .map(Expression::from_other_affine)
//             .reduce(|a,b| a + b)
//             .unwrap()
//             .eq(1));

//         // constraints.push(relevant_digits.iter().map(|d| char_to_dig_vars[&(c,*d)])
//         //         .map(Expression::from_other_affine)
//         //         .reduce(|a,b| a + b)
//         //         .unwrap()
//         //         .eq(1));
//         // for c in 'a'..='g' {
//         //     // Fixing each character, 
//         //     constraints.push(relevant_segments.iter().map(|d| char_to_dig_vars[&(c,*d)])
//         //         .map(Expression::from_other_affine)
//         //         .reduce(|a,b| a + b)
//         //         .unwrap()
//         //         .eq(1));
//         // }
//         // for d in relevant_segments {
//         //     constraints.push(('a'..='g').map(|c| char_to_dig_vars[&(c,d)])
//         //         .map(Expression::from_other_affine)
//         //         .reduce(|a,b| a + b)
//         //         .unwrap()
//         //         .eq(1));
//         // }
//         // for dig in relevant_segments {
//         //     constraints.push(sum_of_char_vars.clone().eq(Expression::from_other_affine(dig_vars[dig])));
//         // }
//     }

//     // let solution = constraints.into_iter().fold(vars.maximise(0).using(good_lp::default_solver), |p,c| p.with(c))
//     //         .solve().unwrap();

//     // for char_ in 'a'..='g' {
//     //     println!("{}: {}", char_, solution.value(char_vars[&char_]));
//     // }

//     // for dig in 0..=9 {
//     //     println!("{}: {}", dig, solution.value(dig_vars[dig]));
//     // }

//     // solution.eval(char_vars[output.0] * 1000 + char_vars[output.1] * 100 + char_vars[output.2] * 10 + char_vars[output.3]).try_into().unwrap()
//     todo!()
// }

// // #[aoc(day8, part2, milp)]
// // fn part2_milp(inp: &Vec<InputLine>) -> i32 {
    

// // }