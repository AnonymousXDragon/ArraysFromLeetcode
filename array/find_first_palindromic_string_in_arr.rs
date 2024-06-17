pub fn check_palindrome(word: &String) -> String {
    let x: Vec<_> = word.chars().rev().collect();
    let res: String = x.iter().collect();
    return res;
}

pub fn first_palindrome(words: Vec<String>) -> String {
    let mut result: Vec<String> = Vec::new();
    words.iter().for_each(|word| {
        if *word == check_palindrome(&word) {
            result.push(word.to_string())
        };
    });

    if result.len() <= 0 {
        return "".to_string();
    } else {
        let res = &result[0];
        return res.to_string();
    }
}

fn main() {
    let words = vec!["def".to_string(), "ghi".to_string()];
    println!("{:?}", first_palindrome(words))
}
