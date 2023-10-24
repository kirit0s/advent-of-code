fn main() {
    println!("part1 result: {:?}", part1("bgvyzdsv"));
    println!("part1 result: {:?}", part2("bgvyzdsv"));
}

fn get_lowest_number_for_hash(input: &str, start_with: &str) -> (i32, md5::Digest) {
    let mut data = (0..10_000_000).map(|number| {
        let data = format!("{}{}", input, number);
        let hash = md5::compute(data);
        let collect = format!("{:?}", hash);
        match collect.starts_with(start_with) {
            true => Some((number, hash)),
            false => None,
        }
    });
    loop {
        if let Some(x) = data.next().unwrap() {
            break x;
        }
    }
}

fn part1(input: &str) -> (i32, md5::Digest) {
    get_lowest_number_for_hash(input, "00000")
}

fn part2(input: &str) -> (i32, md5::Digest) {
    get_lowest_number_for_hash(input, "000000")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_part1_v1() {
        let input = "abcdef";
        let (actual, _) = part1(input);
        assert_eq!(actual, 609043)
    }

    #[test]
    fn correct_part1_v2() {
        let input = "pqrstuv";
        let (actual, _) = part1(input);
        assert_eq!(actual, 1048970)
    }
}
