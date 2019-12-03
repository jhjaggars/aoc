fn run(state: &mut Vec<usize>) -> usize {
    let mut offset = 0;
    loop {
        let opcode = state[offset];

        if opcode == 99 {
            break;
        }

        let left = state[offset + 1];
        let right = state[offset + 2];
        let output = state[offset + 3];

        if opcode == 1 {
            state[output] = state[left] + state[right];
            offset += 4;
            continue;
        }

        if opcode == 2 {
            state[output] = state[left] * state[right];
            offset += 4;
            continue;
        }
    }
    state[0]
}

fn parse(value: String) -> Vec<usize> {
    value
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn main() {
    let program = parse(String::from("1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,1,19,5,23,2,23,6,27,1,27,5,31,2,6,31,35,1,5,35,39,2,39,9,43,1,43,5,47,1,10,47,51,1,51,6,55,1,55,10,59,1,59,6,63,2,13,63,67,1,9,67,71,2,6,71,75,1,5,75,79,1,9,79,83,2,6,83,87,1,5,87,91,2,6,91,95,2,95,9,99,1,99,6,103,1,103,13,107,2,13,107,111,2,111,10,115,1,115,6,119,1,6,119,123,2,6,123,127,1,127,5,131,2,131,6,135,1,135,2,139,1,139,9,0,99,2,14,0,0"));

    for noun in 0 .. 100 {
        for verb in 0 .. 100 {
            let mut inst = program.clone();
            inst[1] = noun as usize;
            inst[2] = verb as usize;
            let output = run(&mut inst);
            if output == 19690720 {
                println!("found it!");
                println!("final state is {:?}", inst);
                println!("100 * noun + verb = {}", 100 * noun + verb);
                break;
            }
        }
    }
}
