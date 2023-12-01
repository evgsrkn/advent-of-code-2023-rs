fn main() {
    let input = include_str!("./../../input/input1.txt");
    let result = proccess(input);
    dbg!(result);
}

fn proccess(input: &str) -> u32 {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>())
        .map(|v| {
            let first = v.first().unwrap();
            let last = v.last().unwrap();

            vec![first, last]
                .into_iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let res = proccess(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(142, res)
    }
}
