use std::collections::HashMap;

#[derive(Debug)]
pub struct DeerData {
    name: String,
    speed: i32,
    fly_time: i32,
    rest_time: i32,
}

impl DeerData {
    fn from_str(s: &str) -> Self {
        if let Ok((name, speed, fly_time, rest_time)) = scan_fmt!(
            s,
            "{} can fly {d} km/s for {d} seconds, but then must rest for {d} seconds.",
            String,
            i32,
            i32,
            i32
        ) {
            return DeerData {
                name,
                speed,
                fly_time,
                rest_time,
            };
        }
        panic!("invalid input format")
    }
}

#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> Vec<DeerData> {
    input.lines().map(DeerData::from_str).collect()
}

const RACE_TIME_LIMIT: i32 = 2503;

#[aoc(day14, part1)]
pub fn part1(data: &[DeerData]) -> i32 {
    data.iter()
        .map(|data| distance_travelled(data, RACE_TIME_LIMIT))
        .max()
        .unwrap()
}

fn distance_travelled(data: &DeerData, time: i32) -> i32 {
    let full_cycles = time / (data.fly_time + data.rest_time);
    let remaining_time = time % (data.fly_time + data.rest_time);

    let fly_in_remainder: i32;
    if remaining_time > data.fly_time {
        fly_in_remainder = data.fly_time
    } else {
        fly_in_remainder = remaining_time
    }

    (full_cycles * data.fly_time * data.speed) + (fly_in_remainder * data.speed)
}

#[aoc(day14, part2)]
pub fn part2(data: &[DeerData]) -> i32 {
    let mut scores: HashMap<String, i32> = HashMap::new();
    for d in data {
        scores.insert(d.name.clone(), 0);
    }

    for t in 1..=RACE_TIME_LIMIT {
        let dist_at_t: Vec<(String, i32)> = data
            .iter()
            .map(|d| (d.name.clone(), distance_travelled(d, t)))
            .collect();
        let max_d_at_t = dist_at_t.iter().map(|i| i.1).max().unwrap();

        for d in dist_at_t {
            if d.1 == max_d_at_t {
                let score = scores.get_mut(&d.0).unwrap();
                *score += 1;
            }
        }
    }

    *scores.iter().map(|s| s.1).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d14_p1_t1() {
        let data = parse_input(SAMPLE1);
        assert_eq!(140, distance_travelled(&data[0], 12));
        assert_eq!(176, distance_travelled(&data[1], 12));

        assert_eq!(1120, distance_travelled(&data[0], 1000));
        assert_eq!(1056, distance_travelled(&data[1], 1000));
    }
    const SAMPLE1: &str =
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
    Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
}
