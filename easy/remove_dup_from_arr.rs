pub fn binary_search(nums: &mut Vec<i32>, mut low: i32, mut high: i32, mut val: i32) -> i32 {
    let mut res: i32 = -1;
    while low <= high {
        let as_mid: f64 = (low + (high - low) / 2) as f64;
        let mut mid = as_mid.floor() as i32;
        if nums[mid as usize] <= val {
            low = mid + 1;
        } else {
            res = mid;
            high = mid - 1;
        }
    }
    if res == -1 {
        return nums.len() as i32;
    }
    return res;
}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx: i32 = 0;
        let n = nums.len() as i32;
        while idx != n {
            let mut i = binary_search(nums, idx + 1, n - 1, nums[idx as usize] as i32);
            if i == n {
                return idx + 1;
            }
            idx += 1;
            nums[idx as usize] = nums[i as usize]
        }
        return idx;
    }
}
