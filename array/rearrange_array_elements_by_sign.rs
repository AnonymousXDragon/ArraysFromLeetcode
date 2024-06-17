fn rearrange_array(v: Vec<i32>) -> Vec<i32> {
    let mut plus: Vec<i32> = Vec::new();
    let mut negative: Vec<i32> = Vec::new();
    let mut result: Vec<i32> = Vec::new();

    v.iter().for_each(|num| {
        if *num < 0 {
            negative.push(*num)
        } else {
            plus.push(*num)
        }
    });

    plus.iter().zip(&negative).for_each(|x| {
        result.push(*x.0);
        result.push(*x.1)
    });
    return result;
}

fn main() {
    let v = vec![3, 1, -2, -5, 2, -4];
    rearrange_array(v);
}
