use std::cmp::min;

#[derive(Debug)]
pub struct Gift {
    l: i32,
    w: i32,
    h: i32,
}

impl Gift {
    fn from_str(s: &str) -> Self {
        let v = s
            .split('x')
            .map(|i| i.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        Gift {
            l: *v.get(0).unwrap(),
            w: *v.get(1).unwrap(),
            h: *v.get(2).unwrap(),
        }
    }

    fn calculate_wrap_area(&self) -> i32 {
        let area = 2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l;
        let sides = vec![self.l * self.w, self.w * self.h, self.h * self.l];
        let min_area = sides.iter().min().unwrap();
        area + min_area
    }

    fn calculate_ribbon(&self) -> i32 {
        let perimeters = vec![2*(self.l + self.w), 2*(self.w+self.h), 2*(self.h+self.l)];
        let min_perimeter = perimeters.iter().min().unwrap();
        let bow = self.l*self.w*self.h;
        min_perimeter + bow
    }
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Gift> {
    input.lines().map(Gift::from_str).collect()
}

#[aoc(day2, part1)]
pub fn part1(gifts: &[Gift]) -> i32 {
    gifts.iter().map(Gift::calculate_wrap_area).sum()
}

#[aoc(day2, part2)]
pub fn part2(gifts: &[Gift]) -> i32 {
    gifts.iter().map(Gift::calculate_ribbon).sum()
}
