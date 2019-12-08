use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::iter::Iterator;

fn get_orbits<'a>(
    orbits: &'a HashMap<&str, &str>,
    subset: &mut Vec<&'a str>,
    initial_key: &str,
) -> usize {
    let mut key = initial_key;
    loop {
        match orbits.get(key) {
            Some(v) => {
                subset.push(v);
                key = v;
            }
            None => break,
        }
    }
    subset.len()
}

fn main() -> Result<(), std::io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;

    let orbits: HashMap<&str, &str> = buffer
        .lines()
        .map(|line| {
            let pair: Vec<&str> = line.split(")").collect();
            (pair[1], pair[0])
        })
        .collect();

    let total: usize = orbits
        .keys()
        .map(|k| {
            let mut subset = vec![];
            get_orbits(&orbits, &mut subset, k)
        })
        .sum();
    println!("total orbits is {}", total);

    let mut you_orbits = vec![];
    get_orbits(&orbits, &mut you_orbits, "YOU");

    let mut san_orbits = vec![];
    get_orbits(&orbits, &mut san_orbits, "SAN");

    println!("YOU orbits = {:?}", you_orbits);
    println!("SAN orbits = {:?}", san_orbits);

    let you_set: HashSet<_> = you_orbits.iter().cloned().map(String::from).collect();
    let san_set: HashSet<_> = san_orbits.iter().cloned().map(String::from).collect();
    let intersections: Vec<_> = you_set.intersection(&san_set).collect();

    let mut steps: Vec<_> = intersections
        .iter()
        .map(|point| {
            let you_steps = you_orbits.iter().position(|i| i == point).unwrap();
            let san_steps = san_orbits.iter().position(|i| i == point).unwrap();
            you_steps + san_steps
        })
        .collect();
    steps.sort();

    println!("minimum orbital transfers: {}", steps[0]);

    Ok(())
}
