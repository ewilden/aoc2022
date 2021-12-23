struct Input {
  template: String,
  rules: Vec<(String, char)>
}

#[aoc_generator(day14)]
fn parse(inp: &str) -> Input {
  let mut splitted = inp.split("\n\n");
  let template = splitted.next().unwrap().to_string();
  let rules = splitted.next().unwrap().lines().map(|line| {
    let mut splitted = line.split(" -> ");
    let l = splitted.next().unwrap();
    let r = splitted.next().unwrap();
    (l.to_string(), r.chars().next().unwrap())
  }).collect();
  Input { template, rules }
}

use std::collections::HashMap;
use num_bigint::BigInt;
use num_rational::Ratio;
use num::FromPrimitive;
use counter::Counter;

fn do_step(s: &str, rules: &[(String, char)]) -> String {
  let mut out: Vec<char> = Vec::new();
  for (a,b) in s.to_string().chars().zip(s.chars().skip(1)) {
    out.push(a);
    for (pat, c) in rules {
      if pat == &format!("{}{}", a, b) {
        out.push(*c);
      }
    }
  }
  out.push(s.chars().last().unwrap());
  out.into_iter().collect()
}

#[aoc(day14, part1)]
fn part1(inp: &Input) -> i64 {
  let Input {template, rules} = inp;
  let mut s = template.clone();
  for _ in 1..=10 {
    s = do_step(&s, rules);
  }
  let counts: Counter<char> = s.chars().collect();
  let by_freq = counts.most_common();
  println!("{}", s);
  (by_freq[0].1 as i64 - by_freq[by_freq.len() - 1].1 as i64).try_into().unwrap()
}

use cached::proc_macro::cached;
#[cached(
  convert = r#"{ (template.to_string(), steps_left) }"#,
  key = r#"(String, i32)"#,
)]
fn count_chars(rules: &HashMap<String, char>, template: &str, steps_left: i32) -> Counter<char, usize> {
  if template.len() <= 1 {
    panic!()
  } else if template.len() == 2 {
    if steps_left < 0 {
        panic!()
      } else if steps_left == 0 {
        template.chars().collect()
      } else if let Some(to_insert) = rules.get(template) {
        count_chars(rules, &format!("{}{}{}", &template[0..1], to_insert, &template[1..]), steps_left - 1)
      } else {
        template.chars().collect()
      }
  } else {
    let mut counts = Counter::new();
    for i in 0..(template.len() - 1) {
      counts += count_chars(rules, &template[i..i+2], steps_left);
      if i != 0 {
        // we're double counting template[i].
        counts -= (&template[i..i+1]).chars().collect::<Counter<char>>();
      }
    }
    counts
  }
}

#[aoc(day14, part2)]
fn part2(inp: &Input) -> i64 {
  let Input {template, rules} = inp;
  // let mut s = template.clone();
  // for _ in 1..=40 {
  //   s = do_step(&s, rules);
  // }
  let counts: Counter<char> = count_chars(&rules.clone().into_iter().collect(), template, 40);
  let by_freq = counts.most_common();
  // println!("{}", s);
  (by_freq[0].1 as i64 - by_freq[by_freq.len() - 1].1 as i64).try_into().unwrap()
}

// 5637, incorrect