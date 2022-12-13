use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Element {
    List(Rc<RefCell<Vec<Element>>>),
    Number(i64),
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(self, other)
    }
}

fn main() {
    let input = include_str!("../../../input/day13.txt");

    let token_regex = Regex::new(r"(\d+|\[|\]|)").unwrap();

    let pairs = input
        .split("\n\n")
        .filter(|l| !l.is_empty())
        .map(|p| {
            p.lines()
                .filter(|l| !l.is_empty())
                .map(|l| {
                    let root = Rc::new(RefCell::new(Vec::new()));
                    let mut vec_stack = Vec::new();
                    let mut active = Rc::clone(&root);
                    for token in token_regex.captures_iter(l) {
                        match &token[0] {
                            "[" => {
                                let new = Rc::default();
                                active.borrow_mut().push(Element::List(Rc::clone(&new)));
                                vec_stack.push(active);
                                active = new;
                            }
                            "]" => active = vec_stack.pop().unwrap(),
                            number => active
                                .borrow_mut()
                                .push(Element::Number(number.parse().unwrap())),
                        }
                    }
                    root
                })
                .collect_vec()
        })
        .collect_vec();

    let part1: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_idx, p)| {
            compare(
                &Element::List(Rc::clone(&p[0])),
                &Element::List(Rc::clone(&p[1])),
            ) != Ordering::Greater
        })
        .map(|(idx, _)| idx + 1)
        .sum();
    dbg!(&part1);

    let mut not_pairs = pairs
        .iter()
        .flatten()
        .map(|l| l.borrow()[0].clone())
        .collect_vec();
    let sep1 = Element::List(Rc::new(RefCell::new(vec![Element::List(Rc::new(
        RefCell::new(vec![Element::Number(2)]),
    ))])));
    let sep2 = Element::List(Rc::new(RefCell::new(vec![Element::List(Rc::new(
        RefCell::new(vec![Element::Number(6)]),
    ))])));
    not_pairs.push(sep1.clone());
    not_pairs.push(sep2.clone());
    not_pairs.sort();

    let idx1 = not_pairs.iter().position(|l| {
        compare(l, &sep1) == Ordering::Equal
    }).unwrap() + 1;
    let idx2 = not_pairs.iter().position(|l| {
        compare(l, &sep2) == Ordering::Equal
    }).unwrap() + 1;

    let part2 = idx1 * idx2;
    dbg!(&part2);
}

fn compare(left: &Element, right: &Element) -> Ordering {
    use itertools::EitherOrBoth::{Both, Left, Right};
    match (left, right) {
        (Element::List(a), Element::List(b)) => {
            for item in a.borrow().iter().zip_longest(b.borrow().iter()) {
                match item {
                    Both(a, b) => match compare(a, b) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => (),
                        Ordering::Greater => return Ordering::Greater,
                    },
                    Left(_) => return Ordering::Greater,
                    Right(_) => return Ordering::Less,
                }
            }
            return Ordering::Equal;
        }
        (a @ Element::List(_), Element::Number(b)) => compare(
            a,
            &Element::List(Rc::new(RefCell::new(vec![Element::Number(*b)]))),
        ),
        (Element::Number(a), b @ Element::List(_)) => compare(
            &Element::List(Rc::new(RefCell::new(vec![Element::Number(*a)]))),
            b,
        ),
        (Element::Number(a), Element::Number(b)) => a.cmp(b),
    }
}
