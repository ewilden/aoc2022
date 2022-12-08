use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    IResult, Parser,
};

#[derive(Clone, Debug)]
pub enum FileEntry {
    Dir(String),
    File(i64, String),
}

#[derive(Clone, Debug)]
pub enum Command {
    Ls,
    Cd(Cd),
}

#[derive(Clone, Debug)]
pub enum Cd {
    Out,
    In(String),
    Root,
}

#[derive(Clone, Debug)]
pub enum InputLine {
    Cmd(Command),
    FileEntry(FileEntry),
}

fn cd(input: &str) -> IResult<&str, Cd> {
    let (input, _) = tag("cd ")(input)?;
    alt((
        tag("..").map(|_| Cd::Out),
        tag("/").map(|_| Cd::Out),
        |s: &str| Ok(("", Cd::In(s.to_owned()))),
    ))(input)
}

fn ls(input: &str) -> IResult<&str, ()> {
    tag("ls").map(|_| ()).parse(input)
}

fn cmd(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ")(input)?;
    alt((ls.map(|()| Command::Ls), cd.map(Command::Cd)))(input)
}

fn dir(input: &str) -> IResult<&str, String> {
    let (name, _) = tag("dir ")(input)?;
    Ok(("", name.to_owned()))
}

fn file(input: &str) -> IResult<&str, (i64, String)> {
    let (input, size) = map_res(digit1, |s: &str| s.parse::<i64>())(input)?;
    let (name, _) = space1(input)?;
    Ok(("", (size, name.to_owned())))
}

fn file_entry(input: &str) -> IResult<&str, FileEntry> {
    alt((
        dir.map(FileEntry::Dir),
        file.map(|(size, name)| FileEntry::File(size, name)),
    ))(input)
}

#[aoc_generator(day7)]
pub fn parse(input: &str) -> Vec<InputLine> {
    input
        .lines()
        .map(|line| {
            if let Ok((_, command)) = cmd(line) {
                // println!("parsed {line} as command {command:?}");
                InputLine::Cmd(command)
            } else if let Ok((_, file_entry)) = file_entry(line) {
                // println!("parsed {line} as file entry");
                InputLine::FileEntry(file_entry)
            } else {
                panic!("couldn't parse {}", line)
            }
        })
        .collect()
}

enum Node<Size = ()> {
    Dir(HashMap<String, Box<Node<Size>>>, Size),
    Leaf(i64),
}

impl Default for Node {
    fn default() -> Self {
        Node::Dir(HashMap::new(), ())
    }
}

impl Node {
    fn compute_sizes(self) -> Node<i64> {
        match self {
            Node::Dir(dir, ()) => {
                let mut size = 0;
                let dir = dir
                    .into_iter()
                    .map(|(name, node)| {
                        let node = node.compute_sizes();
                        size += node.get_size();
                        (name, Box::new(node))
                    })
                    .collect::<HashMap<String, Box<Node<i64>>>>();
                Node::Dir(dir, size)
            }
            Node::Leaf(size) => Node::Leaf(size),
        }
    }
}

impl Node<i64> {
    fn get_size(&self) -> i64 {
        match self {
            Node::Dir(_, size) => *size,
            Node::Leaf(size) => *size,
        }
    }

    fn walk_dirs<'a>(&'a self, at_most: i64) -> Box<dyn Iterator<Item = i64> + 'a> {
        match self {
            Node::Leaf(_) => Box::new(std::iter::empty()),
            Node::Dir(dir, size) => Box::new(
                dir.into_iter()
                    .flat_map(move |(_, node)| node.walk_dirs(at_most).into_iter())
                    .chain((*size <= at_most).then(|| *size)),
            ),
        }
    }
}

fn traverse_tree<'a, 'b>(
    tree: &'a mut Node,
    curr_loc: &'b [String],
    offset: usize,
) -> &'a mut Node {
    if offset == curr_loc.len() {
        return tree;
    }

    let Node::Dir(dir, _) = tree else {
      panic!("expcted tree to be a dir")
    };

    traverse_tree(
        dir.entry(curr_loc[offset].clone())
            .or_insert(Default::default()),
        curr_loc,
        offset + 1,
    )
}

#[aoc(day7, part1)]
fn pt1(input: &Vec<InputLine>) -> i64 {
    let mut tree = Node::default();
    let mut curr_loc: Vec<String> = Vec::new();
    for line in input {
        match line {
            InputLine::Cmd(cmd) => match cmd {
                Command::Ls => (),
                Command::Cd(cd) => match cd {
                    Cd::Out => {
                        let _ = curr_loc.pop();
                    }
                    Cd::In(name) => {
                        curr_loc.push(name.to_owned());
                    }
                    Cd::Root => curr_loc.clear(),
                },
            },
            InputLine::FileEntry(file_entry) => match file_entry {
                FileEntry::Dir(_dirname) => {}
                FileEntry::File(size, name) => {
                    let node = traverse_tree(&mut tree, &curr_loc, 0);
                    let Node::Dir(dir, _) = node else {
                      panic!("expected node to be dir")
                    };
                    dir.insert(name.to_owned(), Box::new(Node::Leaf(*size)));
                }
            },
        }
    }
    let tree = tree.compute_sizes();
    tree.walk_dirs(100000).sum()
}

#[aoc(day7, part2)]
fn pt2(input: &Vec<InputLine>) -> i64 {
    let mut tree = Node::default();
    let mut curr_loc: Vec<String> = Vec::new();
    for line in input {
        match line {
            InputLine::Cmd(cmd) => match cmd {
                Command::Ls => (),
                Command::Cd(cd) => match cd {
                    Cd::Out => {
                        let _ = curr_loc.pop();
                    }
                    Cd::In(name) => {
                        curr_loc.push(name.to_owned());
                    }
                    Cd::Root => curr_loc.clear(),
                },
            },
            InputLine::FileEntry(file_entry) => match file_entry {
                FileEntry::Dir(_dirname) => {}
                FileEntry::File(size, name) => {
                    let node = traverse_tree(&mut tree, &curr_loc, 0);
                    let Node::Dir(dir, _) = node else {
                      panic!("expected node to be dir")
                    };
                    dir.insert(name.to_owned(), Box::new(Node::Leaf(*size)));
                }
            },
        }
    }
    let tree = tree.compute_sizes();
    let need_free_at_least = 30000000 - (70000000 - tree.get_size());
    tree.walk_dirs(70000000)
        .filter(|size| *size >= need_free_at_least)
        .min()
        .unwrap()
}
