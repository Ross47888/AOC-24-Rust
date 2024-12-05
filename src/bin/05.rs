#![allow(unused)]
use std::collections::{HashMap, VecDeque};

fn main() {
    let inpf = include_str!("../../inputs/day05.txt").split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    inpf[0].lines().for_each(|rule| {
        let mut h = rule.split("|");
        let (one, two) = (h.next().unwrap().parse::<u32>().unwrap(), h.next().unwrap().parse::<u32>().unwrap(),);
        rules.entry(one).or_default().push(two);
    });
    let pages = inpf[1].lines()
        .map(|line| {line.split(",").filter_map(|i| i.parse::<u32>().ok()).collect::<Vec<u32>>()})
        .collect::<Vec<Vec<u32>>>();
    println!("Part One: {}", part_one(&rules, &pages));
    part_two(&rules, &pages);
}

fn part_two(rules: &HashMap<u32, Vec<u32>>, pages: &Vec<Vec<u32>>) {
    let mut sum = 0;
    for page in pages {sum += sort_page(&page, &rules);}
    sum -= part_one(&rules, &pages);
    println!("Part Two: {sum}");
}

fn part_one(rules: &HashMap<u32, Vec<u32>>, pages: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for page in pages {
        let mut check = true;
        for i in (0..page.len()).rev() {
            if let Some(highers) = rules.get(&page[i]) {for val in highers {if page[0..i].contains(val) {check = false;}}}
        }
        if check {sum += page[page.len()/2];}
    }
    sum
}

fn sort_page(page: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> u32 {
    let (mut sum, mut rule_count): (u32, HashMap<u32, usize>) = (0, HashMap::new());
    for num in page {
        if rules.get(num).is_some() {for num_2 in page {if rules.get(num).unwrap().contains(num_2) {*rule_count.entry(*num).or_insert(0) += 1;}}}
    }
    let mut rule_count_vec = rule_count.iter().map(|(&k, &v)| (k,v)).collect::<Vec<(u32, usize)>>();
    rule_count_vec.sort_by_key(|&(_, v)| v);
    sum += rule_count_vec[(rule_count_vec.len()/2)-1].0;
    sum
}
