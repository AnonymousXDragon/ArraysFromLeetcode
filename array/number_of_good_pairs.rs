use std::collections::HashMap;

pub fn count_num(num: i32, nums: &Vec<i32>) -> i32 {
    let mut count = 0;
    nums.iter().for_each(|x| {
        if *x == num {
            count += 1
        }
    });
    return count;
}

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut results: HashMap<i32, i32> = HashMap::new();
    nums.iter().for_each(|x| {
        let mut n = count_num(*x, &nums);

        if n != 0 {
            results.insert(*x, n * (n - 1) / 2);
        }
    });

    let mut total = 0;
    results.iter().for_each(|(_key, val)| total += val);
    return total;
}

fn main() {
    let nums = vec![4, 4, 2, 2];
    println!("res:{:?}", num_identical_pairs(nums))
}
