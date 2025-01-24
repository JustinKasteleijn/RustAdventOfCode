use std::cmp::Ordering::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

pub fn part2(input: &str) -> u32 {
    let mut result: i32 = 0;

    //Stupid line :(
    let input = input.replace("\r\n", "\n");
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut rules: Vec<Vec<bool>> = vec![vec![false; 100]; 100];
    for line in parts[0].lines(){
        let rule_parts: Vec<usize> = line
            .split('|')
            .map(|part| part.trim().parse::<usize>().expect("Invalid number")) // Parse as usize
            .collect();

        rules[rule_parts[0]][rule_parts[1]] = true;
    }

    for line in parts[1].lines(){
        let mut values: Vec<i32> = line.split(',').map(|val| val.trim().parse().unwrap()).collect();
        if is_in_order(&values, &rules) {
            continue;
        }
        values.sort_by(|&a, &b| custom_compare(a, b, &rules));
        result += values.get(values.len() / 2).unwrap();
    }

    return result as u32;
}

fn is_in_order(values: &Vec<i32>, rules: &Vec<Vec<bool>>) -> bool {
    for pair in values.windows(2) {
        if !rules[pair[0] as usize][pair[1] as usize] {
            return false;
        }
    }
    true
}

fn custom_compare(
    a: i32,
    b: i32,
    rules: &Vec<Vec<bool>>,
) -> std::cmp::Ordering {

    if rules[a as usize][b as usize] {
        std::cmp::Ordering::Less // a should come before b
    } else if rules[b as usize][a as usize] {
        std::cmp::Ordering::Greater // b should come before a
    } else {
        std::cmp::Ordering::Equal // a and b are equal according to the rules
    }
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn part2_test() {
        let result = part2("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47");
        assert_eq!(result, 123);
    }
}
