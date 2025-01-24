use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

type Page = u32;

#[derive(Debug)]
struct PageOrdering {
    ordering: HashMap<Page, Vec<Page>>,
}

impl PageOrdering {
}

#[derive(Debug)]
struct Updates {
    page: Vec<Vec<Page>>,
}

fn parse(input: &str) -> (PageOrdering, Updates) {
    let normalized_input = input.replace("\r\n", "\n");
    let(ord, upd) = normalized_input.split_once("\n\n").unwrap();
    return (parse_ordering(ord), parse_updates(upd));
}

fn parse_ordering(input: &str) -> PageOrdering {
    return input
        .lines()
        .fold(PageOrdering { ordering: HashMap::new() }, |mut acc, line| {
            let (a, b) = line
                .split_once('|')
                .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
                .unwrap();
            acc.ordering
                .entry(b)
                .or_insert_with(Vec::new)
                .push(a);
            return acc;
        });
}

fn parse_updates(input: &str) -> Updates {
    return input
        .lines()
        .fold(Updates { page: vec![] }, |mut acc, line| {
            acc.page.push(
                line.split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>(),
            );
            return acc;
        });
}

fn part1(input: &str) -> u32 {
    let (ordering, updates) = parse(input);

    let mut valid: Vec<Vec<Page>> = Vec::new();

    for update in &updates.page {
        let mut forbidden: HashSet<&Page> = HashSet::new();
        let mut is_valid = true;

        for page in update {
            if forbidden.contains(&page) {
                is_valid = false;
                break;
            }

            if let Some(forbs) = ordering.ordering.get(&page) {
                for forb in forbs {
                    forbidden.insert(forb);
                }
            }
        }

        if is_valid {
            valid.push(update.clone());
        }
    }

    return valid
        .iter()
        .map(|update| update[update.len()/2])
        .sum::<u32>();
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let result = part1("47|53
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
        assert_eq!(result, 143);
    }
}
