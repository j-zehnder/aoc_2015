use regex::Regex;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    input.lines().map(|l| decode_len_difference(l)).sum()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    input.lines().map(|l| encode_len_difference(l)).sum()
}

fn decode_len(code: &str) -> usize {
    let mut escaped = code.replace(r#"\""#, r#"""#);
    escaped = escaped.replace(r#"\\"#, r#"."#);
    let re = Regex::new(r#"\\x(..)"#).unwrap();
    escaped = re.replace_all(escaped.as_str(), "?").parse().unwrap();
    escaped = escaped[1..escaped.len() - 1].parse().unwrap(); // trim surrounding quotes
    escaped.len()
}

fn decode_len_difference(code: &str) -> usize {
    code.len() - decode_len(code)
}

fn encode_len(code: &str) -> usize {
    let mut encoded = code.replace("\"", ".\"");
    encoded = encoded.replace("\\", "..");
    encoded.len() + 2
}

fn encode_len_difference(code: &str) -> usize {
    encode_len(code) - code.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d08_p1_t1() {
        assert_eq!(2, decode_len_difference(r#""""#));
        assert_eq!(2, decode_len_difference(r#""abc""#));
        assert_eq!(3, decode_len_difference(r#""aaa\"aaa""#));
        assert_eq!(5, decode_len_difference(r#""\x27""#));
        assert_eq!(8, decode_len_difference(r#""\x27\x28""#));
    }

    #[test]
    fn d08_p2_t1() {
        assert_eq!(6, encode_len(r#""""#));
        assert_eq!(9, encode_len(r#""abc""#));
        assert_eq!(16, encode_len(r#""aaa\"aaa""#));
        assert_eq!(11, encode_len(r#""\x27""#));
    }
}
