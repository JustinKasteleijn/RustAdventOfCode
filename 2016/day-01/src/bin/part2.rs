use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let mut occurences: HashMap<u32, u32> = HashMap::new();

    let mut left = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("   ").unwrap();
            let left = a.parse::<u32>().unwrap();
            let right = b.parse::<u32>().unwrap();
            occurences
                .entry(right)
                .and_modify(|c| *c += 1)
                .or_insert(1);
            return left
        }).collect::<Vec<u32>>();

    return left
        .iter()
        .map(|&a| a * occurences.get(&a).unwrap_or(&0))
        .sum::<u32>();
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn part2_test() {
        let result = part2("3   4
4   3
2   5
1   3
3   9
3   3");
        assert_eq!(result, 31);
    }
}
