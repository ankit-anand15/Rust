fn shortest_word(s: &str) -> &str {
    s.split_whitespace()
        .min_by_key(|word| word.len())
        .unwrap_or("")
}

fn main() {
    let text = "My naame is Ankit anand";
    let shortest = shortest_word(&text);
    println!("The shortest word is: {}", shortest);
}
