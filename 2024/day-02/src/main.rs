use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("src/input.txt").unwrap();
    let input = parse_input(BufReader::new(file).lines().map(Result::unwrap));

    println!("part1: {}", part1(&input));
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<Vec<i32>> {
    input
        .map(|x| {
            x.split_whitespace().map(|x| x.parse().unwrap()).collect()
        }).collect()
}

fn part1(input: &[Vec<i32>]) -> usize{
    enum Direction {
        Increase,
        Decrease,
    }

    enum Status {
        Safe,
        Unsafe,
    }

    input.iter().map(|report| {
        let (Some(first), Some(second)) = (report.get(1), report.get(2)) else {
            return Status::Unsafe;
        };

        let direction = match first.cmp(second) {
            std::cmp::Ordering::Less => Direction::Increase,
            std::cmp::Ordering::Greater => Direction::Decrease,
            std::cmp::Ordering::Equal => return Status::Unsafe,
        };

        if report.is_sorted_by(|&a,&b| match direction {
            Direction::Increase => a < b, 
            Direction::Decrease => a > b
        } && (1..=3).contains(&a.abs_diff(b))) {
            Status::Safe
        } else {
            Status::Unsafe
        }
    }).filter(|x| matches!(x, Status::Safe)).count()
}

#[test]
fn correct_part1() {
    let file = File::open("src/input_test.txt").unwrap();
    let input = parse_input(BufReader::new(file).lines().map(Result::unwrap));
    assert_eq!(part1(&input), 2);
}

