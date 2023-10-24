use std::io::BufRead;

fn main() {
    let file = std::fs::File::open("src/input.txt").unwrap();
    let input = std::io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        // .flatten()
        .collect();
    println!("part1 result: {}", part1(input));
}

enum StringType<'a> {
    Nice(&'a str),
    Naughty(&'a str),
}

fn get_string_type(input: &str) -> StringType {
    let mut vowel_amount = 0;
    let mut twice_letter_amount = 0;
    let mut bad_pair_amount = 0;

    let vowel_string = "aeiou";
    let bad_string_list = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];

    let mut previous_char = None;
    for char in input.chars() {
        if previous_char.is_some_and(|prev_char| prev_char == char) {
            twice_letter_amount += 1;
        }
        if vowel_string.contains(char) {
            vowel_amount += 1;
        }
        if previous_char.is_some_and(|prev_char| bad_string_list.contains(&(prev_char, char))) {
            bad_pair_amount += 1;
        };
        if bad_pair_amount > 0 {
            return StringType::Naughty(input);
        }
        previous_char = Some(char);
    }
    match (bad_pair_amount, vowel_amount, twice_letter_amount) {
        (1.., _, _) => StringType::Naughty(input),
        (_, 3.., 1..) => StringType::Nice(input),
        _ => StringType::Naughty(input),
    }
}

fn part1(input: Vec<String>) -> i32 {
    input
        .into_iter()
        .filter_map(|x| match get_string_type(x.as_str()) {
            StringType::Nice(_) => Some(1),
            StringType::Naughty(_) => None,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_nice_part1_v1() {
        let data = vec!["ugknbfddgicrmopn"];
        let actual = part1(data);
        assert_eq!(actual, 1)
    }

    #[test]
    fn should_nice_part1_v2() {
        let data = vec!["aaa"];
        let actual = part1(data);
        assert_eq!(actual, 1)
    }

    #[test]
    fn should_bad_part1_v1() {
        let data = vec!["jchzalrnumimnmhp"];
        let actual = part1(data);
        assert_eq!(actual, 0)
    }

    #[test]
    fn should_bad_part1_v2() {
        let data = vec!["haegwjzuvuyypxyu"];
        let actual = part1(data);
        assert_eq!(actual, 0)
    }

    #[test]
    fn should_bad_part1_v3() {
        let data = vec!["dvszwmarrgswjxmb"];
        let actual = part1(data);
        assert_eq!(actual, 0)
    }
}
