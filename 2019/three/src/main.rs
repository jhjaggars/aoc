use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::iter::FromIterator;

fn trace(input: String) -> Vec<(i32, i32)> {
    let mut current = (0, 0);
    let mut wire = vec![];
    for mv in input.trim().split(",") {
        let mut it = mv.chars();
        let direction = it.next().unwrap();
        let magnitude: i32 = String::from(it.as_str()).parse().unwrap();
        for _ in 1..magnitude + 1 {
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

fn intersections(wire1: Vec<(i32, i32)>, wire2: Vec<(i32, i32)>) -> HashSet<(i32, i32)> {
    let w1: HashSet<(i32, i32)> = HashSet::from_iter(wire1);
    let w2: HashSet<(i32, i32)> = HashSet::from_iter(wire2);
    HashSet::from_iter(w1.intersection(&w2).cloned())
}

fn nearest(wire1: Vec<(i32, i32)>, wire2: Vec<(i32, i32)>) -> i32 {
    intersections(wire1, wire2)
        .iter()
        .map(|p| p.0.abs() + p.1.abs())
        .min()
        .unwrap()
}

fn counter(wire: &Vec<(i32, i32)>) -> HashMap<(i32, i32), usize> {
    let mut m: HashMap<(i32, i32), usize> = HashMap::new();
    for (i, item) in wire.iter().enumerate() {
        m.entry(*item).or_insert(i);
    }
    m
}

fn shortest(wire1: Vec<(i32, i32)>, wire2: Vec<(i32, i32)>) -> usize {
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

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1])?;
    let lines: Vec<&str> = contents.lines().collect();
    let w1 = lines[0].to_string();
    let w2 = lines[1].to_string();

    let wire1 = trace(w1);
    let wire2 = trace(w2);

    let md = nearest(wire1.clone(), wire2.clone());
    println!("manhattan distance = {}", md);
    let sh = shortest(wire1, wire2);
    println!("shortest steps = {}", sh);

    Ok(())
}
