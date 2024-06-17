// Numbers of employees who met the target

pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
    let mut count = 0;
    hours.iter().for_each(|x| {
        if *x >= target {
            count += 1
        }
    });

    count
}

fn main() {
    println!(number_of_employees_who_met_target(vec![0, 1, 2, 3, 4], 5));
}
