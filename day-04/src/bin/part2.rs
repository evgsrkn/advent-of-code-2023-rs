use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/input2.txt");
    let result = proccess(input);
    dbg!(result);
}

fn proccess(input: &str) -> u32 {
    let mut cards = HashMap::new();

    input
        .lines()
        .map(|l| l.split_once(":").unwrap())
        .map(|(a, b)| (a.split_once(" ").unwrap().1, b.split_once(" | ").unwrap()))
        .map(|(a, b)| (a, (b.0.split_whitespace(), b.1.split_whitespace())))
        .map(|(a, b)| {
            (
                a.parse::<u32>().unwrap(),
                (
                    b.0.map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>(),
                    b.1.map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>(),
                ),
            )
        })
        .for_each(|(a, b)| {
            cards.insert(
                a,
                b.1.iter()
                    .filter(|n| b.0.contains(n))
                    .collect::<Vec<&u32>>()
                    .len(),
            );
        });

    println!("{:?}", cards);

    0
}

#[cfg(test)]
mod tests {
    use crate::proccess;

    #[test]
    fn part2() {
        let res = proccess(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );

        assert_eq!(res, 30)
    }
}
