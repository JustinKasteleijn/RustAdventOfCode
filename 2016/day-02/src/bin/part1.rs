fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

struct Rules {
    levels: Vec<Vec<u32>>,
}

impl Rules {
    fn new(levels: Vec<Vec<u32>>) -> Rules {
        return Rules {
            levels,
        };
    }

    fn count_safe_reports(self) -> u32 {
        return self.levels
            .iter()
            .filter(|level| {
                level.windows(2).all(|levels| {
                    Self::is_safe(levels[0], levels[1])
                }) && Self::all_increasing_or_decreasing(level)
            })
            .count() as u32;
    }

    const fn is_safe(a: u32, b: u32) -> bool {
        return a.abs_diff(b) >= 1 && a.abs_diff(b) <= 3;
    }
    fn all_increasing_or_decreasing(level: &[u32]) -> bool {
        let increasing = level
            .windows(2)
            .all(|pair| pair[0] < pair[1]);

        let decreasing = level
            .windows(2)
            .all(|pair| pair[0] > pair[1]);

        return increasing ^ decreasing;
    }
}

fn part1(input: &str) -> u32 {
    let levels: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
        )
        .collect::<Vec<Vec<u32>>>();

    let rules = Rules::new(levels);
    return rules.count_safe_reports();
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let result = part1("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");
        assert_eq!(result, 2);
    }
}