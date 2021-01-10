#[derive(Debug)]
pub enum Instruction {
    HLF(char),
    TPL(char),
    INC(char),
    JMP(i32),
    JIE(char, i32),
    JIO(char, i32),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let mut spl = s.split(' ');

        let one = spl.next().unwrap();
        let two = spl.next().unwrap();

        match one {
            "hlf" => Instruction::HLF(two.chars().next().unwrap()),
            "tpl" => Instruction::TPL(two.chars().next().unwrap()),
            "inc" => Instruction::INC(two.chars().next().unwrap()),
            "jmp" => Instruction::JMP(two.parse::<i32>().unwrap()),
            "jie" => Instruction::JIE(
                two.chars().next().unwrap(),
                spl.next().unwrap().parse::<i32>().unwrap(),
            ),
            "jio" => Instruction::JIO(
                two.chars().next().unwrap(),
                spl.next().unwrap().parse::<i32>().unwrap(),
            ),
            _ => panic!("invalid instruction"),
        }
    }
}

struct TuringMachine {
    a: usize,
    b: usize,
    ins_ptr: usize,
}

impl TuringMachine {
    fn new() -> Self {
        TuringMachine {
            a: 0,
            b: 0,
            ins_ptr: 0,
        }
    }

    fn run(&mut self, instructions: &[Instruction]) {
        while self.ins_ptr < instructions.len() {
            let ins = &instructions[self.ins_ptr];

            match ins {
                Instruction::HLF(r) => {
                    match r {
                        'a' => self.a /= 2,
                        'b' => self.b /= 2,
                        _ => panic!("unknown register"),
                    }
                    self.ins_ptr += 1;
                }
                Instruction::TPL(r) => {
                    match r {
                        'a' => self.a *= 3,
                        'b' => self.b *= 3,
                        _ => panic!("unknown register"),
                    }
                    self.ins_ptr += 1;
                }
                Instruction::INC(r) => {
                    match r {
                        'a' => self.a += 1,
                        'b' => self.b += 1,
                        _ => panic!("unknown register"),
                    }
                    self.ins_ptr += 1;
                }
                Instruction::JMP(amt) => {
                    self.ins_ptr = (self.ins_ptr as i32 + amt) as usize;
                }
                Instruction::JIE(r, amt) => match r {
                    'a' => {
                        if self.a % 2 == 0 {
                            self.ins_ptr = (self.ins_ptr as i32 + amt) as usize;
                        } else {
                            self.ins_ptr += 1;
                        }
                    }
                    'b' => {
                        if self.b % 2 == 0 {
                            self.ins_ptr = (self.ins_ptr as i32 + amt) as usize;
                        } else {
                            self.ins_ptr += 1;
                        }
                    }
                    _ => panic!("unknown register"),
                },
                Instruction::JIO(r, amt) => match r {
                    'a' => {
                        if self.a == 1 {
                            self.ins_ptr = (self.ins_ptr as i32 + amt) as usize;
                        } else {
                            self.ins_ptr += 1;
                        }
                    }
                    'b' => {
                        if self.b == 1 {
                            self.ins_ptr = (self.ins_ptr as i32 + amt) as usize;
                        } else {
                            self.ins_ptr += 1;
                        }
                    }
                    _ => panic!("unknown register"),
                },
            }
        }
    }
}

#[aoc_generator(day23)]
pub fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from_str).collect()
}

#[aoc(day23, part1)]
pub fn part1(instructions: &[Instruction]) -> usize {
    let mut tm = TuringMachine::new();
    tm.run(&instructions);
    tm.b
}

#[aoc(day23, part2)]
pub fn part2(instructions: &[Instruction]) -> usize {
    let mut tm = TuringMachine::new();
    tm.a = 1;
    tm.run(&instructions);
    tm.b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d23_parse_test() {}
}
