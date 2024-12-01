use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("src/input.txt").unwrap();
    let input = parse_input(BufReader::new(file).lines().map(Result::unwrap));

    println!("part1: {}", part1(&input.0, &input.1));
    println!("part2: todo!()");
}

fn parse_input(input: impl Iterator<Item = String>) -> (Vec<i32>, Vec<i32>) {
    input
        .map(|x| {
            let x = x.split_once("   ").unwrap();
            (x.0.to_string(), x.1.to_string())
        })
        .fold((Vec::new(), Vec::new()), |mut acc, x| {
            acc.0.push(x.0.parse().unwrap());
            acc.1.push(x.1.parse().unwrap());
            acc
        })
}

fn part1(list1: &[i32], list2: &[i32]) -> u32 {
    let list1 = {
        let mut x = list1.to_vec();
        x.sort();
        x
    };

    let list2 = {
        let mut x = list2.to_vec();
        x.sort();
        x
    };

    list1.iter().zip(list2).map(|x| x.0.abs_diff(x.1)).sum()
}

#[test]
fn correct_part1() {
    let file = File::open("src/input_test.txt").unwrap();
    let input = parse_input(BufReader::new(file).lines().map(Result::unwrap));
    assert_eq!(part1(&input.0, &input.1), 11u32);
}
