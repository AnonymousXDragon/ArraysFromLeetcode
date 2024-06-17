// widest vertical area between two points contaning no points

pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
    use std::cmp::max;

    let mut h: Vec<i32> = Vec::with_capacity(points.len());

    points.iter().for_each(|x| {
        h.push(x[0]);
    });
    h.sort();

    let mut wide = 0;
    for x in 1..h.len() {
        wide = max(wide, i32::abs((h[x - 1] - h[x])));
    }
    wide
}

fn main() {
    let points = vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]];
    println!("{:?}", max_width_of_vertical_area(points))
}
