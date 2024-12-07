fn main() {
    let inpf = include_str!("../../inputs/day04.txt")
        .split_ascii_whitespace()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    println!("Part 1: {}", part_one(&inpf));
    println!("Part 2: {}", part_two(&inpf));
}

fn part_one(inp: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    // Horizontal
    inp.iter().for_each(|line|{for subst in line.windows(4){if compare(&subst.to_vec()) {sum += 1;}}});
    inp.windows(4).for_each(|window| {
        // Vertical
        for i in 0..window[0].len() {if compare(&vec![window[0][i], window[1][i], window[2][i], window[3][i]]) {sum += 1;}}
        // Diagnoal
        let h = window[0].windows(4) .zip(window[1].windows(4)) .zip(window[2].windows(4)) .zip(window[3].windows(4))
            .map(|tup| vec![tup.0.0.0.to_vec(), tup.0.0.1.to_vec(), tup.0.1.to_vec(), tup.1.to_vec()])
            .collect::<Vec<Vec<Vec<char>>>>();
        for four_square in h {
            let bslash = vec![four_square[0][0], four_square[1][1], four_square[2][2], four_square[3][3]];
            let fslash = vec![four_square[0][3], four_square[1][2], four_square[2][1], four_square[3][0]];
            match (compare(&bslash), compare(&fslash)) {
                (true, true) =>  sum += 2,
                (true, false) => sum += 1,
                (false, true) => sum += 1,
                _ => (),}}});
    sum
}

fn part_two(inp: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    for y in 1..inp.len()-1 {
        for x in 1..inp[y].len()-1 {
            if inp[y][x] == 'A' {
                sum += compare2(&inp, y, x);
            }
        }
        println!();
    }
    sum
}

// fn _part_two(inp: &Vec<Vec<char>>) -> u32 {
//     let mut sum = 0;
//     let _ = inp.windows(3).map(|window| {
//         window[0].windows(3).zip(window[1].windows(3)).zip(window[2].windows(3))
//             .map(|tup| vec![tup.0.0.to_vec(), tup.0.1.to_vec(), tup.1.to_vec()])
//             .for_each(|v| {
//                 if compare2(&v) > 0 {
//                     for ve in v.clone() {
//                         println!("{:?}", ve);
//                     }
//                     println!("{:?}", compare2(&v));
//                 }
//                 sum += compare2(&v);
//             });
//     });
//     sum
// }
fn compare2(inp: &Vec<Vec<char>>, y, x) -> u32 {
}

fn _compare2(v: &Vec<Vec<char>>) -> u32 {
    match ((v[0][0], v[1][1], v[2][2]), (v[0][2], v[1][1], v[2][0])) {
        (('S', 'A', 'M'),
         ('S', 'A', 'M')) => 1,

        (('S', 'A', 'S'),
         ('M', 'A', 'M')) => 1,

        (('M', 'A', 'S'),
         ('M', 'A', 'S')) => 1,

        (('M', 'A', 'M'),
         ('S', 'A', 'S')) => 1,

        (('S', 'A', 'M'),
         ('M', 'A', 'S')) => 1,

        (('M', 'A', 'S'),
         ('S', 'A', 'M')) => 1,
        _ => 0,
    }
}

fn compare(char_vec: &Vec<char>) -> bool {
    *char_vec == vec!['X', 'M', 'A', 'S'] || *char_vec == vec!['S', 'A', 'M', 'X']
}
