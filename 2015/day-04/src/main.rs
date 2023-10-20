fn main() {
    let x = part1("abcdef");
    println!("{:?}", x);
}

fn part1(input: &str) -> md5::Digest {
    let mut data = (0..1_000_000).map(|number| {
        let data = format!("{}{}", input, number);
        let hash = md5::compute(data);
        // println!("{:?}", hash);
        // let x = format!("{:?}", hash.iter().take(5).collect::<Vec<&u8>>());
        println!("{}", number);
        let collect = format!("{:?}", hash);
        match collect.starts_with("00000") {
            true => Some(hash),
            false => None,
        }
    });
    loop {
        if let Some(x) = data.next().unwrap() {
            break x;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn correct_part1() {
    //     let input = "abcdef";
    //     let actual = part1(input);
    //     assert_eq!(actual, "asd")
    // }
}
