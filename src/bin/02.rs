fn main() {
    let report = include_str!("../../inputs/02ex.txt")
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    println!("Part One: {}", part_one(&report));
    println!("Part Two: {}", part_two(&report));
}

fn part_one(report: &Vec<Vec<i32>>) -> usize {
    report
        .iter()
        .filter(|level| check_level(level))
        .count()
}

fn part_two(report: &Vec<Vec<i32>>) -> usize {
    report
        .iter()
        .map(|level| check_one_diff(level))
        .filter(|level| check_level(level))
        .count();
    0
}

fn check_level(level: &Vec<i32>) -> bool {
    level.windows(3).all(|w| {
        (((w[0] - w[1]).abs() < 4) && ((w[1] - w[2]).abs() < 4))
            && (((w[0] - w[1]).abs() > 0) && ((w[1] - w[2]).abs() > 0))
            && ((w[0] > w[1] && w[1] > w[2]) || (w[0] < w[1] && w[1] < w[2]))
    })
}

fn check_one_diff(level: &Vec<i32>) -> Vec<i32> {
    println!("Level {:?}", level);

    if check_level(&level[1..level.len()].to_vec()) {
        return level[1..level.len()].to_vec();
    }
    if check_level(&level[0..level.len()-1].to_vec()) {
        return level[0..level.len()-1].to_vec();
    }

    level.windows(3).for_each(|window| {
        println!("{:?}", window);
    });

    println!();
    level.to_vec()
}
