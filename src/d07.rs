use std::collections::HashMap;

#[derive(Debug)]
pub enum Wire {
    Variable(String),
    Value(u16),
}

impl Wire {
    fn from_str(s: &str) -> Self {
        if let Ok(val) = s.parse::<u16>() {
            Wire::Value(val)
        } else {
            Wire::Variable(s.to_string())
        }
    }
}

#[derive(Debug)]
pub enum Rule {
    RShift(Wire, Wire, Wire),
    LShift(Wire, Wire, Wire),
    AND(Wire, Wire, Wire),
    OR(Wire, Wire, Wire),
    NOT(Wire, Wire),
    Signal(Wire, Wire),
}

impl Rule {
    fn from_str(s: &str) -> Self {
        let target = Wire::from_str(s.split(" -> ").last().unwrap());
        let operands = s.split(" -> ").next().unwrap();

        if operands.contains(" RSHIFT ") {
            let mut s1 = operands.split(" RSHIFT ");
            let a = Wire::from_str(s1.next().unwrap());
            let b = Wire::from_str(s1.next().unwrap());
            Rule::RShift(a, b, target)
        } else if operands.contains(" LSHIFT ") {
            let mut s1 = operands.split(" LSHIFT ");
            let a = Wire::from_str(s1.next().unwrap());
            let b = Wire::from_str(s1.next().unwrap());
            Rule::LShift(a, b, target)
        } else if operands.contains(" AND ") {
            let mut s1 = operands.split(" AND ");
            let a = Wire::from_str(s1.next().unwrap());
            let b = Wire::from_str(s1.next().unwrap());
            Rule::AND(a, b, target)
        } else if operands.contains(" OR ") {
            let mut s1 = operands.split(" OR ");
            let a = Wire::from_str(s1.next().unwrap());
            let b = Wire::from_str(s1.next().unwrap());
            Rule::OR(a, b, target)
        } else if operands.contains("NOT ") {
            let s1 = operands.split("NOT ");
            let a = Wire::from_str(s1.last().unwrap());
            Rule::NOT(a, target)
        } else {
            Rule::Signal(Wire::from_str(operands), target)
        }
    }
}

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<Rule> {
    input.lines().map(Rule::from_str).collect()
}

struct Solution {
    solved: HashMap<String, u16>,
}

impl Solution {
    fn new() -> Self {
        Solution {
            solved: HashMap::new(),
        }
    }

    fn solve(&mut self, rules: &[Rule], patch_b: Option<u16>) -> u16 {
        if let Some(v) = patch_b {
            self.solved.insert("b".to_string(), v);
        }

        while self.solved.len() < rules.len() {
            for rule in rules {
                match rule {
                    Rule::Signal(source, target) => {
                        if let Some((s, t)) = self.lookup_two(source, target) {
                            self.solved.entry(t).or_insert(s);
                        }
                    }
                    Rule::NOT(source, target) => {
                        if let Some((s, t)) = self.lookup_two(source, target) {
                            self.solved.entry(t).or_insert(!s);
                        }
                    }
                    Rule::AND(left, right, target) => {
                        if let Some((l, r, t)) = self.lookup_three(left, right, target) {
                            self.solved.entry(t).or_insert(l & r);
                        }
                    }
                    Rule::OR(left, right, target) => {
                        if let Some((l, r, t)) = self.lookup_three(left, right, target) {
                            self.solved.entry(t).or_insert(l | r);
                        }
                    }
                    Rule::LShift(left, right, target) => {
                        if let Some((l, r, t)) = self.lookup_three(left, right, target) {
                            self.solved.entry(t).or_insert(l << r);
                        }
                    }
                    Rule::RShift(left, right, target) => {
                        if let Some((l, r, t)) = self.lookup_three(left, right, target) {
                            self.solved.entry(t).or_insert(l >> r);
                        }
                    }
                }
            }
        }

        *self.solved.get(&String::from("a")).unwrap()
    }

    fn lookup_three(&self, left: &Wire, right: &Wire, target: &Wire) -> Option<(u16, u16, String)> {
        if let Wire::Variable(t) = target {
            let mut lv: u16 = 0;
            let mut rv: u16 = 0;

            if let Wire::Value(tlv) = left {
                lv = *tlv;
            } else if let Wire::Variable(l) = left {
                if let Some(tlv) = self.solved.get(l) {
                    lv = *tlv;
                } else {
                    return None;
                }
            }

            if let Wire::Value(trv) = right {
                rv = *trv;
            } else if let Wire::Variable(r) = right {
                if let Some(trv) = self.solved.get(r) {
                    rv = *trv;
                } else {
                    return None;
                }
            }
            Some((lv, rv, t.clone()))
        } else {
            panic!("target must be a variable")
        }
    }

    fn lookup_two(&self, source: &Wire, target: &Wire) -> Option<(u16, String)> {
        if let Wire::Variable(t) = target {
            let mut sv = 0;

            if let Wire::Value(tsv) = source {
                sv = *tsv;
            } else if let Wire::Variable(s) = source {
                if let Some(tsv) = self.solved.get(s) {
                    sv = *tsv;
                } else {
                    return None;
                }
            }

            Some((sv, t.clone()))
        } else {
            panic!("target must be a variable")
        }
    }
}

#[aoc(day7, part1)]
pub fn part1(rules: &[Rule]) -> u16 {
    let mut solution = Solution::new();
    solution.solve(rules, None)
}

#[aoc(day7, part2)]
pub fn part2(rules: &[Rule]) -> u16 {
    let mut solution = Solution::new();
    solution.solve(rules, Some(46065))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d07_p1_t1() {
        let rules = parse_input(SAMPLE_1);
        assert_eq!(65412, part1(&rules));
    }

    const SAMPLE_1: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> a
NOT y -> i";
}
