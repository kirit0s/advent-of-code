use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let file = File::open("src/input.txt").unwrap();
    let input = BufReader::new(file).bytes().map(|x| char::from(x.unwrap()));
    println!("part1: {}", part1(input));
}

fn part1(input: impl Iterator<Item = char>) -> i32 {
    let mut multiplier_list: Vec<(i32, i32)> = Vec::new();

    #[derive(Debug)]
    struct Command {
        position: u8,
    }

    #[derive(Debug, Default)]
    struct Multiplier {
        position: u8,
        value: String,
    }

    #[derive(Debug)]
    enum ParseStage {
        Command(Command),
        MultiplierOne(Multiplier),
        MultiplierTwo((Multiplier, Multiplier)),
        Empty,
    }

    let mut stage = ParseStage::Empty;

    for char in input {
        println!("{:?}, {}", stage, char);
        match stage {
            ParseStage::Empty => match char {
                'm' => stage = ParseStage::Command(Command { position: 1 }),
                _ => stage = ParseStage::Empty,
            },
            ParseStage::Command(x) => match (x.position, char) {
                (1, 'u') => stage = ParseStage::Command(Command { position: 2 }),
                (2, 'l') => stage = ParseStage::Command(Command { position: 3 }),
                (3, '(') => stage = ParseStage::MultiplierOne(Multiplier::default()),
                _ => stage = ParseStage::Empty,
            },
            ParseStage::MultiplierOne(x) => {
                match (x.position, char.is_ascii_digit().then_some(char), char) {
                    (0..=2, Some(value), _) => {
                        stage = ParseStage::MultiplierOne(Multiplier {
                            position: x.position + 1,
                            value: x.value + &value.to_string(),
                        })
                    }
                    (1..=3, _, ',') => {
                        stage = ParseStage::MultiplierTwo((x, Multiplier::default()))
                    }
                    _ => stage = ParseStage::Empty,
                }
            }
            ParseStage::MultiplierTwo((mul_1, x)) => {
                match (x.position, char.is_ascii_digit().then_some(char), char) {
                    (0..=2, Some(value), _) => {
                        stage = ParseStage::MultiplierTwo((
                            mul_1,
                            Multiplier {
                                position: x.position + 1,
                                value: x.value + &value.to_string(),
                            },
                        ))
                    }
                    (1..=3, _, ')') => {
                        multiplier_list
                            .push((mul_1.value.parse().unwrap(), x.value.parse().unwrap()));
                        stage = ParseStage::Empty
                    }
                    _ => stage = ParseStage::Empty,
                }
            }
        };
    }

    multiplier_list.iter().map(|(x, y)| x * y).sum()
}

#[test]
fn correct_part1() {
    let file = File::open("src/input_test.txt").unwrap();
    let input = BufReader::new(file).bytes().map(|x| char::from(x.unwrap()));
    assert_eq!(part1(input), 161);
}
