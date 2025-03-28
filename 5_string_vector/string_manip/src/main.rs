use std::collections::HashMap;

fn longest_word(s: &str) -> &str {
    let mut max_length = 0;
    let mut max_word = "";
    for word in s.split_whitespace() {
        if word.len() > max_length {
            max_length = word.len();
            max_word = word;
        }
    }
    max_word
}

fn main() {
    let s = "the blade itself incites to deeds of violence";
    println!("{}", &s[0..4]);

    let b = "Title: ".to_string() + &s;
    println!("{}", b);

    let d = format!("Title: {}", s);
    println!("{}", d);

    let mut m = HashMap::from([('a', 0), ('e', 0), ('i', 0), ('o', 0), ('u', 0)]);
    for c in s.chars() {
        // Count the number of occurences of each vowel
        match c{
            'a' | 'e' | 'i' | 'o' | 'u' => {
                m.insert(c, m[&c] + 1);
            },
            _ => {}
        }
    }
    for (k, v) in m {
        println!("{}: {}", k, v);
    }
    let word = longest_word(s);
    println!("{}", word);
}
