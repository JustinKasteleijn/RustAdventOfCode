fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
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

    fn count_safe_reports(&self) -> u32 {
        let mut safe_reports: u32 = 0;

        for level in &self.levels {
            if Self::all_increasing_or_decreasing(level) && Self::is_safe(level) {
                safe_reports += 1;
                continue;
            }

            for index in 0..level.len() {
                let clone = {
                    let mut temp = (*level).clone();
                    temp.remove(index);
                    temp
                };

                if Self::all_increasing_or_decreasing(&clone) && Self::is_safe(&clone) {
                    safe_reports += 1;
                    break;
                }
            }
        }

        safe_reports
    }

    fn is_safe(level: &Vec<u32>) -> bool {
         return level
             .windows(2)
             .all(|pair| {
                 let a = pair[0];
                 let b = pair[1];
                 a.abs_diff(b) >= 1 && a.abs_diff(b) <= 3
             })
    }
    fn all_increasing_or_decreasing(level: &Vec<u32>) -> bool {
        let increasing = level
            .windows(2)
            .all(|pair| pair[0] < pair[1]);

        let decreasing = level
            .windows(2)
            .all(|pair| pair[0] > pair[1]);

        return increasing ^ decreasing;
    }
}

fn part2(input: &str) -> u32 {
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
    use crate::part2;

    #[test]
    fn part2_test() {
        let result = part2("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");
        assert_eq!(result, 4);
    }
}