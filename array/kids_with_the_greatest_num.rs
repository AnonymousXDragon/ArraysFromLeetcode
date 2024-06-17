// Kids With The Greatest Number of Candies

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::with_capacity(candies.len());
    for candle in 0..candies.len() {
        if candies[candle] + extra_candies >= *candies.iter().max().unwrap() {
            result.push(true)
        } else {
            result.push(false)
        }
    }
    result
}

fn main() {
    println!(kids_with_candies(vec![2, 3, 5, 1, 3]), 3);
}
