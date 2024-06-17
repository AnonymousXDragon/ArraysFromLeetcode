pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let f = &nums[..n];
    let l = &nums[n..];
    let mut res: Vec<i32> = Vec::with_capacity(nums.len());

    f.iter().zip(l).for_each(|x| {
        res.push(*x.0);
        res.push(*x.1);
    });
    res
}

fn main() {
    let arr = vec![2, 5, 1, 3, 4, 7];
    let n = 3;
    println!("{:?}", shuffle(arr, n))
}
