use std::collections::HashSet;

fn main() {
    let start = std::time::Instant::now();
    let inp = include_str!("../../inputs/day08.txt");
    let y_len = inp.lines().count() as i64;
    let x_len = inp.lines().next().unwrap().len() as i64;
    let roof = inp
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch != '.')
                .map(move |(x, ch)| (y as i64, x as i64, ch))
        })
        .collect::<Vec<(i64, i64, char)>>();
    part_one(&roof, x_len, y_len);
    part_two(&roof, x_len, y_len);
    println!("Time: {:.4?}s", start.elapsed().as_secs_f64());
}

fn part_one(roof: &Vec<(i64, i64, char)>, x_len: i64, y_len: i64) {
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    for i in 0..roof.len() {
        for j in i + 1..roof.len() {
            if roof[i].2 == roof[j].2 {
                let (p0, p3) = find_valid_antinodes(roof[i], roof[j], x_len, y_len);
                if p0 != None {
                    visited.insert(p0.unwrap());
                }
                if p3 != None {
                    visited.insert(p3.unwrap());
                }
            }
        }
    }
    println!("Part One: {}", visited.len());
}

fn part_two(roof: &Vec<(i64, i64, char)>, x_len: i64, y_len: i64) {
    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();
    for i in 0..roof.len() {
        for j in i+1..roof.len() {
            if roof[i].2 == roof[j].2 {
                    antinode_lines((roof[i].0, roof[i].1), (roof[j].0, roof[j].1), (y_len, x_len), &mut antinodes);
            }
        }
    }
    println!("Part Two: {}", antinodes.len());
}

fn antinode_lines((y1, x1): (i64, i64), (y2, x2): (i64, i64), (y_len, x_len): (i64, i64), antinodes: &mut HashSet<(i64, i64)>){
    let (dy, dx) = (y2 - y1, x2 - x1);
    let (mut cy, mut cx) = (y2, x2);
    antinodes.insert((cy, cx));
    while 0 <= cy && cy < y_len && 0 <= cx && cx < x_len {
        antinodes.insert((cy, cx));
        cy += dy;
        cx += dx;
    }
    let (mut cy, mut cx) = (y2 - dy, x2 - dx);
    while 0 <= cy && cy < y_len && 0 <= cx && cx < x_len {
        antinodes.insert((cy, cx));
        cy -= dy;
        cx -= dx;
    }
}


fn find_valid_antinodes(
    point_1: (i64, i64, char),
    point_2: (i64, i64, char),
    x_len: i64,
    y_len: i64,
) -> (Option<(i64, i64)>, Option<(i64, i64)>) {
    let diff: (i64, i64) = ((point_1.0 - point_2.0), (point_1.1 - point_2.1));
    let point_0 = ((point_1.0 + diff.0), (point_1.1 + diff.1));
    let point_3 = ((point_2.0 - diff.0), (point_2.1 - diff.1));
    (
        check_point_valid(point_0, x_len, y_len),
        check_point_valid(point_3, x_len, y_len),
    )
}

// fn check_valid((y, x): (i64, i64), x_len: i64, y_len: i64) -> bool {
//     if 0 > x || x >= x_len || 0 > y || y >= y_len {
//         return false;
//     }
//     true
// }

fn check_point_valid((y, x): (i64, i64), x_len: i64, y_len: i64) -> Option<(i64, i64)> {
    if 0 > x || x >= x_len || 0 > y || y >= y_len {
        return None;
    }
    Some((y, x))
}
