use std::collections::VecDeque;

fn main() {
    let start = std::time::Instant::now();
    let inp = include_str!("../../inputs/day07.txt")
        .lines()
        .map(|l| {
            let mut spl = l.split(":");
            (
                spl.next().unwrap().parse::<u64>().unwrap(),
                spl.next()
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<VecDeque<u64>>(),
            )
        })
        .collect::<Vec<(u64, VecDeque<u64>)>>();
    part_one(&inp);
    part_two(&inp);
    println!("Total Time: {:.6?}s", start.elapsed().as_secs_f64());
}

fn part_one(inp: &Vec<(u64, VecDeque<u64>)>) {
    println!("Part One: {}", inp
        .iter()
        .filter_map(|eq| if one_solve(eq.0, 0, 0, &eq.1) { Some(eq.0) } else {None} )
        .sum::<u64>());
}

fn part_two(inp: &Vec<(u64, VecDeque<u64>)>) {
    println!("Part Two: {}", inp
        .iter()
        .filter_map(|eq| if two_solve(eq.0, 0, 0, &eq.1) { Some(eq.0) } else {None} )
        .sum::<u64>());
}


fn one_solve(tar: u64, tot: u64, ind: usize, nums: &VecDeque<u64>) -> bool {
    if ind == nums.len() {
        tar == tot
    }
    else {
        if ind == 0{
        return one_solve(tar, tot+nums[ind], ind+1, nums) || one_solve(tar, 1*nums[ind], ind+1, nums);
        }
        one_solve(tar, tot+nums[ind], ind+1, nums) || one_solve(tar, tot*nums[ind], ind+1, nums)
    }
}

fn two_solve(tar: u64, tot: u64, ind: usize, nums: &VecDeque<u64>) -> bool {
    if ind == nums.len() {
        tar == tot
    }
    else {
        if ind == 0{
        return two_solve(tar, tot+nums[ind], ind+1, nums) || two_solve(tar, 1*nums[ind], ind+1, nums) || two_solve(tar, conc_u64(tot, nums[ind]), ind+1, nums);
        }
        match (conc_u64(tot, nums[ind]) > tar, tot*nums[ind] > tar) {
            (true, true) => two_solve(tar, tot+nums[ind], ind+1, nums),
            (false, true) => two_solve(tar, tot+nums[ind], ind+1, nums) || two_solve(tar, conc_u64(tot, nums[ind]), ind+1, nums),
            (true, false) => two_solve(tar, tot+nums[ind], ind+1, nums) || two_solve(tar, tot*nums[ind], ind+1, nums),
            (false, false) => two_solve(tar, tot+nums[ind], ind+1, nums) || two_solve(tar, tot*nums[ind], ind+1, nums) || two_solve(tar, conc_u64(tot, nums[ind]), ind+1, nums),
        }
    }
}

fn conc_u64(num_1: u64, num_2: u64) -> u64 {
    let (mut one, mut two) = (num_1, num_2);
    while two != 0 {
        one *= 10;
        two /= 10;
    }
    one + num_2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conc_test() {
        println!("{}", conc_u64(10, 1));
        println!("{}", conc_u64(6, 7896));
        println!("{}", conc_u64(2343, 5));
        println!("{}", conc_u64(233, 435));
        println!("{}", conc_u64(623, 7896));
        assert_eq!(101, conc_u64(10, 1));
        assert_eq!(67896, conc_u64(6, 7896));
        assert_eq!(23435, conc_u64(2343, 5));
        assert_eq!(233435, conc_u64(233, 435));
        assert_eq!(6237896, conc_u64(623, 7896));
    }
}
