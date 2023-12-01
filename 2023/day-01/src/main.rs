fn main() {
    let file = std::fs::File::open("src/input.txt").unwrap();
    let input: Vec<String> = std::io::BufRead::lines(std::io::BufReader::new(file))
        .map(|x| x.unwrap())
        .collect();
    println!("part1 result: {}", part1(&input));
}

fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|text| {
            text.chars()
                .filter_map(|x| x.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|x| x.first().unwrap() * 10 + x.last().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_correct_part1() {
        let data = vec![
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
            String::from("treb7uchet"),
        ];
        let actual = part1(&data);
        assert_eq!(actual, 142);
    }
}
