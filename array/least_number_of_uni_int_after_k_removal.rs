// Least Number Of Unique Integers After K Removals

pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;
    use std::iter::FromIterator;

    let mut hmap: HashMap<i32, i32> = HashMap::with_capacity(arr.len());

    arr.iter().for_each(|x| {
        hmap.insert(*x, arr.iter().filter(|num| *num == x).count() as i32);
    });

    let mut m = Vec::from_iter(hmap);
    m.sort_by(|(_, a), (_, b)| a.cmp(b));
    let mut y = k;
    for x in 0..m.len() {
        y -= m[x].1;
        if y < 0 {
            return (m.len() - x) as i32;
        }
    }
    return 0;
}

fn main() {
    let arr = vec![2, 1, 1, 3, 3, 3];
    //let arr = vec![4,3,1,1,3,3,2];
    //let arr = vec![5,5,4];
    let arr = vec![2, 1, 1, 3, 3, 3];
    // let arr = vec![5,5,4];
    let arr = vec![4, 3, 1, 1, 3, 3, 2];
    let k = 3;
    println!("{}", find_least_num_of_unique_ints(arr, k));
}
