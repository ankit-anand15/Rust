fn main() {
    let x = is_prime(25); //function calling
    println!("{}", x);
}

fn is_prime(n: u32) -> bool {
    let mut result: bool = for a in 2..n {
        result = if n % a == 0 { false } else { true };
    }
    result
}