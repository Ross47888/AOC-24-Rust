use std::iter::zip;

fn main() {
    let inpf = include_str!("../../inputs/day01.txt");
    let (mut v0, mut v2): (Vec<i32>, Vec<i32>) =
        (Vec::with_capacity(1000), Vec::with_capacity(1000));
    inpf.lines().for_each(|line| {
        let mut t = line.split_ascii_whitespace();
        v0.push(t.next().unwrap().parse::<i32>().unwrap());
        v2.push(t.next().unwrap().parse::<i32>().unwrap());
    });
    v0.sort();
    v2.sort();
    println!("{}", part_one(&v0, &v2));
    println!("{}", part_two(&v0, &v2));
}

fn part_one(v0: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    zip(v0, v2).fold(0, |sum, x| sum + (x.0 - x.1).abs())
}

fn part_two(v0: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    v0.iter()
        .map(|x| x * (v2.iter().filter(|y| *y == x).count() as i32))
        .sum()
}


fn part_two_efficiant(v0: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // Want to traverse along v0 while it is same number, 
    // count along v2 same number
    // then continue along v0
    for i in 0..v0.len() {
    }
    0
}
