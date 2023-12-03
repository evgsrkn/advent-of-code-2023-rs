use std::collections::LinkedList;

fn main() {
    let input = include_str!("../../input/input2.txt");
    let res = proccess(input);
    dbg!(res);
}

fn proccess(input: &str) -> u32 {
    let mut res = 0;
    let mut map = vec![vec![' '; input.lines().next().unwrap().len()]; input.lines().count()];

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            map[i][j] = char
        }
    }

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '*' {
                res += find_gear(i as isize, j as isize, &mut map);
            }
        }
    }

    res
}

fn find_gear(i: isize, j: isize, map: &mut Vec<Vec<char>>) -> u32 {
    let mut res = 1;
    let mut count = 0;

    let directions = [
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];

    for d in directions {
        if j > 0
            && i > 0
            && j < map[0].len() as isize - 1
            && i < map.len() as isize - 1
            && map[(i + d.0) as usize][(j + d.1) as usize].is_digit(10)
        {
            count += 1;
            res *= get_digit(i + d.0, j + d.1, map);
        }
    }

    if count == 2 {
        res
    } else {
        0
    }
}

fn get_digit(i: isize, j: isize, map_out: &mut Vec<Vec<char>>) -> u32 {
    let mut digits = vec![];
    let mut queue = LinkedList::new();
    let mut map = map_out.clone();
    queue.push_back((i, j));

    while !queue.is_empty() {
        let n = queue.pop_front().unwrap();
        digits.push((n.0 as usize, n.1 as usize));
        map[n.0 as usize][n.1 as usize] = '.';

        if n.1 > 0 && map[n.0 as usize][(n.1 - 1) as usize].is_digit(10) {
            queue.push_back((n.0, n.1 - 1));
        }

        if n.1 + 1 < map[0].len() as isize && map[n.0 as usize][(n.1 + 1) as usize].is_digit(10) {
            queue.push_back((n.0, n.1 + 1));
        }
    }

    digits.sort_by(|a, b| (&a.1).cmp(&b.1));

    digits
        .iter()
        .map(|(i, j)| {
            let res = map_out[*i][*j];
            map_out[*i][*j] = '.';
            res
        })
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::proccess;

    #[test]
    fn part2() {
        let res = proccess(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );

        assert_eq!(res, 467835)
    }
}
