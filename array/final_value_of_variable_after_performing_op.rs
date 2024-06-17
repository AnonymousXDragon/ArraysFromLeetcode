pub fn final_value_after_operations(operations: Vec<&str>) -> i32 {
    let mut x = 0;
    operations.iter().for_each(|op| {
        if op.contains("--") {
            x -= 1;
        } else {
            x += 1;
        }
    });
    x
}

fn main() {
    let operations = vec!["++X", "++X", "X++"];
    println!("{}", final_value_after_operations(operations));
}
