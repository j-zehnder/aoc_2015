use fancy_regex::Regex;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Sue {
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

impl Sue {
    fn from_str(s: &str) -> Self {
        //Sue 2: goldfish: 7, trees: 1, akitas: 0
        let mut sue = Sue::default();
        [
            "children",
            "cats",
            "samoyeds",
            "pomeranians",
            "akitas",
            "vizslas",
            "goldfish",
            "trees",
            "cars",
            "perfumes",
        ]
        .iter()
        .for_each(|k| {
            let re = Regex::new(format!("({}): (\\d+)", k).as_str()).unwrap();
            if let Some(c) = re.captures(s).unwrap() {
                let field = c.get(1).unwrap().as_str();
                let ct = c.get(2).unwrap().as_str().parse::<usize>().unwrap();
                match field {
                    "children" => sue.children = Some(ct),
                    "cats" => sue.cats = Some(ct),
                    "samoyeds" => sue.samoyeds = Some(ct),
                    "pomeranians" => sue.pomeranians = Some(ct),
                    "akitas" => sue.akitas = Some(ct),
                    "vizslas" => sue.vizslas = Some(ct),
                    "goldfish" => sue.goldfish = Some(ct),
                    "trees" => sue.trees = Some(ct),
                    "cars" => sue.cars = Some(ct),
                    "perfumes" => sue.perfumes = Some(ct),
                    _ => {}
                }
            }
        });
        sue
    }
}
#[aoc_generator(day16)]
pub fn parse_input(input: &str) -> Vec<Sue> {
    input.lines().map(Sue::from_str).collect()
}

#[aoc(day16, part1)]
pub fn part1(sues: &[Sue]) -> usize {
    let mut matches: Vec<(usize, &Sue)> = Vec::new();

    for (i, sue) in sues.iter().enumerate() {
        if (sue.children.is_none() || sue.children == Some(3))
            && (sue.cats.is_none() || sue.cats == Some(7))
            && (sue.samoyeds.is_none() || sue.samoyeds == Some(2))
            && (sue.pomeranians.is_none() || sue.pomeranians == Some(3))
            && (sue.akitas.is_none() || sue.akitas == Some(0))
            && (sue.vizslas.is_none() || sue.vizslas == Some(0))
            && (sue.goldfish.is_none() || sue.goldfish == Some(5))
            && (sue.trees.is_none() || sue.trees == Some(3))
            && (sue.cars.is_none() || sue.cars == Some(2))
            && (sue.perfumes.is_none() || sue.perfumes == Some(1))
        {
            matches.push((i + 1, sue))
        }
    }

    if matches.len() == 1 {
        return matches[0].0;
    }
    panic!("unable to find match!")
}

#[aoc(day16, part2)]
pub fn part2(sues: &[Sue]) -> usize {
    // In particular, the cats and trees readings indicates that there are greater than that many
    // (due to the unpredictable nuclear decay of cat dander and tree pollen),
    // while the pomeranians and goldfish readings indicate that there are fewer than that many (due to the modial interaction of magnetoreluctance).
    let mut matches: Vec<(usize, &Sue)> = Vec::new();

    for (i, sue) in sues.iter().enumerate() {
        if (sue.children.is_none() || sue.children == Some(3))
            && (sue.cats.is_none() || sue.cats > Some(7))
            && (sue.samoyeds.is_none() || sue.samoyeds == Some(2))
            && (sue.pomeranians.is_none() || sue.pomeranians < Some(3))
            && (sue.akitas.is_none() || sue.akitas == Some(0))
            && (sue.vizslas.is_none() || sue.vizslas == Some(0))
            && (sue.goldfish.is_none() || sue.goldfish < Some(5))
            && (sue.trees.is_none() || sue.trees > Some(3))
            && (sue.cars.is_none() || sue.cars == Some(2))
            && (sue.perfumes.is_none() || sue.perfumes == Some(1))
        {
            matches.push((i + 1, sue))
        }
    }

    if matches.len() == 1 {
        return matches[0].0;
    }

    println!("possible matches");
    for m in matches {
        println!("{} - {:?}", m.0, m.1);
    }
    panic!("unable to find match!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d16_parse_test() {
        let mut expected = Sue::default();
        expected.goldfish = Some(7);
        expected.trees = Some(1);
        expected.akitas = Some(0);
        assert_eq!(expected, parse_input(SAMPLE)[0]);
    }

    const SAMPLE: &str = "Sue 2: goldfish: 7, trees: 1, akitas: 0";
}
