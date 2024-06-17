pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut x: i32 = 0;
    let length = nums.len() as i32;
    while x <= length {
        if !nums.contains(&x) {
            break;
        }
        x += 1;
    }
    x
}

fn main() {
    println!(missing_number(vec![3, 0, 1]))
}
