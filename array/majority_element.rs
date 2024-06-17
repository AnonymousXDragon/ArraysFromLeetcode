pub fn majority_element(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    use std::iter::FromIterator;

    let majority = (nums.len() / 2) as i32;
    let mut hmap: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

    nums.iter().for_each(|x| {
        hmap.insert(*x, nums.iter().filter(|y| *y == x).count() as i32);
    });

    let mut res = 0;
    println!("{:?}", hmap);
    hmap.iter()
        .inspect(|x| println!("{:?}", x))
        .for_each(|tup| {
            if *tup.1 > majority {
                res = *tup.0;
            }
        });
    return res;
}

fn main() {
    let nums = vec![3, 2, 3];
    println!("{}", majority_element(nums));
}
