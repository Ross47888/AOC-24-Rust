use regex::Regex;

fn main() {
    let inpf = include_str!("../../inputs/day03.txt");
    println!("{}", part_one(&inpf));
    println!("{}", part_two(&inpf));
}

fn part_one(inp: &str) -> i32 {
    let re: Regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(inp)
        .map(|line| line[1].parse::<i32>().unwrap() * line[2].parse::<i32>().unwrap())
        .sum::<i32>()
}

fn part_two(inp: &str) -> i32 {
    let mut combined = Regex::new(r"do\(\)")
        .unwrap()
        .captures_iter(&inp)
        .map(|cap| (true, cap.get(0).unwrap().start()))
        .chain(
            Regex::new(r"don't\(\)")
                .unwrap()
                .captures_iter(inp)
                .map(|cap| (false, cap.get(0).unwrap().start())),
        )
        .chain(vec![(true, 0)])
        .collect::<Vec<(bool, usize)>>();
    combined.sort_by_key(|&(_, x)| x);
    let matches = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")
        .unwrap()
        .captures_iter(inp)
        .map(|line| {
            (
                line.get(0).unwrap().start(),
                line[1].parse::<i32>().unwrap() * line[2].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(usize, i32)>>();
    let mut sum = 0;
    combined.windows(2).for_each(|window| {
        if window[0].0 == true {
            for i in &matches {
                if i.0 > window[0].1 && i.0 < window[1].1 {
                    sum += i.1;
                }
            }
        }
    });
    if combined[combined.len() - 1].0 == true {
        for i in matches {
            if i.0 > combined[combined.len() - 1].1 {
                sum += i.1;
            }
        }
    }
    sum
}
