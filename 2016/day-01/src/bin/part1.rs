fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let (mut left, mut right) = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("   ").unwrap();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        }).collect::<(Vec<u32>, Vec<u32>)>();

    left.sort_unstable();
    right.sort_unstable();

    assert_eq!(left.len(), right.len());

    return left
        .iter()
        .zip(right.iter())
        .map(|(&a, &b)| (b as i32 - a as i32).abs())
        .sum::<i32>() as u32;
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let result = part1("3   4
4   3
2   5
1   3
3   9
3   3");
        assert_eq!(result, 11);
    }
}