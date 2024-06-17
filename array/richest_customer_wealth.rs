pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut result: Vec<i32> = Vec::with_capacity(accounts.len());
    accounts
        .iter()
        .for_each(|account| result.push(account.iter().sum::<i32>()));
    *result.iter().max().unwrap()
}

fn main() {
    let accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];
    println!("{:?}", maximum_wealth(accounts))
}
