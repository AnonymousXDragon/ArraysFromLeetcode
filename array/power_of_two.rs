pub fn is_power_of_two(n: i32) -> bool {
    let mut num = n;

    if num == 0 {
        return false;
    } else {
        while num != 1 {
            if num % 2 != 0 {
                return false;
            };
            num = num / 2;
        }
    }
    return true;
}

fn main() {
    println!("{}", is_power_of_two(1));
}
