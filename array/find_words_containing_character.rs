pub fn find_words_containing(words: Vec<&str>, x: char) -> Vec<i32> {
    
    let mut res:Vec<i32> = Vec::with_capacity(words.len());
    words.iter()
        .enumerate()
        .for_each(|(index,word)| {
            if word.contains(x) {
                res.push(index as i32)     
            }
        });
    res
}

fn main(){
    let (words,x) = (vec!["leet","code"] ,  'e');
    println!("{:?}",find_words_containing(words,x))
}