#![warn(clippy::all)]
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn trace(input: String) -> Vec<(i32, i32)> {
    let mut current = (0, 0);
    let mut wire = vec![];
    for mv in input.trim().split(',') {
        let mut it = mv.chars();
        let direction = it.next().unwrap();
        let magnitude: i32 = String::from(it.as_str()).parse().unwrap();
        for _ in 1..=magnitude {
            match direction {
                'U' => current.1 += 1,
                'D' => current.1 -= 1,
                'L' => current.0 -= 1,
                'R' => current.0 += 1,
                _ => println!("What child is this? {:?}, {:?}", direction, magnitude),
            }
            wire.push(current);
        }
    }
    wire
}

fn intersections(wire1: &[(i32, i32)], wire2: &[(i32, i32)]) -> HashSet<(i32, i32)> {
    let w1: HashSet<(i32, i32)> = HashSet::from_iter(wire1.to_owned());
    let w2: HashSet<(i32, i32)> = HashSet::from_iter(wire2.to_owned());
    HashSet::from_iter(w1.intersection(&w2).cloned())
}

pub fn nearest(wire1: &[(i32, i32)], wire2: &[(i32, i32)]) -> i32 {
    intersections(wire1, wire2)
        .iter()
        .map(|p| p.0.abs() + p.1.abs())
        .min()
        .unwrap()
}

pub fn counter(wire: &[(i32, i32)]) -> HashMap<(i32, i32), usize> {
    let mut m: HashMap<(i32, i32), usize> = HashMap::new();
    for (i, item) in wire.iter().enumerate() {
        m.entry(*item).or_insert(i);
    }
    m
}

pub fn shortest(wire1: &[(i32, i32)], wire2: &[(i32, i32)]) -> usize {
    println!("wire1 len {}", wire1.len());
    println!("wire2 len {}", wire2.len());
    let wc1 = counter(&wire1);
    let wc2 = counter(&wire2);
    let ints = intersections(wire1, wire2);
    let mut cheapest = 0;
    for p in ints {
        let left = wc1.get(&p).unwrap();
        let right = wc2.get(&p).unwrap();
        let cost = left + right + 2; // add two b/c of zero-index
        if cheapest == 0 || cost < cheapest {
            cheapest = cost;
        }
    }
    cheapest
}
