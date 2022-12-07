use std::{cell::RefCell, rc::Rc};

use regex::Regex;

#[derive(Eq, PartialEq, Debug, Clone)]
enum Instruction {
    Cd(String),
    Ls,
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Output {
    File { file: String, size: usize },
    Dir(String),
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Size {
    file: String,
    size: usize,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Commmand {
    instruction: Instruction,
    output: Vec<Output>,
}

#[derive(Debug, Clone)]
struct Tree {
    name: String,
    output: Option<Vec<Output>>,
    children: Vec<Rc<RefCell<Tree>>>,
    parent: Option<Rc<RefCell<Tree>>>,
    size: usize,
}

fn part1(tree: &mut Tree) -> (usize, usize) {
    let mut total = 0;
    let mut total_score = 0;
    for child in &tree.children {
        let (child_size, child_score) = part1(&mut child.borrow_mut());
        total += child_size;
        total_score += child_score;
    }
    if let Some(output) = &tree.output {
        for file in output {
            if let Output::File { size, .. } = file {
                total += size;
            }
        }
    }
    if total <= 100000 {
        total_score += total;
    }
    tree.size = total;
    (total, total_score)
}

fn part2(tree: &mut Tree, needed: usize) -> usize {
    let mut min = usize::MAX;
    if tree.size > needed {
        min = tree.size;
    }
    for child in &tree.children {
        let child_min = part2(&mut child.borrow_mut(), needed);
        if child_min < min {
            min = child_min;
        }
    }
    min
}

fn main() {
    let input = include_str!("../../../input/day07.txt");

    let mut commands = Vec::new();

    let mut instruction = None;
    let mut output = Vec::new();

    for line in input.lines().filter(|line| !line.is_empty()) {
        if line.starts_with("$") && instruction.is_some() {
            commands.push(Commmand {
                instruction: instruction.clone().unwrap(),
                output: output.clone(),
            });
            output.clear();
            instruction = None;
        }

        let cd_regex = Regex::new(r"\$ cd (.*)").unwrap();

        if line == "$ ls" {
            instruction = Some(Instruction::Ls);
        } else if let Some(m) = cd_regex.captures_iter(line).next() {
            instruction = Some(Instruction::Cd(m[1].to_string()));
        } else {
            let mut split = line.split_whitespace();
            let size = split.next().unwrap();
            let file = split.next().unwrap().to_owned();
            let size_int: Option<usize> = size.parse().ok();
            if let Some(size_int) = size_int {
                output.push(Output::File {
                    file,
                    size: size_int,
                });
            } else {
                output.push(Output::Dir(file));
            }
        }
    }
    if instruction.is_some() {
        commands.push(Commmand {
            instruction: instruction.clone().unwrap(),
            output: output.clone(),
        });
        output.clear();
    }

    let tree = Rc::new(RefCell::new(Tree {
        name: "/".to_string(),
        output: Some(commands[1].output.clone()),
        children: Vec::new(),
        parent: None,
        size: 0,
    }));

    let mut cwd = Rc::clone(&tree);

    for cmd in commands[1..].iter() {
        match &cmd.instruction {
            Instruction::Ls => {
                cwd.borrow_mut().children.clear();
                for o in cmd.output.iter() {
                    if let Output::Dir(name) = o {
                        let child = Tree {
                            name: name.to_string(),
                            output: None,
                            children: Vec::new(),
                            parent: Some(Rc::clone(&cwd)),
                            size: 0,
                        };
                        cwd.borrow_mut().children.push(Rc::new(RefCell::new(child)));
                    }
                }
                cwd.borrow_mut().output = Some(cmd.output.clone());
            }
            Instruction::Cd(path) => {
                if path == ".." {
                    let next;
                    {
                        let lock = cwd.borrow();
                        next = Rc::clone(lock.parent.as_ref().unwrap());
                    }
                    cwd = next;
                } else {
                    let child;
                    {
                        let mut lock = cwd.borrow_mut();
                        child = Rc::clone(
                            lock.children
                                .iter_mut()
                                .find(|c| c.borrow_mut().name == *path)
                                .unwrap(),
                        );
                    }
                    cwd = child;
                }
            }
        }
    }

    let part1 = part1(&mut tree.borrow_mut());
    dbg!(&part1);
    let available = 70000000 - part1.0;
    let needed = 30000000 - available;

    let part2 = part2(&mut tree.borrow_mut(), needed);
    dbg!(&part2);
}
