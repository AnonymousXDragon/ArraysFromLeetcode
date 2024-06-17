pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<_> = Vec::new();
    nums.iter()
        .enumerate()
        .for_each(|(index, val)| ans.push(nums[*val as usize]));
    return ans;
}

fn main() {
    let nums = vec![5, 0, 1, 2, 3, 4];
    println!("{:?}", build_array(nums));
}
