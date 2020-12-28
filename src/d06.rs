pub enum LightInstruction {
    TurnOn(usize, usize, usize, usize),
    TurnOff(usize, usize, usize, usize),
    Toggle(usize, usize, usize, usize),
}

impl LightInstruction {
    fn from_str(s: &str) -> Self {
        let split = s.split(" ").collect::<Vec<&str>>();

        if split.get(0).unwrap() == &"toggle" {
            let c1 = split.get(1).unwrap();
            let x1 = c1
                .split(",")
                .collect::<Vec<&str>>()
                .get(0)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let y1 = c1
                .split(",")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let c2 = split.get(3).unwrap();
            let x2 = c2
                .split(",")
                .collect::<Vec<&str>>()
                .get(0)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let y2 = c2
                .split(",")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();

            return LightInstruction::Toggle(x1, y1, x2, y2);
        } else if split.get(1).unwrap() == &"on" {
            let c1 = split.get(2).unwrap();
            let x1 = c1
                .split(",")
                .collect::<Vec<&str>>()
                .get(0)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let y1 = c1
                .split(",")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let c2 = split.get(4).unwrap();
            let x2 = c2
                .split(",")
                .collect::<Vec<&str>>()
                .get(0)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let y2 = c2
                .split(",")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();

            return LightInstruction::TurnOn(x1, y1, x2, y2);
        } else if split.get(1).unwrap() == &"off" {
            let c1 = split.get(2).unwrap();
            let x1 = c1
                .split(",")
                .collect::<Vec<&str>>()
                .get(0)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let y1 = c1
                .split(",")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let c2 = split.get(4).unwrap();
            let x2 = c2
                .split(",")
                .collect::<Vec<&str>>()
                .get(0)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let y2 = c2
                .split(",")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();

            return LightInstruction::TurnOff(x1, y1, x2, y2);
        }

        panic!("invalid format")
    }
}

fn turn_off(lights: &mut [[bool; 1000]; 1000], x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..x2 + 1 {
        for y in y1..y2 + 1 {
            lights[x][y] = false;
        }
    }
}

fn turn_on(lights: &mut [[bool; 1000]; 1000], x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..x2 + 1 {
        for y in y1..y2 + 1 {
            lights[x][y] = true;
        }
    }
}

fn toggle(lights: &mut [[bool; 1000]; 1000], x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..x2 + 1 {
        for y in y1..y2 + 1 {
            lights[x][y] = !lights[x][y];
        }
    }
}

fn dim(lights: &mut [[u32; 1000]; 1000], x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..x2 + 1 {
        for y in y1..y2 + 1 {
            if lights[x][y] != 0 {
                lights[x][y] -= 1;
            }
        }
    }
}

fn brighten(
    lights: &mut [[u32; 1000]; 1000],
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    amount: u32,
) {
    for x in x1..x2 + 1 {
        for y in y1..y2 + 1 {
            lights[x][y] += amount;
        }
    }
}

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<LightInstruction> {
    input.lines().map(LightInstruction::from_str).collect()
}

#[aoc(day6, part1)]
pub fn part1(instructions: &[LightInstruction]) -> usize {
    let mut lights = [[false; 1000]; 1000];
    instructions.iter().for_each(|i| match i {
        LightInstruction::Toggle(x1, y1, x2, y2) => {
            toggle(&mut lights, *x1, *y1, *x2, *y2);
        }
        LightInstruction::TurnOff(x1, y1, x2, y2) => {
            turn_off(&mut lights, *x1, *y1, *x2, *y2);
        }
        LightInstruction::TurnOn(x1, y1, x2, y2) => {
            turn_on(&mut lights, *x1, *y1, *x2, *y2);
        }
    });

    lights.iter().flat_map(|r| r.iter()).filter(|b| **b).count()
}

#[aoc(day6, part2)]
pub fn part2(instructions: &[LightInstruction]) -> u32 {
    let mut light_intensity: [[u32; 1000]; 1000] = [[0; 1000]; 1000];

    instructions.iter().for_each(|i| match i {
        LightInstruction::Toggle(x1, y1, x2, y2) => {
            brighten(&mut light_intensity, *x1, *y1, *x2, *y2, 2);
        }
        LightInstruction::TurnOff(x1, y1, x2, y2) => {
            dim(&mut light_intensity, *x1, *y1, *x2, *y2);
        }
        LightInstruction::TurnOn(x1, y1, x2, y2) => {
            brighten(&mut light_intensity, *x1, *y1, *x2, *y2, 1);
        }
    });
    light_intensity.iter().flat_map(|r| r.iter()).sum()
}
