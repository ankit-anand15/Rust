pub fn reverse(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars().rev() {
        result.push(c)
    }

    result
}
fn main() {
    let original = "ANKIT ANAND";
    let reversed = reverse(original);
    println!("Reversed: {}", reversed);
}