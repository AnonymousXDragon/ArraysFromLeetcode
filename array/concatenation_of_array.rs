pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    nums.iter().for_each(|x| result.push(*x));
    result.extend(nums);
    result
}

fn main() {
    let nums = vec![1, 2, 1];
    get_concatenation(nums);
}
