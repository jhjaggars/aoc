use std::collections::VecDeque;
use std::fs;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[derive(Debug)]
struct Op {
    code: usize,
    mode1: usize,
    mode2: usize,
    mode3: usize,
}

#[derive(Debug)]
struct IntCodeComputer {
    state: Vec<i64>,
    offset: usize,
    relative_base: i64,
}

#[derive(Debug)]
struct ICResult {
    output: Vec<i64>,
    done: bool,
}

impl IntCodeComputer {
    fn new(mut program: Vec<i64>) -> IntCodeComputer {
        program.resize_with(2 << 16, Default::default);
        IntCodeComputer {
            state: program,
            offset: 0,
            relative_base: 0,
        }
    }

    fn get_opcode(&mut self) -> Op {
        let input = self.state[self.offset];
        Op {
            code: (input % 100) as usize,
            mode1: ((input / 100) % 10) as usize,
            mode2: ((input / 1000) % 10) as usize,
            mode3: ((input / 10000) % 10) as usize,
        }
    }

    fn param(&mut self, op: &Op, pos: usize) -> i64 {
        let actual = self.state[self.offset + pos];
        let mode = match pos {
            1 => op.mode1,
            2 => op.mode2,
            3 => op.mode3,
            _ => unreachable!("oh jesus!"),
        };
        debug!(
            "\top: {:?}, pos: {}, relative_base: {}, actual: {}",
            op, pos, self.relative_base, actual
        );
        match mode {
            0 => self.state[actual as usize],
            1 => actual,
            2 => self.state[(self.relative_base + actual) as usize],
            _ => unreachable!("invalid mode"),
        }
    }

    fn dest(&mut self, op: &Op, pos: usize) -> usize {
        let mode = match pos {
            1 => op.mode1,
            2 => op.mode2,
            3 => op.mode3,
            _ => unreachable!("oh jesus!"),
        };
        match mode {
            0 => self.state[self.offset + pos] as usize,
            2 => {
                let x = self.state[self.offset + pos];
                (x + self.relative_base) as usize
            }
            _ => unreachable!("invalid dest offset"),
        }
    }

    fn run(&mut self, input: Vec<i64>) -> ICResult {
        let mut output = ICResult {
            output: vec![],
            done: false,
        };
        let mut in_q = VecDeque::from(input);

        loop {
            let op = self.get_opcode();
            debug!("offset is {}", self.offset);
            debug!("\tstate is {:?}", &self.state[..20]);

            if op.code == 99 {
                output.done = true;
                break;
            }

            match op.code {
                1 => {
                    let left = self.param(&op, 1);
                    let right = self.param(&op, 2);
                    // let dest = self.state[self.offset + 3] as usize;
                    let dest = self.dest(&op, 3);
                    debug!(
                        "\t setting {} to {} + {} = {}",
                        dest,
                        left,
                        right,
                        left + right
                    );
                    self.state[dest] = left + right;
                    self.offset += 4;
                }
                2 => {
                    let left = self.param(&op, 1);
                    let right = self.param(&op, 2);
                    // let dest = self.state[self.offset + 3] as usize;
                    let dest = self.dest(&op, 3);
                    debug!("\t setting {} to {}", dest, left * right);
                    self.state[dest] = left * right;
                    self.offset += 4;
                }
                3 => {
                    // let dest = self.state[self.offset + 1] as usize;
                    let dest = self.dest(&op, 1);
                    match in_q.pop_front() {
                        Some(v) => {
                            debug!("reading input: {}, and storing to {}", v, dest);
                            self.state[dest] = v;
                        }
                        None => {
                            debug!("failed to read input");
                            break;
                        }
                    }
                    self.offset += 2;
                }
                4 => {
                    let v = self.param(&op, 1);
                    output.output.push(v);
                    debug!(
                        "\t setting output to {:?}, it is now: {:?}",
                        v, output.output
                    );
                    self.offset += 2;
                }
                5 => {
                    let left = self.param(&op, 1);
                    let right = self.param(&op, 2);
                    // debug!("\t {} != 0", left);
                    if left != 0 {
                        debug!("\t* {} neq 0, jumping to {}", left, right);
                        self.offset = right as usize;
                    } else {
                        self.offset += 3;
                    }
                }
                6 => {
                    let left = self.param(&op, 1);
                    let right = self.param(&op, 2);
                    if left == 0 {
                        debug!("\t* {} eq 0, jumping to {}", left, right);
                        self.offset = right as usize;
                    } else {
                        self.offset += 3;
                    }
                }
                7 => {
                    let left = self.param(&op, 1);
                    let right = self.param(&op, 2);
                    // let dest = self.state[self.offset + 3] as usize;
                    let dest = self.dest(&op, 3);
                    debug!("\t* storing {} < {} into {}", left, right, dest);
                    self.state[dest] = if left < right { 1 } else { 0 };
                    self.offset += 4;
                }
                8 => {
                    let left = self.param(&op, 1);
                    let right = self.param(&op, 2);
                    // let dest = self.state[self.offset + 3] as usize;
                    let dest = self.dest(&op, 3);
                    debug!("\t* storing {} == {} into {}", left, right, dest);
                    self.state[dest] = if left == right { 1 } else { 0 };
                    self.offset += 4;
                }
                9 => {
                    self.relative_base += self.param(&op, 1);
                    debug!("\tsetting relative base to {}", self.relative_base);
                    self.offset += 2;
                }
                _ => unreachable!("unknown op code!"),
            };
        }
        // debug!("output is {:?}", output);
        output
    }
}

fn parse(value: String) -> Vec<i64> {
    value
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn main() {
    pretty_env_logger::init();
    let program = parse(fs::read_to_string("input.txt").unwrap().trim().to_string());
    let mut ic = IntCodeComputer::new(program.clone());
    let result = ic.run(vec![2]);
    println!("BOOST output: {:?}", result.output);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quine() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut ic = IntCodeComputer::new(program.clone());
        let result = ic.run(vec![]);
        assert_eq!(result.output, program);
    }

    #[test]
    fn sixteen() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut ic = IntCodeComputer::new(program.clone());
        let result = ic.run(vec![]);
        assert_eq!(result.output[0].to_string().len(), 16);
    }

    #[test]
    fn bignum() {
        let program = vec![104, 1125899906842624, 99];
        let mut ic = IntCodeComputer::new(program.clone());
        let result = ic.run(vec![]);
        assert_eq!(result.output[0], 1125899906842624);
    }

    #[test]
    fn t203() {
        let program = vec![203, 10, 204, 10, 99];
        let mut ic = IntCodeComputer::new(program.clone());
        let result = ic.run(vec![5]);
        assert_eq!(result.output[0], 5);
    }
}
