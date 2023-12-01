fn main() {
    let input = include_str!("./../../input/input2.txt");
    let result = proccess(input);
    dbg!(result);
}

fn proccess(input: &str) -> u32 {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .lines()
        .map(|line| line.to_string())
        .map(|mut line| {
            while !words.into_iter().any(|word| line.starts_with(word))
                && !line.chars().next().unwrap().is_digit(10)
            {
                line = line[1..].to_string();
            }
            while !words.into_iter().any(|word| line.ends_with(word))
                && !line.chars().last().unwrap().is_digit(10)
            {
                line = line[..line.len() - 1].to_string();
            }

            words.iter().enumerate().for_each(|(i, word)| {
                if line.starts_with(word) || line.ends_with(word) {
                    line = line.to_string().replace(word, (i + 1).to_string().as_str());
                }
            });

            let chs = line.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();
            if chs.len() == 0 {
                0
            } else {
                format!("{}{}", chs.first().unwrap(), chs.last().unwrap())
                    .parse::<u32>()
                    .unwrap()
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2() {
        let res = proccess(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(281, res)
    }
}
