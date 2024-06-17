fn third_max(v: &mut Vec<i32>) -> i32 {
    if v.len() < 3 {
        return v[v.len() - 1];
    }
    v.sort_by(|a, b| b.partial_cmp(a).unwrap());
    v.dedup();

    if v.len() < 3 {
        return v[0];
    }

    if v.len() > 3 {
        return v[3 - 1];
    }
    return v[v.len() - 1];
}
fn main() {
    let mut v = vec![1, 2];
    println!("{}", third_max(&mut v));
    println!("{:?}", v)
}
