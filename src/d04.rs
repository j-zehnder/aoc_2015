fn find_salt(secret_key: &str, starts_with: &str) -> i32 {
    let mut i = 0;
    loop {
        let data = format!("{}{}", secret_key, i);
        let digest = md5::compute(data);
        let digest = format!("{:?}", digest);
        if digest.starts_with(starts_with) {
            break;
        }
        i += 1;
    }
    i
}

#[aoc(day4, part1)]
pub fn part1(secret_key: &str) -> i32 {
    find_salt(secret_key, "00000")
}

#[aoc(day4, part2)]
pub fn part2(secret_key: &str) -> i32 {
    find_salt(secret_key, "000000")
}
