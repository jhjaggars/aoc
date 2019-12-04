use std::fs;
use std::env;
use three;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1])?;
    let lines: Vec<&str> = contents.lines().collect();
    let w1 = lines[0].to_string();
    let w2 = lines[1].to_string();

    let wire1 = three::trace(w1);
    let wire2 = three::trace(w2);

    let md = three::nearest(&wire1, &wire2);
    println!("manhattan distance = {}", md);
    let sh = three::shortest(&wire1, &wire2);
    println!("shortest steps = {}", sh);

    Ok(())
}
