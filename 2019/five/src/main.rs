use std::fs;

#[derive(Debug)]
struct Op {
    code: usize,
    mode1: usize,
    mode2: usize,
    mode3: usize,
}

impl Op {
    fn param(&self, state: &Vec<i64>, offset: usize, param: usize) -> i64 {
        let actual = state[offset + param];
        let mode = match param {
            1 => self.mode1,
            2 => self.mode2,
            3 => self.mode3,
            _ => unreachable!("oh jesus!"),
        };
        if mode == 1 {
            actual
        } else {
            state[actual as usize]
        }
    }
}

fn get_opcode(input: i64) -> Op {
    Op {
        code: (input % 100) as usize,
        mode1: ((input / 100) % 10) as usize,
        mode2: ((input / 1000) % 10) as usize,
        mode3: ((input / 10000) % 10) as usize,
    }
}

fn run(state: &mut Vec<i64>, input: i64) -> i64 {
    let mut offset = 0;
    let mut output = 0;
    loop {
        let op = get_opcode(state[offset]);
        // println!("offset is {}", offset);
        // println!("\tstate is {:?}", state);
        // println!("\top is {:?}", op);

        if op.code == 99 {
            break;
        }

        match op.code {
            1 => {
                let left = op.param(state, offset, 1);
                let right = op.param(state, offset, 2);
                let dest = state[offset + 3] as usize;
                state[dest] = left + right;
                offset += 4;
            }
            2 => {
                let left = op.param(state, offset, 1);
                let right = op.param(state, offset, 2);
                let dest = state[offset + 3] as usize;
                state[dest] = left * right;
                offset += 4;
            }
            3 => {
                let dest = state[offset + 1] as usize;
                // println!("storing {} into {}", input, dest);
                state[dest] = input;
                offset += 2;
            }
            4 => {
                output = op.param(state, offset, 1);
                offset += 2;
            }
            5 => {
                let left = op.param(state, offset, 1);
                let right = op.param(state, offset, 2);
                // println!("\t {} != 0", left);
                if left != 0 {
                    // println!("\t * {} neq 0, jumping to {}", left, right);
                    offset = right as usize;
                } else {
                    offset += 3;
                }
            }
            6 => {
                let left = op.param(state, offset, 1);
                let right = op.param(state, offset, 2);
                if left == 0 {
                    // println!("\t * {} eq 0, jumping to {}", left, right);
                    offset = right as usize;
                } else {
                    offset += 3;
                }
            }
            7 => {
                let left = op.param(state, offset, 1);
                let right = op.param(state, offset, 2);
                let dest = state[offset + 3] as usize;
                state[dest] = if left < right { 1 } else { 0 };
                offset += 4;
            }
            8 => {
                let left = op.param(state, offset, 1);
                let right = op.param(state, offset, 2);
                let dest = state[offset + 3] as usize;
                // println!("\t * storing {} == {} into {}", left, right, dest);
                state[dest] = if left == right { 1 } else { 0 };
                offset += 4;
            }
            _ => unreachable!("unknown op code!"),
        };
    }
    output
}

fn parse(value: String) -> Vec<i64> {
    value
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn main() {
    let program = parse(fs::read_to_string("input.txt").unwrap().trim().to_string());
    let output = run(&mut program.clone(), 1);
    assert_eq!(output, 7265618);
    println!("run(1) = {}", output);
    let output = run(&mut program.clone(), 5);
    assert_eq!(output, 7731427);
    println!("run(5) = {}", output);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cat() {
        let mut program = parse(String::from("3,0,4,0,99"));
        let output = run(&mut program, 1);
        assert_eq!(output, 1i64);
    }

    #[test]
    fn test_multiply() {
        let mut program = parse(String::from("1002,4,3,4,33"));
        let _output = run(&mut program, 1);
    }

    #[test]
    fn test_negative() {
        let mut program = parse(String::from("1101,100,-1,4,0"));
        let _output = run(&mut program, 1);
    }

    #[test]
    fn test_read_params() {
        let program = parse(String::from("3,0,4,0,99"));
        let op = get_opcode(program[0]);
        assert_eq!(op.code, 3);
        assert_eq!(program[op.param(&program, 0, 1) as usize], 0);
    }

    #[test]
    fn test_read_immediate_params() {
        let program = vec![1101, 100, -1, 4, 0];
        let op = get_opcode(program[0]);
        assert_eq!(op.code, 1);
        assert_eq!(op.param(&program, 0, 1), 100);
        assert_eq!(op.param(&program, 0, 2), -1);
    }

    #[test]
    fn test_read_position_params() {
        let program = vec![1002, 4, 3, 4, 33];
        let op = get_opcode(program[0]);
        assert_eq!(op.code, 2);
        assert_eq!(op.param(&program, 0, 1), 33);
        assert_eq!(op.param(&program, 0, 2), 3);
    }

    #[test]
    fn test_opcode_parser() {
        assert_eq!(get_opcode(1002).code, 2);
    }
    #[test]
    fn test_mode1() {
        assert_eq!(get_opcode(1002).mode1, 0);
    }
    #[test]
    fn test_mode2() {
        assert_eq!(get_opcode(1002).mode2, 1);
    }
    #[test]
    fn test_mode3() {
        assert_eq!(get_opcode(1002).mode3, 0);
    }

    #[test]
    fn test_double_digit_opcode() {
        assert_eq!(get_opcode(99).code, 99);
    }

    #[test]
    fn test_equal_to_eight() {
        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(run(&mut program.clone(), 8), 1);
        assert_eq!(run(&mut program.clone(), 7), 0);
    }

    #[test]
    fn test_less_than_eight() {
        let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(run(&mut program.clone(), 8), 0);
        assert_eq!(run(&mut program.clone(), 9), 0);
        assert_eq!(run(&mut program.clone(), 3), 1);
    }

    #[test]
    fn test_equal_to_eight_im() {
        let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(run(&mut program.clone(), 8), 1);
        assert_eq!(run(&mut program.clone(), 5), 0);
    }

    #[test]
    fn test_less_than_eight_im() {
        let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(run(&mut program.clone(), 2), 1);
        assert_eq!(run(&mut program.clone(), 9), 0);
    }

    #[test]
    fn test_jump_pos() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(run(&mut program.clone(), 0), 0);
    }

    #[test]
    fn test_jump_im() {
        let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(run(&mut program.clone(), 1), 1);
        assert_eq!(run(&mut program.clone(), 2), 1);
    }

    #[test]
    fn test_lt_8_cmp() {
        let program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(run(&mut program.clone(), 5), 999);
        assert_eq!(run(&mut program.clone(), 8), 1000);
        assert_eq!(run(&mut program.clone(), 15), 1001);
    }
}
