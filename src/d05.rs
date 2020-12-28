use fancy_regex::Regex;

fn is_nice_p1(line: &str) -> bool {
    // does not contain ab, cd, pq, or xy
    if line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy") {
        return false;
    }

    //   contains at least 3 of aeiou
    let vowel_count = line.matches("a").count()
        + line.matches("e").count()
        + line.matches("i").count()
        + line.matches("o").count()
        + line.matches("u").count();

    //   contains at least 1 double letter
    let re = Regex::new(r"(.)\1").unwrap();

    return vowel_count >= 3 && re.is_match(line).unwrap();
}

fn is_nice_p2(line: &str) -> bool {
    is_nice_p2_c1(line) && is_nice_p2_c2(line)
}

fn is_nice_p2_c1(line: &str) -> bool {
    //It contains a pair of any two letters that appears at least twice in the string without overlapping,
    // like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
    let re = Regex::new(r"(..).*\1").unwrap();
    re.is_match(line).unwrap()
}

fn is_nice_p2_c2(line: &str) -> bool {
    //It contains at least one letter which repeats with exactly one letter between them, like xyx,
    // abcdefeghi (efe), or even aaa.
    let re = Regex::new(r"(.).\1").unwrap();
    re.is_match(line).unwrap()
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    input.lines().filter(|&l| is_nice_p1(l)).count()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    input.lines().filter(|&l| is_nice_p2(l)).count()
}
