pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
    let mut count = 0;
    for x in 0..nums.len() {
        for y in x + 1..nums.len() {
            if nums[x] + nums[y] < target {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let (nums, target) = (vec![-1, 1, 2, 3, 1], 2);
    println!("{:?}", count_pairs(nums, target))
}
