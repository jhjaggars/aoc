use std::io;

fn calc(incoming: f64) -> f64 {
    let mass = (incoming / 3.0).floor() - 2.0;
    if mass > 0.0 {
        mass + calc(mass)
    } else {
        0.0
    }
}

fn main() {
    let mut mass_vec: Vec<f64> = Vec::new();

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("failed to read line");

        let mass: f64 = match s.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        mass_vec.push(mass);
    }

    let total: f64 = mass_vec.iter().map(|&v| calc(v)).sum();

    println!("Total fuel needed is {}", total);
}
