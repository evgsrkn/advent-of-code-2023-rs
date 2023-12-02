use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/input2.txt");
    let res = proccess(input);
    dbg!(res);
}

fn proccess(input: &str) -> isize {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split_once(": ").unwrap())
        .map(|(game, sets)| (game, sets.split("; ").collect::<Vec<&str>>()))
        .map(|(_, sets)| {
            let mut rgb = HashMap::from([("red", 1), ("green", 1), ("blue", 1)]);
            for set in sets {
                set.split(", ").for_each(|s| {
                    let color = s.split_once(" ").unwrap();
                    let tmp = color.0.parse::<isize>().unwrap();
                    if rgb[color.1] < tmp {
                        *rgb.get_mut(color.1).unwrap() = tmp
                    }
                });
            }
            rgb.values().product::<isize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::proccess;

    #[test]
    fn part2() {
        let res = proccess(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );

        assert_eq!(res, 2286)
    }
}
